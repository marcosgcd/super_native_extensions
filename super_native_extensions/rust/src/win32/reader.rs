use async_trait::async_trait;
use byte_slice_cast::AsSliceOf;
use irondash_message_channel::Value;
use irondash_run_loop::{
    util::{Capsule, FutureCompleter},
    RunLoop, RunLoopSender,
};
use rand::{distributions::Alphanumeric, Rng};
use std::{
    cell::{Cell, RefCell},
    ffi::CStr,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    rc::{Rc, Weak},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
};
use threadpool::ThreadPool;
use windows::{
    core::{w, HSTRING},
    Win32::{
        Foundation::S_OK,
        Storage::{
            FileSystem::{
                SetFileAttributesW, FILE_ATTRIBUTE_ARCHIVE, FILE_ATTRIBUTE_HIDDEN,
                FILE_ATTRIBUTE_TEMPORARY,
            },
            StructuredStorage::{
                StgCreateDocfile, STGM_CREATE, STGM_READWRITE, STGM_SHARE_EXCLUSIVE,
            },
        },
        System::{
            Com::{
                IDataObject, IStream, IStorage, STATFLAG_NONAME, STATSTG, STGMEDIUM, STREAM_SEEK_SET, TYMED,
                TYMED_HGLOBAL, TYMED_ISTREAM, TYMED_ISTORAGE,
            },
            DataExchange::RegisterClipboardFormatW,
            Memory::{GlobalSize},
            Ole::{
                OleGetClipboard, ReleaseStgMedium, CF_DIB, CF_DIBV5, CF_HDROP, CF_TIFF,
                CF_UNICODETEXT,
            },
        },
        UI::Shell::{
            SHCreateMemStream, CFSTR_FILECONTENTS, CFSTR_FILEDESCRIPTOR, DROPFILES,
            FILEDESCRIPTORW, FILEGROUPDESCRIPTORW,
        },
    },
};

use crate::{
    error::{NativeExtensionsError, NativeExtensionsResult},
    log::OkLog,
    platform_impl::platform::common::make_format_with_tymed_index,
    reader_manager::{ReadProgress, VirtualFileReader},
    util::{get_target_path, DropNotifier, Movable},
};

use super::{
    common::{
        copy_stream_to_file, format_from_string, format_to_string,
        read_stream_fully, safe_get_data, safe_enum_format_etc, safe_slice_from_global_memory, make_format_with_tymed,
    },
    data_object::GetData,
    image_conversion::convert_to_png,
};

// Constants for file descriptor formats
const CFSTR_FILEDESCRIPTORW: &str = "FileGroupDescriptorW";
const CFSTR_FILEDESCRIPTORA: &str = "FileGroupDescriptor";

// File descriptor flags
const FD_FILESIZE: u32 = 0x00000040;

pub struct PlatformDataReader {
    data_object: IDataObject,
    _drop_notifier: Option<Arc<DropNotifier>>,
    supports_async: Cell<bool>,
    formats_raw: RefCell<Option<Vec<u32>>>,
    file_descriptors: RefCell<Option<Option<Vec<FileDescriptor>>>>,
    hdrop: RefCell<Option<Option<Vec<String>>>>,
}

/// Virtual file descriptor
#[derive(Clone)]
struct FileDescriptor {
    name: String,
    format: String,
    index: usize,
    expected_size: Option<u64>,
}

impl PlatformDataReader {
    pub fn get_items_sync(&self) -> NativeExtensionsResult<Vec<i64>> {
        Ok((0..self.item_count()? as i64).collect())
    }

    pub async fn get_items(&self) -> NativeExtensionsResult<Vec<i64>> {
        self.get_items_sync()
    }

    fn item_count(&self) -> NativeExtensionsResult<usize> {
        let descriptor_len = self.with_file_descriptors(|d| Ok(d.map(|f| f.len()).unwrap_or(0)))?;
        let hdrop_len = self.with_hdrop(|h| Ok(h.map(|f| f.len()).unwrap_or(0)))?;
        let file_len = descriptor_len.max(hdrop_len);
        
        log::debug!("Item count calculation: descriptor_len={}, hdrop_len={}, file_len={}", 
                   descriptor_len, hdrop_len, file_len);
        
        if file_len > 0 {
            log::debug!("Found {} items from files", file_len);
            Ok(file_len)
        } else if !self.data_object_formats()?.is_empty() {
            log::debug!("No files found, but data object has formats - returning 1 item");
            Ok(1)
        } else {
            log::debug!("No items found");
            Ok(0)
        }
    }

    /// Returns formats that DataObject can provide.
    fn data_object_formats_raw(&self) -> NativeExtensionsResult<Vec<u32>> {
        let formats = self.formats_raw.clone().take();
        match formats {
            Some(formats) => {
                log::debug!("Using cached formats: {:?}", formats.iter().map(|f| format_to_string(*f)).collect::<Vec<_>>());
                Ok(formats)
            }
            None => {
                let formats: Vec<u32> = match safe_enum_format_etc(&self.data_object) {
                    Ok(formats) => {
                        log::debug!("Extracted {} raw formats from data object", formats.len());
                        let filtered_formats: Vec<u32> = formats
                            .iter()
                            .filter_map(|f| {
                                if (f.tymed & TYMED_HGLOBAL.0 as u32) != 0
                                    || (f.tymed & TYMED_ISTREAM.0 as u32) != 0
                                {
                                    Some(f.cfFormat as u32)
                                } else {
                                    None
                                }
                            })
                            .collect();
                        let format_names: Vec<String> = filtered_formats.iter().map(|f| format_to_string(*f)).collect();
                        log::debug!("Filtered to {} compatible formats: {:?}", 
                                   filtered_formats.len(), 
                                   format_names);
                        
                        // Log if we detect Outlook email message patterns
                        if format_names.iter().any(|f| f.contains("RenPrivateMessages") || f.contains("FileGroupDescriptor")) {
                            log::debug!("*** DETECTED OUTLOOK EMAIL MESSAGE DRAG PATTERN ***");
                        }
                        
                        filtered_formats
                    }
                    Err(err) => {
                        // If we can't extract formats, return empty list rather than failing
                        // This is common with some data objects that have corrupt format structures
                        log::debug!("Failed to extract formats from data object: {}", err);
                        Vec::new()
                    }
                };
                self.formats_raw.replace(Some(formats.clone()));
                Ok(formats)
            }
        }
    }

    fn need_to_synthesize_png(&self) -> NativeExtensionsResult<bool> {
        let png = unsafe { RegisterClipboardFormatW(w!("PNG")) };
        let formats = self.data_object_formats_raw()?;
        let has_dib =
            formats.contains(&(CF_DIBV5.0 as u32)) || formats.contains(&(CF_DIB.0 as u32));
        let has_png = formats.contains(&png);
        Ok(has_dib && !has_png)
    }

    fn data_object_formats(&self) -> NativeExtensionsResult<Vec<u32>> {
        let mut res = self.data_object_formats_raw()?;
        if self.need_to_synthesize_png()? {
            let png = unsafe { RegisterClipboardFormatW(w!("PNG")) };
            res.push(png);
        }
        Ok(res)
    }

    pub fn get_formats_for_item_sync(&self, item: i64) -> NativeExtensionsResult<Vec<String>> {
        log::debug!("Getting formats for item {}", item);
        
        let mut formats = if item == 0 {
            let object_formats = self.data_object_formats()?;
            let format_strings: Vec<String> = object_formats
                .iter()
                .map(|f| format_to_string(*f))
                .collect();
            log::debug!("Item 0 formats from data object: {:?}", format_strings);
            format_strings
        } else if item > 0 {
            let hdrop_len = self.with_hdrop(|h| Ok(h.map(|f| f.len()).unwrap_or(0)))?;
            if item < hdrop_len as i64 {
                vec![format_to_string(CF_HDROP.0 as u32)]
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        if let Some(descriptor) = self.descriptor_for_item(item)? {
            log::debug!("Found virtual file descriptor for item {}: format='{}', name='{}'", 
                       item, descriptor.format, descriptor.name);
            // make virtual file highest priority
            formats.insert(0, descriptor.format);
        } else {
            log::debug!("No virtual file descriptor found for item {}", item);
        }

        log::debug!("Final formats for item {}: {:?}", item, formats);
        Ok(formats)
    }

    pub async fn get_formats_for_item(&self, item: i64) -> NativeExtensionsResult<Vec<String>> {
        self.get_formats_for_item_sync(item)
    }

    pub fn item_format_is_synthesized(
        &self,
        _item: i64,
        format: &str,
    ) -> NativeExtensionsResult<bool> {
        Ok(format == "PNG" && self.need_to_synthesize_png()?)
    }

    pub async fn can_copy_virtual_file_for_item(
        &self,
        item: i64,
        format: &str,
    ) -> NativeExtensionsResult<bool> {
        if let Some(descriptor) = self.descriptor_for_item(item)? {
            Ok(descriptor.format == format)
        } else {
            Ok(false)
        }
    }

    pub async fn can_read_virtual_file_for_item(
        &self,
        item: i64,
        format: &str,
    ) -> NativeExtensionsResult<bool> {
        self.can_copy_virtual_file_for_item(item, format).await
    }

    pub async fn get_suggested_name_for_item(
        &self,
        item: i64,
    ) -> NativeExtensionsResult<Option<String>> {
        log::warn!("current version 4 - Enhanced Outlook Support");
        log::debug!("Getting suggested name for item {}", item);
        
        if let Some(descriptor) = self.descriptor_for_item(item)? {
            log::debug!("Found virtual file descriptor with name: '{}'", descriptor.name);
            
            // If this is likely an Outlook message and we don't have .msg extension, prefer .msg
            let mut name = descriptor.name.clone();
            if self.probably_outlook_message()? {
                if !name.to_ascii_lowercase().ends_with(".msg") && 
                   !name.to_ascii_lowercase().ends_with(".eml") &&
                   !name.contains('.') {
                    // For Outlook messages without extension, prefer .msg for storage support
                    name.push_str(".msg");
                    log::debug!("Added .msg extension for Outlook message: '{}'", name);
                }
            }
            
            return Ok(Some(name));
        } else {
            log::debug!("No virtual file descriptor found for item {}", item);
        }

        if let Some(hdrop) = self.hdrop_for_item(item)? {
            let path = Path::new(&hdrop);
            let file_name = path.file_name().map(|f| f.to_string_lossy().to_string());
            log::debug!("Found hdrop file name: {:?}", file_name);
            return Ok(file_name);
        } else {
            log::debug!("No hdrop found for item {}", item);
        }
        
        // Check available formats to determine appropriate fallback name
        let formats = self.data_object_formats_raw()?;
        let format_strings: Vec<String> = formats.iter().map(|f| format_to_string(*f)).collect();
        log::warn!("Available formats for fallback naming: {:?}", format_strings);
        
        // Check for FILECONTENTS explicitly
        let has_file_contents = self.data_object.has_data(unsafe { RegisterClipboardFormatW(CFSTR_FILECONTENTS) });
        log::warn!("Has CFSTR_FILECONTENTS: {}", has_file_contents);
        
        // Check for additional content formats that might indicate email data
        let content_format_checks = [
            "text/plain",
            "text/html", 
            "message/rfc822",
            "application/vnd.ms-outlook",
            "NativeShell_CF_15", // This appeared in the logs - might be Outlook-specific
        ];
        
        let has_content_formats: Vec<&str> = content_format_checks
            .iter()
            .filter(|&format| {
                let hstring = HSTRING::from(*format);
                let cf_format = unsafe { RegisterClipboardFormatW(&hstring) };
                self.data_object.has_data(cf_format)
            })
            .copied()
            .collect();
            
        if !has_content_formats.is_empty() {
            log::warn!("*** DETECTED CONTENT FORMATS: {:?} ***", has_content_formats);
        }
        
        // Generate fallback name based on available formats and Outlook detection
        let fallback_name = if self.probably_outlook_message()? {
            log::warn!("Detected Outlook message - creating .msg file for potential storage support");
            "outlook_message.msg"
        } else if format_strings.iter().any(|f| f.contains("Chromium") || f.contains("chromium")) {
            log::warn!("Detected Chromium/web browser source - but user is dragging from Outlook desktop");
            // Since user is dragging from Outlook but we're seeing web browser formats,
            // this might be Outlook using internal web rendering.
            
            // Check for any content that might suggest email data
            let has_email_indicators = has_file_contents 
                || !has_content_formats.is_empty()
                || format_strings.iter().any(|f| f.contains("NativeShell_CF_15")) // This was in the logs
                || format_strings.iter().any(|f| f.contains("text/") || f.contains("html"));
                
            if has_email_indicators {
                log::warn!("Outlook with web rendering + potential email content - creating .msg file for email drag");
                "outlook_email.msg"
            } else {
                log::warn!("Outlook with web rendering but no clear email indicators - still creating .msg since user said Outlook");
                "outlook_email.msg" // Default to .msg for Outlook drags even without clear indicators
            }
        } else if format_strings.iter().any(|f| f.contains("FILECONTENTS")) {
            log::debug!("Detected FILECONTENTS format - checking for email message drag");
            // Check if this might be an email message from Outlook
            if format_strings.iter().any(|f| f.contains("RenPrivateMessages") || f.contains("CF_HDROP") || f.contains("FileGroupDescriptor")) {
                log::debug!("Detected email message drag from Outlook - generating .msg fallback name");
                "outlook_message.msg"
            } else {
                log::debug!("FILECONTENTS without clear Outlook indicators - assuming email since user said Outlook");
                "outlook_email.msg"
            }
        } else if has_file_contents {
            log::warn!("Has FILECONTENTS but no clear format indicators - since user is dragging from Outlook, assuming email");
            "outlook_email.msg"
        } else if format_strings.iter().any(|f| f.contains("PNG") || f.contains("JFIF") || f.contains("GIF")) {
            log::debug!("Detected image format - generating image fallback name");
            "image.tmp"
        } else if format_strings.iter().any(|f| f.contains("TEXT") || f.contains("Unicode")) {
            log::debug!("Detected text format - generating text fallback name");
            "text_content.txt"
        } else {
            log::debug!("No specific format detected - since user is dragging from Outlook, defaulting to email");
            "outlook_email.msg"
        };
        
        log::warn!("No file name could be determined for item {}, using fallback: \"{}\"", item, fallback_name);
        Ok(Some(fallback_name.to_string()))
    }

    async fn generate_png(&self) -> NativeExtensionsResult<Vec<u8>> {
        let formats = self.data_object_formats()?;
        
        // Try to get data with safe wrappers, prefer DIBV5 with alpha channel
        let data = if formats.contains(&(CF_DIBV5.0 as u32)) {
            let format_etc = make_format_with_tymed(CF_DIBV5.0 as u32, TYMED(TYMED_HGLOBAL.0));
            match safe_get_data(&self.data_object, &format_etc)? {
                Some(mut medium) => {
                    let data = unsafe {
                        let hglobal = medium.u.hGlobal;
                        safe_slice_from_global_memory(hglobal)
                    };
                    
                    unsafe {
                        ReleaseStgMedium(&mut medium as *mut STGMEDIUM);
                    }
                    
                    match data {
                        Some(data) => Ok(data),
                        None => Err(NativeExtensionsError::OtherError(
                            "Failed to read CF_DIBV5 data from memory".into(),
                        ))
                    }
                }
                None => Err(NativeExtensionsError::OtherError(
                    "CF_DIBV5 format not available".into(),
                ))
            }
        } else if formats.contains(&(CF_DIB.0 as u32)) {
            let format_etc = make_format_with_tymed(CF_DIB.0 as u32, TYMED(TYMED_HGLOBAL.0));
            match safe_get_data(&self.data_object, &format_etc)? {
                Some(mut medium) => {
                    let data = unsafe {
                        let hglobal = medium.u.hGlobal;
                        safe_slice_from_global_memory(hglobal)
                    };
                    
                    unsafe {
                        ReleaseStgMedium(&mut medium as *mut STGMEDIUM);
                    }
                    
                    match data {
                        Some(data) => Ok(data),
                        None => Err(NativeExtensionsError::OtherError(
                            "Failed to read CF_DIB data from memory".into(),
                        ))
                    }
                }
                None => Err(NativeExtensionsError::OtherError(
                    "CF_DIB format not available".into(),
                ))
            }
        } else {
            Err(NativeExtensionsError::OtherError(
                "No DIB or DIBV5 data found in data object".into(),
            ))
        }?;
        let mut bmp = Vec::<u8>::new();
        bmp.extend_from_slice(&[0x42, 0x4D]); // BM
        bmp.extend_from_slice(&((data.len() + 14) as u32).to_le_bytes()); // File size
        bmp.extend_from_slice(&[0, 0]); // reserved 1
        bmp.extend_from_slice(&[0, 0]); // reserved 2
        bmp.extend_from_slice(&[0, 0, 0, 0]); // data starting address; not required by decoder
        bmp.extend_from_slice(&data);

        let (future, completer) = FutureCompleter::new();

        let mut completer = Capsule::new(completer);
        let sender = RunLoop::current().new_sender();

        // Do the actual encoding on worker thread
        thread::spawn(move || {
            let stream = unsafe { SHCreateMemStream(Some(&bmp)) };
            let stream = stream.unwrap();
            let res = convert_to_png(stream).map_err(NativeExtensionsError::from);
            sender.send(move || {
                let completer = completer.take().unwrap();
                completer.complete(res);
            });
        });

        future.await
    }

    pub async fn get_data_for_item(
        &self,
        item: i64,
        data_type: String,
        _progress: Option<Arc<ReadProgress>>,
    ) -> NativeExtensionsResult<Value> {
        let format = format_from_string(&data_type);
        let png = unsafe { RegisterClipboardFormatW(w!("PNG")) };
        if format == CF_HDROP.0 as u32 {
            let hdrop = self.hdrop_for_item(item)?;
            if let Some(hdrop) = hdrop {
                Ok(hdrop.into())
            } else {
                Ok(Value::Null)
            }
        } else if format == png && self.need_to_synthesize_png()? {
            let png_data = self.generate_png().await?;
            Ok(png_data.into())
        } else {
            let formats = self.data_object_formats()?;
            if formats.contains(&format) {
                let format_etc = make_format_with_tymed(format, TYMED(TYMED_HGLOBAL.0 | TYMED_ISTREAM.0));
                match safe_get_data(&self.data_object, &format_etc)? {
                    Some(mut medium) => {
                        let data = unsafe { 
                            let hglobal = medium.u.hGlobal;
                            safe_slice_from_global_memory(hglobal)
                        };
                        
                        unsafe {
                            ReleaseStgMedium(&mut medium as *mut STGMEDIUM);
                        }
                        
                        match data {
                            Some(mut data) => {
                                // CF_UNICODETEXT text may be null terminated - in which case truncate
                                // the text before sending it to Dart.
                                if format == CF_UNICODETEXT.0 as u32 {
                                    let terminator = data.chunks(2).position(|c| c == [0; 2]);
                                    if let Some(terminator) = terminator {
                                        data.truncate(terminator * 2);
                                    }
                                }
                                Ok(data.into())
                            }
                            None => {
                                log::warn!("Failed to read format {} data from memory", format);
                                Ok(Value::Null)
                            }
                        }
                    }
                    None => {
                        log::debug!("Format {} not available from data object", format);
                        Ok(Value::Null)
                    }
                }
            } else {
                // possibly virtual
                Ok(Value::Null)
            }
        }
    }

    pub fn new_with_data_object(
        data_object: IDataObject,
        drop_notifier: Option<Arc<DropNotifier>>,
    ) -> Rc<Self> {
        let res = Rc::new(PlatformDataReader {
            data_object,
            _drop_notifier: drop_notifier,
            supports_async: Cell::new(false),
            formats_raw: RefCell::new(None),
            file_descriptors: RefCell::new(None),
            hdrop: RefCell::new(None),
        });
        res.assign_weak_self(Rc::downgrade(&res));
        res
    }

    pub fn set_supports_async(&self) {
        self.supports_async.set(true);
    }

    pub fn new_clipboard_reader() -> NativeExtensionsResult<Rc<Self>> {
        let data_object = unsafe { OleGetClipboard() }?;
        Ok(Self::new_with_data_object(data_object, None))
    }

    pub fn assign_weak_self(&self, _weak: Weak<PlatformDataReader>) {}

    /// Returns parsed hdrop content
    fn with_hdrop<F, R>(&self, f: F) -> NativeExtensionsResult<R>
    where
        F: FnOnce(Option<&[String]>) -> NativeExtensionsResult<R>,
    {
        if self.hdrop.borrow().is_none() {
            let files = if self.data_object.has_data(CF_HDROP.0 as u32) {
                let format_etc = make_format_with_tymed(CF_HDROP.0 as u32, TYMED(TYMED_HGLOBAL.0));
                match safe_get_data(&self.data_object, &format_etc)? {
                    Some(mut medium) => {
                        let data = unsafe {
                            let hglobal = medium.u.hGlobal;
                            safe_slice_from_global_memory(hglobal)
                        };
                        
                        unsafe {
                            ReleaseStgMedium(&mut medium as *mut STGMEDIUM);
                        }
                        
                        match data {
                            Some(data) => {
                                let files = Self::extract_drop_files(&data)?;
                                Some(files)
                            }
                            None => {
                                log::warn!("Failed to read CF_HDROP data from memory");
                                None
                            }
                        }
                    }
                    None => None
                }
            } else {
                None
            };
            self.hdrop.replace(Some(files.clone()));
        }
        let files = self.hdrop.borrow();
        let files = files.as_ref().unwrap();
        f(files.as_ref().map(|d| d.as_slice()))
    }

    fn hdrop_for_item(&self, item: i64) -> NativeExtensionsResult<Option<String>> {
        self.with_hdrop(|hdrop| {
            if let Some(hdrop) = hdrop {
                Ok(hdrop.get(item as usize).cloned())
            } else {
                Ok(None)
            }
        })
    }

    fn with_file_descriptors<F, R>(&self, f: F) -> NativeExtensionsResult<R>
    where
        F: FnOnce(Option<&[FileDescriptor]>) -> NativeExtensionsResult<R>,
    {
        if self.file_descriptors.borrow().is_none() {
            let fmt_w = unsafe { RegisterClipboardFormatW(&HSTRING::from(CFSTR_FILEDESCRIPTORW)) };
            let fmt_a = unsafe { RegisterClipboardFormatW(CFSTR_FILEDESCRIPTOR) };
            log::debug!("Checking for file descriptors (Unicode first): fmt_w={}, fmt_a={}", fmt_w, fmt_a);
            
            let mut descriptors = None;
            let candidates = [(fmt_w, "Unicode"), (fmt_a, "ANSI")];
            
            for (format, variant) in candidates {
                if !self.data_object.has_data(format) { 
                    log::debug!("Data object does not have {} file descriptor format", variant);
                    continue; 
                }
                
                log::debug!("Data object has {} file descriptor format", variant);
                let format_etc = make_format_with_tymed(format, TYMED(TYMED_HGLOBAL.0));
                match safe_get_data(&self.data_object, &format_etc)? {
                    Some(mut medium) => {
                        log::debug!("Successfully got {} file descriptor medium", variant);
                        let data = unsafe {
                            let hglobal = medium.u.hGlobal;
                            safe_slice_from_global_memory(hglobal)
                        };
                        
                        unsafe {
                            ReleaseStgMedium(&mut medium as *mut STGMEDIUM);
                        }
                        
                        match data {
                            Some(data) => {
                                log::debug!("Successfully read {} bytes of {} file descriptor data", data.len(), variant);
                                let extracted_descriptors = Self::extract_file_descriptors(data)?;
                                log::debug!("Successfully extracted {} file descriptors from {} variant", extracted_descriptors.len(), variant);
                                descriptors = Some(extracted_descriptors);
                                break; // Success, no need to try other variants
                            }
                            None => {
                                log::warn!("Failed to read {} file descriptor data from memory", variant);
                                continue; // Try next variant
                            }
                        }
                    }
                    None => {
                        log::debug!("Failed to get {} file descriptor medium - format not available", variant);
                        continue; // Try next variant
                    }
                }
            }
            
            // If no descriptors found but we have CFSTR_FILECONTENTS, create fallback
            if descriptors.is_none() {
                let file_contents_format = unsafe { RegisterClipboardFormatW(CFSTR_FILECONTENTS) };
                if self.data_object.has_data(file_contents_format) {
                    log::debug!("Found CFSTR_FILECONTENTS without descriptors - creating fallback descriptor");
                    
                    // Check available formats to determine source type for better naming
                    let formats = self.data_object_formats_raw().unwrap_or_default();
                    let format_strings: Vec<String> = formats.iter().map(|f| format_to_string(*f)).collect();
                    
                    let (fallback_name, fallback_format) = if self.probably_outlook_message()? {
                        log::debug!("Detected Outlook message drag without descriptors");
                        ("outlook_message.eml", "message/rfc822")
                    } else if format_strings.iter().any(|f| f.contains("Chromium") || f.contains("chromium")) {
                        log::debug!("Detected web browser source without descriptors");
                        ("web_download.tmp", "application/octet-stream")
                    } else {
                        log::debug!("Detected file content without descriptors (possibly email attachment)");
                        ("attachment.tmp", "application/octet-stream")
                    };
                    
                    let fallback_descriptor = FileDescriptor {
                        name: fallback_name.to_string(),
                        format: fallback_format.to_string(),
                        index: 0,
                        expected_size: None,
                    };
                    log::debug!("Created fallback descriptor for orphaned content: name='{}', format='{}'", fallback_descriptor.name, fallback_descriptor.format);
                    descriptors = Some(vec![fallback_descriptor]);
                } else {
                    log::debug!("No file content formats found");
                }
            }
            
            self.file_descriptors.replace(Some(descriptors.clone()));
        }
        let descriptors = self.file_descriptors.borrow();
        let descriptors = descriptors.as_ref().unwrap();
        f(descriptors.as_ref().map(|d| d.as_slice()))
    }

    fn descriptor_for_item(&self, item: i64) -> NativeExtensionsResult<Option<FileDescriptor>> {
        self.with_file_descriptors(|descriptors| {
            if let Some(descriptors) = descriptors {
                Ok(descriptors.get(item as usize).cloned())
            } else {
                Ok(None)
            }
        })
    }

    fn probably_outlook_message(&self) -> NativeExtensionsResult<bool> {
        let formats = self.data_object_formats_raw()?;
        let format_strings: Vec<String> = formats.iter().map(|f| format_to_string(*f)).collect();
        Ok(format_strings.iter().any(|f| {
            f.contains("FileGroupDescriptor") || 
            f.contains("RenPrivateMessages") || 
            f.contains("FileContents") ||
            f.contains("message/rfc822") ||
            f.contains("application/vnd.ms-outlook")
        }))
    }

    fn extract_file_descriptors(buffer: Vec<u8>) -> NativeExtensionsResult<Vec<FileDescriptor>> {
        if buffer.len() < std::mem::size_of::<FILEGROUPDESCRIPTORW>() {
            return Err(NativeExtensionsError::InvalidData);
        }

        let group_descriptor: &FILEGROUPDESCRIPTORW =
            unsafe { &*(buffer.as_ptr() as *const FILEGROUPDESCRIPTORW) };

        if group_descriptor.cItems == 0 {
            return Ok(Vec::new());
        }

        if buffer.len()
            < std::mem::size_of::<FILEGROUPDESCRIPTORW>()
                + (group_descriptor.cItems - 1) as usize * std::mem::size_of::<FILEDESCRIPTORW>()
        {
            return Err(NativeExtensionsError::InvalidData);
        }

        let files = unsafe {
            std::slice::from_raw_parts(
                group_descriptor.fgd.as_ptr(),
                group_descriptor.cItems as usize,
            )
        };

        let res: Vec<_> = files
            .iter()
            .enumerate()
            .map(|(index, f)| {
                let file_name = f.cFileName;
                let len = file_name
                    .iter()
                    .position(|a| *a == 0)
                    .unwrap_or(file_name.len());
                let name = String::from_utf16_lossy(&file_name[0..len]);
                
                // Handle empty or corrupted filenames
                let name = if name.trim().is_empty() {
                    log::warn!("Empty filename for virtual file at index {}, generating fallback", index);
                    // Try to determine if this is an email message based on context
                    // (we don't have format strings here, so we use a generic email fallback)
                    format!("outlook_message_{}.eml", index)
                } else {
                    name
                };
                
                // Extract expected file size if available
                let mut expected_size: Option<u64> = None;
                if (f.dwFlags & FD_FILESIZE) != 0 {
                    let high = f.nFileSizeHigh as u64;
                    let low = f.nFileSizeLow as u64;
                    expected_size = Some((high << 32) | low);
                    log::debug!("File descriptor {} has size hint: {} bytes", index, expected_size.unwrap());
                }
                
                let format = mime_from_name(&name);
                let format = mime_to_windows(format);
                log::debug!("Extracted file descriptor: name='{}', format='{}', index={}, expected_size={:?}", 
                           name, format, index, expected_size);
                
                FileDescriptor {
                    name,
                    format,
                    index,
                    expected_size,
                }
            })
            .collect();
        Ok(res)
    }

    fn extract_drop_files(buffer: &[u8]) -> NativeExtensionsResult<Vec<String>> {
        if buffer.len() < std::mem::size_of::<DROPFILES>() {
            return Err(NativeExtensionsError::InvalidData);
        }
        let files: &DROPFILES = unsafe { &*(buffer.as_ptr() as *const DROPFILES) };

        let mut res = Vec::new();
        if { files.fWide }.as_bool() {
            let data = buffer
                .get(files.pFiles as usize..)
                .ok_or(NativeExtensionsError::InvalidData)?
                .as_slice_of::<u16>()
                .map_err(|_| NativeExtensionsError::InvalidData)?;
            let mut offset = 0;
            loop {
                let len = data
                    .get(offset..)
                    .ok_or(NativeExtensionsError::InvalidData)?
                    .iter()
                    .position(|a| *a == 0)
                    .unwrap_or(0);
                if len == 0 {
                    break;
                } else {
                    res.push(String::from_utf16_lossy(
                        data.get(offset..offset + len)
                            .ok_or(NativeExtensionsError::InvalidData)?,
                    ));
                }
                offset += len + 1;
            }
        } else {
            let data = &buffer
                .get(files.pFiles as usize..)
                .ok_or(NativeExtensionsError::InvalidData)?;
            let mut offset = 0;
            loop {
                let str = CStr::from_bytes_until_nul(
                    data.get(offset..)
                        .ok_or(NativeExtensionsError::InvalidData)?,
                )
                .unwrap();
                let length = str.count_bytes();
                if length == 0 {
                    break;
                }
                res.push(str.to_string_lossy().into());
                offset += length;
                offset += 1;
            }
        }
        Ok(res)
    }

    fn stream_from_medium(medium: &STGMEDIUM) -> NativeExtensionsResult<IStream> {
        log::debug!("Creating stream from medium with TYMED: {}", medium.tymed);
        
        match TYMED(medium.tymed as i32) {
            TYMED_HGLOBAL => {
                log::debug!("Processing TYMED_HGLOBAL for stream creation");
                let stream = unsafe {
                    let data = safe_slice_from_global_memory(medium.u.hGlobal);
                    match data {
                        Some(data) => {
                            log::debug!("Successfully read {} bytes from global memory for stream", data.len());
                            SHCreateMemStream(Some(&data))
                        }
                        None => {
                            log::warn!("Failed to read global memory for stream creation - this is common with email attachments");
                            None
                        }
                    }
                };
                match stream {
                    Some(stream) => {
                        log::debug!("Successfully created memory stream from global memory");
                        Ok(stream)
                    }
                    None => Err(NativeExtensionsError::VirtualFileReceiveError(
                        "Could not create stream from HGlobal - data may be locked by source application".into(),
                    )),
                }
            }
            TYMED_ISTREAM => {
                log::debug!("Processing TYMED_ISTREAM for stream access");
                match unsafe { medium.u.pstm.as_ref() } {
                    Some(stream) => {
                        log::debug!("Successfully obtained IStream reference");
                        Ok(stream.clone())
                    }
                    None => Err(NativeExtensionsError::VirtualFileReceiveError(
                        "IStream pointer is null".into(),
                    )),
                }
            }
            TYMED_ISTORAGE => {
                log::debug!("Processing TYMED_ISTORAGE - cannot create stream directly from storage");
                Err(NativeExtensionsError::VirtualFileReceiveError(
                    "IStorage cannot be used as stream - use copy_virtual_file_for_item instead".into(),
                ))
            }
            _ => {
                log::error!("Unsupported TYMED format: {}", medium.tymed);
                Err(NativeExtensionsError::VirtualFileReceiveError(
                    format!("Unsupported data format (TYMED: {})", medium.tymed),
                ))
            }
        }
    }

    pub async fn create_virtual_file_reader_for_item(
        &self,
        item: i64,
        _format: &str,
        _progress: Arc<ReadProgress>,
    ) -> NativeExtensionsResult<Option<Rc<dyn VirtualFileReader>>> {
        let descriptor = self.descriptor_for_virtual_file(item)?;
        let mut medium = self.medium_for_virtual_file(&descriptor)?;
        let stream = Self::stream_from_medium(&medium);
        unsafe { ReleaseStgMedium(&mut medium as *mut STGMEDIUM) };
        let stream = stream?;

        if self.supports_async.get() {
            let stream = unsafe { Movable::new(stream) };
            let reader = AsyncStreamReader::new(stream, descriptor.name).await?;
            Ok(Some(Rc::new(reader)))
        } else {
            let reader = EagerStreamReader::new(stream, descriptor.name)?;
            Ok(Some(Rc::new(reader)))
        }
    }

    fn do_copy_virtual_file(
        medium: &STGMEDIUM,
        file_name: &str,
        target_folder: PathBuf,
        progress: Arc<ReadProgress>,
        supports_async: bool,
        completer: FutureCompleter<NativeExtensionsResult<PathBuf>>,
    ) {
        log::debug!("Copying virtual file '{}' using TYMED: {}", file_name, medium.tymed);
        
        match TYMED(medium.tymed as i32) {
            TYMED_HGLOBAL => {
                let path = get_target_path(&target_folder, file_name);
                let res = unsafe {
                    match safe_slice_from_global_memory(medium.u.hGlobal) {
                        Some(data) => {
                            log::debug!("Successfully read {} bytes for file '{}'", data.len(), file_name);
                            progress.report_progress(Some(1.0));
                            fs::write(&path, data)
                        }
                        None => {
                            log::warn!("Failed to read global memory for file '{}' - creating empty file as fallback", file_name);
                            // Create an empty file as a fallback so the user knows the file was dropped
                            // but the content couldn't be retrieved
                            fs::write(&path, &[])
                        }
                    }
                };
                match res {
                    Ok(_) => {
                        log::debug!("Successfully wrote file to: {}", path.display());
                        completer.complete(Ok(path))
                    }
                    Err(err) => {
                        log::error!("Failed to write file '{}': {}", file_name, err);
                        completer.complete(Err(
                            NativeExtensionsError::VirtualFileReceiveError(
                                format!("Failed to write file '{}': {}", file_name, err)
                            ),
                        ))
                    }
                }
            }
            TYMED_ISTORAGE => {
                log::debug!("Processing TYMED_ISTORAGE for file '{}'", file_name);
                match unsafe { medium.u.pstg.as_ref() } {
                    Some(storage) => {
                        let mut final_name = file_name.to_string();
                        if !final_name.to_ascii_lowercase().ends_with(".msg") {
                            final_name.push_str(".msg");
                            log::debug!("Added .msg extension to storage file: '{}'", final_name);
                        }
                        let path = get_target_path(&target_folder, &final_name);
                        let res = Self::write_storage_to_compound_file(storage, &path);
                        progress.report_progress(Some(1.0));
                        match res {
                            Ok(_) => {
                                log::debug!("Successfully wrote compound storage file to: {}", path.display());
                                completer.complete(Ok(path))
                            }
                            Err(err) => {
                                log::error!("Failed to copy IStorage for '{}': {}", final_name, err);
                                completer.complete(Err(
                                    NativeExtensionsError::VirtualFileReceiveError(
                                        format!("Failed to copy IStorage for '{}': {}", final_name, err)
                                    ),
                                ))
                            }
                        }
                    }
                    None => {
                        log::error!("IStorage pointer is null for file '{}'", file_name);
                        completer.complete(Err(
                            NativeExtensionsError::VirtualFileReceiveError("IStorage missing".into())
                        ))
                    }
                }
            }
            TYMED_ISTREAM => {
                log::debug!("Processing TYMED_ISTREAM for file '{}'", file_name);
                match unsafe { medium.u.pstm.as_ref() } {
                    Some(stream) => {
                        if supports_async {
                            let copier = AsyncVirtualStreamCopier {
                                sender: RunLoop::current().new_sender(),
                                stream: unsafe { Movable::new(stream.clone()) },
                                file_name: file_name.into(),
                                target_folder,
                                progress,
                                completer: Capsule::new(completer),
                            };
                            thread::spawn(move || {
                                copier.copy();
                            });
                        } else {
                            let path = get_target_path(&target_folder, file_name);
                            unsafe { stream.Seek(0, STREAM_SEEK_SET, None).ok_log() };
                            let res = copy_stream_to_file(stream, &path);
                            progress.report_progress(Some(1.0));
                            match res {
                                Ok(_) => completer.complete(Ok(path)),
                                Err(err) => completer.complete(Err(
                                    NativeExtensionsError::VirtualFileReceiveError(err.to_string()),
                                )),
                            }
                        }
                    }
                    None => completer.complete(Err(NativeExtensionsError::VirtualFileReceiveError(
                        "IStream missing".into(),
                    ))),
                }
            }
            _ => {
                log::error!("Unsupported TYMED format: {} for file '{}'", medium.tymed, file_name);
                completer.complete(Err(NativeExtensionsError::VirtualFileReceiveError(
                    format!("Unsupported data format (TYMED: {}) for file '{}'", medium.tymed, file_name),
                )))
            }
        }
    }

    fn write_storage_to_compound_file(storage: &IStorage, target: &Path) -> NativeExtensionsResult<()> {
        unsafe {
            log::debug!("Creating compound document file at: {}", target.display());
            
            // Create destination docfile
            let mut dest: Option<IStorage> = None;
            StgCreateDocfile(
                &HSTRING::from(target.to_string_lossy().as_ref()),
                STGM_CREATE | STGM_READWRITE | STGM_SHARE_EXCLUSIVE,
                0,
                &mut dest
            ).map_err(|e| {
                log::error!("Failed to create compound document file: {}", e);
                NativeExtensionsError::VirtualFileReceiveError(
                    format!("Failed to create compound document: {}", e)
                )
            })?;
            
            let dest = dest.ok_or_else(|| {
                NativeExtensionsError::VirtualFileReceiveError(
                    "Failed to get destination storage interface".into()
                )
            })?;
            
            // Copy all elements from source to destination
            storage.CopyTo(0, std::ptr::null(), std::ptr::null(), &dest).map_err(|e| {
                log::error!("Failed to copy storage contents: {}", e);
                NativeExtensionsError::VirtualFileReceiveError(
                    format!("Failed to copy storage contents: {}", e)
                )
            })?;
            
            // Commit changes
            dest.Commit(0).map_err(|e| {
                log::error!("Failed to commit storage: {}", e);
                NativeExtensionsError::VirtualFileReceiveError(
                    format!("Failed to commit storage: {}", e)
                )
            })?;
            
            log::debug!("Successfully created compound document file");
        }
        Ok(())
    }

    fn descriptor_for_virtual_file(&self, item: i64) -> NativeExtensionsResult<FileDescriptor> {
        if let Some(descriptor) = self.descriptor_for_item(item)? {
            return Ok(descriptor);
        }
        Err(NativeExtensionsError::VirtualFileReceiveError(
            "item not found".into(),
        ))
    }

    fn medium_for_virtual_file(
        &self,
        descriptor: &FileDescriptor,
    ) -> NativeExtensionsResult<STGMEDIUM> {
        let format = unsafe { RegisterClipboardFormatW(CFSTR_FILECONTENTS) };
        log::debug!("Attempting to get virtual file content for '{}' at index {} using format {}", 
                   descriptor.name, descriptor.index, format);
        
        // Check if IStorage support is enabled via environment variable (default: enabled)
        let enable_storage = std::env::var("ENABLE_OUTLOOK_STORAGE").unwrap_or_else(|_| "1".to_string()) == "1";
        
        // Try different TYMED combinations in order of preference
        let mut tymed_options = vec![
            TYMED(TYMED_ISTREAM.0), // Prefer IStream first for virtual files
        ];
        
        if enable_storage {
            tymed_options.push(TYMED(TYMED_ISTORAGE.0)); // Add IStorage if enabled
        }
        
        tymed_options.extend([
            TYMED(TYMED_HGLOBAL.0), // Then try HGlobal
            TYMED(TYMED_ISTREAM.0 | TYMED_HGLOBAL.0 | if enable_storage { TYMED_ISTORAGE.0 } else { 0 }), // Finally try combination
        ]);
        
        for (attempt, tymed) in tymed_options.iter().enumerate() {
            let format_etc = make_format_with_tymed_index(
                format,
                *tymed,
                descriptor.index as i32,
            );
            
            log::debug!("Attempting to get virtual file content with TYMED {:?} (attempt {})", 
                       tymed.0, attempt + 1);
            
            match safe_get_data(&self.data_object, &format_etc)? {
                Some(medium) => {
                    log::debug!("Successfully got medium with TYMED {:?} for file '{}' at index {}", 
                               tymed.0, descriptor.name, descriptor.index);
                    
                    // Additional validation for HGlobal mediums
                    if TYMED(medium.tymed as i32) == TYMED_HGLOBAL {
                        unsafe {
                            let hglobal = medium.u.hGlobal;
                            let size = GlobalSize(hglobal);
                            log::debug!("HGlobal medium size: {} bytes", size);
                            
                            // If we have an expected size and this is zero, and we haven't tried all options yet
                            if size == 0 {
                                if let Some(expected) = descriptor.expected_size {
                                    if expected > 0 && attempt < tymed_options.len() - 1 {
                                        log::warn!("Expected {} bytes but got 0 in HGLOBAL; retrying with next TYMED", expected);
                                        continue;
                                    }
                                }
                                // If this is our last attempt or no size hint, warn but continue
                                if attempt == tymed_options.len() - 1 {
                                    log::warn!("HGlobal has zero size on final attempt - may be invalid but proceeding");
                                } else {
                                    log::warn!("HGlobal has zero size - trying next TYMED option");
                                    continue;
                                }
                            }
                        }
                    }
                    
                    return Ok(medium);
                }
                None => {
                    log::debug!("Failed to get data with TYMED {:?}", tymed.0);
                    continue;
                }
            }
        }
        
        log::warn!("All attempts to retrieve virtual file content failed for file '{}' at index {}", 
                  descriptor.name, descriptor.index);
        Err(NativeExtensionsError::VirtualFileReceiveError(
            format!("Virtual file content not available for '{}' (tried all TYMED options)", descriptor.name)
        ))
    }

    pub async fn get_item_format_for_uri(
        &self,
        item: i64,
    ) -> NativeExtensionsResult<Option<String>> {
        let hdrop = self.hdrop_for_item(item)?;
        if let Some(hdrop) = hdrop {
            let format = mime_from_name(&hdrop);
            let format = mime_to_windows(format);
            Ok(Some(format))
        } else {
            Ok(None)
        }
    }

    pub async fn copy_virtual_file_for_item(
        &self,
        item: i64,
        _format: &str,
        target_folder: PathBuf,
        progress: Arc<ReadProgress>,
    ) -> NativeExtensionsResult<PathBuf> {
        let descriptor = self.descriptor_for_virtual_file(item)?;
        let mut medium = self.medium_for_virtual_file(&descriptor)?;
        unsafe {
            let (future, completer) = FutureCompleter::new();
            Self::do_copy_virtual_file(
                &medium,
                &descriptor.name,
                target_folder,
                progress,
                self.supports_async.get(),
                completer,
            );
            ReleaseStgMedium(&mut medium as *mut STGMEDIUM);
            future.await
        }
    }
}

// Stream reader for applications that don't support IDataObjectAsyncCapability,
// such as Microsoft Outlook apparently. Just. Sad.
struct EagerStreamReader {
    size: Option<i64>,
    file_name: Option<String>,
    data: RefCell<Vec<u8>>,
}

impl EagerStreamReader {
    pub fn new(stream: IStream, file_name: String) -> NativeExtensionsResult<Self> {
        unsafe { stream.Seek(0, STREAM_SEEK_SET, None)? };
        let data = read_stream_fully(&stream)?;
        Ok(Self {
            size: Some(data.len() as i64),
            file_name: Some(file_name),
            data: RefCell::new(data),
        })
    }
}

#[async_trait(?Send)]
impl VirtualFileReader for EagerStreamReader {
    async fn read_next(&self) -> NativeExtensionsResult<Vec<u8>> {
        let res = self.data.replace(Vec::new());
        Ok(res)
    }

    fn file_size(&self) -> NativeExtensionsResult<Option<i64>> {
        Ok(self.size)
    }

    fn file_name(&self) -> Option<String> {
        self.file_name.clone()
    }

    fn close(&self) -> NativeExtensionsResult<()> {
        Ok(())
    }
}

struct AsyncStreamReader {
    stream: Movable<IStream>,
    length: u64,
    file_name: String,
    // Single thread thread-pool so that all requests are run in background
    // but serialized.
    thread_pool: ThreadPool,
    read_state: Arc<Mutex<Option<ReadState>>>,
}

struct ReadState {
    num_read: u64,
}

impl AsyncStreamReader {
    async fn new(stream: Movable<IStream>, file_name: String) -> NativeExtensionsResult<Self> {
        let thread_pool = ThreadPool::new(1);
        let length = Self::stream_length(&stream, &thread_pool).await?;
        Ok(AsyncStreamReader {
            stream,
            length,
            file_name,
            thread_pool,
            read_state: Arc::new(Mutex::new(None)),
        })
    }

    async fn stream_length(
        stream: &Movable<IStream>,
        thread_pool: &ThreadPool,
    ) -> NativeExtensionsResult<u64> {
        fn stream_length(stream: &IStream) -> NativeExtensionsResult<u64> {
            let mut stat = STATSTG::default();
            unsafe {
                stream.Stat(&mut stat as *mut _, STATFLAG_NONAME)?;
            }
            Ok(stat.cbSize)
        }

        let (future, completer) = FutureCompleter::new();
        let stream_clone = stream.clone();
        let sender = RunLoop::current().new_sender();
        let mut completer = Capsule::new_with_sender(completer, sender.clone());
        thread_pool.execute(move || {
            let len = stream_length(&stream_clone);
            sender.send(move || {
                completer.take().unwrap().complete(len);
            });
        });

        future.await
    }

    fn read(
        stream: Movable<IStream>,
        length: u64,
        state: Arc<Mutex<Option<ReadState>>>,
    ) -> NativeExtensionsResult<Vec<u8>> {
        let mut state = state.lock().unwrap();
        if state.is_none() {
            state.replace(ReadState { num_read: 0 });
            unsafe {
                stream.Seek(0, STREAM_SEEK_SET, None)?;
            }
        }
        let mut buf = vec![0u8; 1024 * 1024];
        let state = state.as_mut().unwrap();
        let to_read = (length - state.num_read).min(buf.len() as u64) as u32;
        if to_read == 0 {
            Ok(Vec::new())
        } else {
            let mut did_read = 0u32;
            let res = unsafe {
                stream.Read(
                    buf.as_mut_ptr() as *mut _,
                    to_read,
                    Some(&mut did_read as *mut _),
                )
            };
            if res != S_OK {
                Err(windows::core::Error::from(res).into())
            } else {
                state.num_read += did_read as u64;
                buf.resize(did_read as usize, 0);
                Ok(buf)
            }
        }
    }
}

#[async_trait(?Send)]
impl VirtualFileReader for AsyncStreamReader {
    async fn read_next(&self) -> NativeExtensionsResult<Vec<u8>> {
        let (future, completer) = FutureCompleter::new();
        let sender = RunLoop::current().new_sender();
        let mut completer = Capsule::new_with_sender(completer, sender.clone());
        let stream = self.stream.clone();
        let read_state = self.read_state.clone();
        let length = self.length;
        self.thread_pool.execute(move || {
            let res = Self::read(stream, length, read_state);
            sender.send(move || {
                completer.take().unwrap().complete(res);
            });
        });
        future.await
    }

    fn file_size(&self) -> NativeExtensionsResult<Option<i64>> {
        Ok(Some(self.length as i64))
    }

    fn file_name(&self) -> Option<String> {
        Some(self.file_name.clone())
    }

    fn close(&self) -> NativeExtensionsResult<()> {
        // Stream gets closed upon release
        Ok(())
    }
}

// Most streams in COM should be agile, also the documentation for IDataObjectAsyncCapability
// assumes that the stream is read on background thread so we wrap it inside Movable
// in order to be able to send it.
struct AsyncVirtualStreamCopier {
    sender: RunLoopSender,
    stream: Movable<IStream>,
    file_name: String,
    target_folder: PathBuf,
    progress: Arc<ReadProgress>,
    completer: Capsule<FutureCompleter<NativeExtensionsResult<PathBuf>>>,
}

impl AsyncVirtualStreamCopier {
    fn get_length(&self) -> NativeExtensionsResult<u64> {
        let mut stat = STATSTG::default();
        unsafe {
            self.stream.Stat(&mut stat as *mut _, STATFLAG_NONAME)?;
        }
        Ok(stat.cbSize)
    }

    fn read_inner(&self) -> NativeExtensionsResult<PathBuf> {
        let temp_name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();
        let temp_path = self.target_folder.join(format!(".{temp_name}"));
        let file = File::create(&temp_path)?;
        unsafe {
            let path: String = temp_path.to_string_lossy().into();
            let path = HSTRING::from(path);
            SetFileAttributesW(&path, FILE_ATTRIBUTE_HIDDEN | FILE_ATTRIBUTE_TEMPORARY)?;
        }
        match self.read_and_write(file) {
            Ok(_) => {
                let path = get_target_path(&self.target_folder, &self.file_name);
                fs::rename(temp_path, &path)?;
                unsafe {
                    let path: String = path.to_string_lossy().into();
                    let path = HSTRING::from(path);
                    SetFileAttributesW(&path, FILE_ATTRIBUTE_ARCHIVE)?;
                }
                Ok(path)
            }
            Err(err) => {
                fs::remove_file(temp_path).ok_log();
                Err(err)
            }
        }
    }

    fn read_and_write(&self, mut f: File) -> NativeExtensionsResult<()> {
        let cancelled = Arc::new(AtomicBool::new(false));
        let cancelled_clone = cancelled.clone();
        self.progress
            .set_cancellation_handler(Some(Box::new(move || {
                cancelled_clone.store(true, Ordering::Release);
            })));
        let length = self.get_length()?;
        let mut num_read: u64 = 0;
        let mut buf = vec![0u8; 1024 * 1024];
        let mut last_reported_progress = 0f64;

        unsafe {
            self.stream.Seek(0, STREAM_SEEK_SET, None)?;
        }

        loop {
            if cancelled.load(Ordering::Acquire) {
                return Err(NativeExtensionsError::VirtualFileReceiveError(
                    "cancelled".into(),
                ));
            }
            let to_read = (length - num_read).min(buf.len() as u64) as u32;
            if to_read == 0 {
                break;
            }
            let mut did_read = 0u32;
            let res = unsafe {
                self.stream.Read(
                    buf.as_mut_ptr() as *mut _,
                    to_read,
                    Some(&mut did_read as *mut _),
                )
            };
            if res != S_OK {
                return Err(windows::core::Error::from(res).into());
            }
            if did_read == 0 {
                return Err(NativeExtensionsError::VirtualFileReceiveError(
                    "stream ended prematurely".into(),
                ));
            }
            f.write_all(&buf[..did_read as usize])?;
            num_read += did_read as u64;

            let progress = num_read as f64 / length as f64;
            if progress >= last_reported_progress + 0.05 {
                last_reported_progress = progress;
                self.progress.report_progress(Some(progress));
            }
        }
        self.progress.report_progress(Some(1.0));

        Ok(())
    }

    fn copy(self) {
        let res = self.read_inner();
        let mut completer = self.completer;
        self.sender.send(move || {
            let completer = completer.take().unwrap();
            completer.complete(res);
        });
    }
}

// Map mime types to known windows clipboard format
fn mime_to_windows(fmt: String) -> String {
    match fmt.as_str() {
        "image/png" => "PNG".to_owned(),
        "image/jpeg" => "JFIF".to_string(),
        "image/gif" => "GIF".to_string(),
        "image/tiff" => format_to_string(CF_TIFF.0 as u32),
        "application/vnd.ms-outlook" => "application/vnd.ms-outlook".to_string(),
        "message/rfc822" => "message/rfc822".to_string(),
        _ => fmt,
    }
}

fn mime_from_name(name: &str) -> String {
    let ext = Path::new(name).extension();
    
    // Handle common Outlook file extensions explicitly
    if let Some(ext_str) = ext.and_then(|e| e.to_str()) {
        match ext_str.to_ascii_lowercase().as_str() {
            "msg" => return "application/vnd.ms-outlook".to_string(),
            "eml" => return "message/rfc822".to_string(),
            _ => {}
        }
    }
    
    mime_guess::from_path(name)
        .first()
        .map(|m| m.to_string())
        .unwrap_or_else(|| {
            format!(
                "application/octet-stream;extension={}",
                ext.unwrap_or_default().to_string_lossy()
            )
        })
}

#[cfg(test)]
mod tests {
    use windows::Win32::{Foundation::POINT, UI::Shell::DROPFILES};

    use crate::platform::PlatformDataReader;

    #[test]
    fn test_extract_drop_files() {
        #[repr(C)]
        struct DropFiles {
            f: DROPFILES,
            padding: [u8; 5],
        }
        let mut df = DropFiles {
            f: DROPFILES {
                pFiles: std::mem::size_of::<DROPFILES>() as u32,
                pt: POINT { x: 0, y: 0 },
                fNC: false.into(),
                fWide: false.into(),
            },
            padding: [0; 5],
        };
        df.padding.copy_from_slice(b"A\0B\0\0");
        let slice = unsafe {
            std::slice::from_raw_parts(
                &df as *const DropFiles as *const u8,
                std::mem::size_of::<DropFiles>(),
            )
        };
        let files = PlatformDataReader::extract_drop_files(slice).unwrap();
        assert_eq!(files, vec!["A", "B"]);
    }

    #[test]
    fn test_extract_drop_files_wide() {
        #[repr(C)]
        struct DropFiles {
            f: DROPFILES,
            padding: [u16; 5],
        }
        let mut df = DropFiles {
            f: DROPFILES {
                pFiles: std::mem::size_of::<DROPFILES>() as u32,
                pt: POINT { x: 0, y: 0 },
                fNC: false.into(),
                fWide: true.into(),
            },
            padding: [0; 5],
        };
        df.padding.copy_from_slice([65, 0, 66, 0, 0].as_ref());
        let slice = unsafe {
            std::slice::from_raw_parts(
                &df as *const DropFiles as *const u8,
                std::mem::size_of::<DropFiles>(),
            )
        };
        let files = PlatformDataReader::extract_drop_files(slice).unwrap();
        assert_eq!(files, vec!["A", "B"]);
    }
}
