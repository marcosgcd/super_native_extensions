use std::{fs::OpenOptions, io::Write, mem::size_of, path::Path, ptr::null_mut, slice};

use once_cell::sync::Lazy;
use windows::{
    core::{s, ComInterface, GUID, HRESULT, HSTRING},
    Win32::{
        Foundation::{E_UNEXPECTED, HANDLE, HWND, S_OK, DV_E_FORMATETC},
        Graphics::Gdi::{
            CreateDIBSection, GetDC, GetDeviceCaps, MonitorFromWindow, ReleaseDC, BITMAPINFO,
            BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBITMAP, HMONITOR, LOGPIXELSX,
            MONITOR_DEFAULTTOPRIMARY,
        },
        System::{
            Com::{
                CoCreateInstance, IDataObject, IStream, CLSCTX_ALL, DATADIR_GET, DVASPECT_CONTENT,
                FORMATETC, TYMED, TYMED_HGLOBAL, TYMED_ISTREAM, TYMED_ISTORAGE, TYMED_FILE,
                STGMEDIUM, STREAM_SEEK_SET,
            },
            DataExchange::{GetClipboardFormatNameW, RegisterClipboardFormatW, RegisterClipboardFormatA},
            LibraryLoader::{GetProcAddress, LoadLibraryA},
            Memory::{GlobalLock, GlobalSize, GlobalUnlock},
            Ole::{ReleaseStgMedium, CF_HDROP},
        },
        UI::HiDpi::{MDT_EFFECTIVE_DPI, MONITOR_DPI_TYPE},
    },
};

use crate::{
    api_model::ImageData,
    error::{NativeExtensionsError, NativeExtensionsResult},
    log::OkLog,
};

const INTERNAL_PREFIX: &str = "NativeShell_CF_";

// Outlook-specific clipboard format constants
const CF_OUTLOOK_MSG: &[u8] = b"RenPrivateMessages\0";
const CF_OUTLOOK_ATTACH: &[u8] = b"RenPrivateAttachments\0";
const CF_FILEDESCRIPTOR: &[u8] = b"FileGroupDescriptor\0";
const CF_FILECONTENTS: &[u8] = b"FileContents\0";
const CF_UNIFORMRESOURCELOCATOR: &[u8] = b"UniformResourceLocator\0";
const CF_HDROP_STR: &[u8] = b"CF_HDROP\0";
const CF_TEXT_STR: &[u8] = b"CF_TEXT\0";
const CF_UNICODETEXT_STR: &[u8] = b"CF_UNICODETEXT\0";

/// Safe wrapper for IDataObject::GetData that prevents DV_E_FORMATETC from surfacing
pub fn safe_get_data(obj: &IDataObject, fmt: &FORMATETC) -> NativeExtensionsResult<Option<STGMEDIUM>> {
    unsafe {
        // First check if format is available
        if obj.QueryGetData(fmt).is_err() {
            log::debug!("Format not available (QueryGetData failed), skipping: cfFormat={}, tymed={:?}", fmt.cfFormat, fmt.tymed);
            return Ok(None);
        }
        
        // Log the COM call for debugging
        log::trace!("GetData cfFormat={}, tymed={:?}, lindex={}", fmt.cfFormat, fmt.tymed, fmt.lindex);
        
        match obj.GetData(fmt) {
            Ok(medium) => Ok(Some(medium)),
            Err(e) => {
                if e.code().0 == 0x80040064 {
                    // DV_E_FORMATETC - format not supported, this is expected
                    log::debug!("DV_E_FORMATETC for format {}, returning None", fmt.cfFormat);
                    Ok(None)
                } else {
                    // Other errors should be propagated
                    Err(NativeExtensionsError::from(e))
                }
            }
        }
    }
}

/// Safe wrapper for IDataObject::EnumFormatEtc that handles DV_E_FORMATETC gracefully
pub fn safe_enum_format_etc(obj: &IDataObject) -> NativeExtensionsResult<Vec<FORMATETC>> {
    unsafe {
        log::trace!("EnumFormatEtc");
        
        match obj.EnumFormatEtc(DATADIR_GET.0 as u32) {
            Ok(enumerator) => {
                let mut res = Vec::new();
                loop {
                    let mut format = [FORMATETC::default()];
                    let mut fetched = 0u32;
                    if enumerator.Next(&mut format, Some(&mut fetched as *mut _)).is_err() || fetched == 0 {
                        break;
                    }
                    res.push(format[0]);
                }
                log::debug!("EnumFormatEtc succeeded, found {} formats", res.len());
                Ok(res)
            }
            Err(e) if e.code().0 == 0x80040064 => {
                // DV_E_FORMATETC - format enumeration not supported
                log::debug!("DV_E_FORMATETC in EnumFormatEtc, using fallback formats");
                Ok(try_common_outlook_formats(obj))
            }
            Err(e) => {
                log::warn!("EnumFormatEtc failed with error: {}", e);
                Err(NativeExtensionsError::from(e))
            }
        }
    }
}

pub fn format_to_string(format: u32) -> String {
    let mut buf: [_; 1024] = [0u16; 1024];
    let len = unsafe { GetClipboardFormatNameW(format, &mut buf) };
    if len == 0 {
        format!("{INTERNAL_PREFIX}{format}")
    } else {
        String::from_utf16_lossy(&buf[..len as usize])
    }
}

pub fn format_from_string(format: &str) -> u32 {
    if let Some(format) = format.strip_prefix(INTERNAL_PREFIX) {
        format.parse::<u32>().ok().unwrap_or(0)
    } else {
        unsafe { RegisterClipboardFormatW(&HSTRING::from(format)) }
    }
}

pub fn make_format_with_tymed(format: u32, tymed: TYMED) -> FORMATETC {
    make_format_with_tymed_index(format, tymed, -1)
}

pub fn make_format_with_tymed_index(format: u32, tymed: TYMED, index: i32) -> FORMATETC {
    FORMATETC {
        cfFormat: format as u16,
        ptd: null_mut(),
        dwAspect: DVASPECT_CONTENT.0,
        lindex: index,
        tymed: tymed.0 as u32,
    }
}

impl From<windows::core::Error> for NativeExtensionsError {
    fn from(error: windows::core::Error) -> Self {
        NativeExtensionsError::WindowsError(error.code(), format!("Windows Error: {error}"))
    }
}

/// # Safety
///
/// Data must be properly aligned (see slice::from_raw_parts)
pub unsafe fn as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
}

/// Check if this is an Outlook email drag operation
pub fn is_outlook_email_drag(data_object: &IDataObject) -> bool {
    log::debug!("Checking if this is an Outlook email drag operation");
    
    // Check for Outlook-specific formats
    let msg_fmt = match unsafe { RegisterClipboardFormatA(CF_OUTLOOK_MSG.as_ptr() as _) } {
        Ok(fmt) => fmt,
        Err(_) => {
            log::warn!("Failed to register CF_OUTLOOK_MSG format");
            return false;
        }
    };
    
    let desc_fmt = match unsafe { RegisterClipboardFormatA(CF_FILEDESCRIPTOR.as_ptr() as _) } {
        Ok(fmt) => fmt,
        Err(_) => {
            log::warn!("Failed to register CF_FILEDESCRIPTOR format");
            return false;
        }
    };
    
    let result = query_format_with_fallback_safe(data_object, msg_fmt) || 
                 query_format_with_fallback_safe(data_object, desc_fmt);
    
    log::debug!("Outlook email drag check result: {}", result);
    result
}

/// Safe version of query_format_with_fallback that doesn't propagate errors
pub fn query_format_with_fallback_safe(data_object: &IDataObject, format: u32) -> bool {
    let tymed_options = [
        TYMED_HGLOBAL,
        TYMED_ISTREAM,
        TYMED_ISTORAGE,
        TYMED_FILE,
    ];
    
    for tymed in tymed_options {
        let formatetc = FORMATETC {
            cfFormat: format as u16,
            ptd: std::ptr::null_mut(),
            dwAspect: DVASPECT_CONTENT.0,
            lindex: -1,
            tymed: tymed.0,
        };
        
        match unsafe { data_object.QueryGetData(&formatetc) } {
            Ok(_) => {
                log::debug!("Format {} available with TYMED {}", format, tymed.0);
                return true;
            }
            Err(e) => {
                log::debug!("Format {} not available with TYMED {}: {}", format, tymed.0, e);
                // Continue to try other TYMED values
            }
        }
    }
    
    log::debug!("Format {} not available with any TYMED", format);
    false
}

/// Query format with multiple TYMED values as fallback
pub fn query_format_with_fallback(data_object: &IDataObject, format: u32) -> bool {
    let tymed_options = [
        TYMED_HGLOBAL,
        TYMED_ISTREAM,
        TYMED_ISTORAGE,
        TYMED_FILE,
    ];
    
    for tymed in tymed_options {
        let formatetc = FORMATETC {
            cfFormat: format as u16,
            ptd: std::ptr::null_mut(),
            dwAspect: DVASPECT_CONTENT.0,
            lindex: -1,
            tymed: tymed.0,
        };
        
        if unsafe { data_object.QueryGetData(&formatetc) }.is_ok() {
            return true;
        }
    }
    false
}

/// Try common Outlook formats when enumeration fails
pub fn try_common_outlook_formats(data_object: &IDataObject) -> Vec<FORMATETC> {
    log::info!("Trying common Outlook formats as fallback");
    let mut formats = Vec::new();
    
    // Common Outlook formats to try
    let outlook_formats = [
        CF_OUTLOOK_MSG,
        CF_OUTLOOK_ATTACH,
        CF_FILEDESCRIPTOR,
        CF_FILECONTENTS,
        CF_UNIFORMRESOURCELOCATOR,
        CF_HDROP_STR,
        CF_TEXT_STR,
        CF_UNICODETEXT_STR,
    ];
    
    for format_name in outlook_formats {
        let format_name_str = std::str::from_utf8(format_name).unwrap_or("unknown");
        
        match unsafe { RegisterClipboardFormatA(format_name.as_ptr() as _) } {
            Ok(format_id) => {
                log::debug!("Registered format '{}' with ID {}", format_name_str, format_id);
                
                // Try different TYMED values for each format
                let tymed_options = [
                    TYMED_HGLOBAL,
                    TYMED_ISTREAM,
                    TYMED_ISTORAGE,
                    TYMED_FILE,
                ];
                
                for tymed in tymed_options {
                    let formatetc = FORMATETC {
                        cfFormat: format_id as u16,
                        ptd: std::ptr::null_mut(),
                        dwAspect: DVASPECT_CONTENT.0,
                        lindex: -1,
                        tymed: tymed.0,
                    };
                    
                    match unsafe { data_object.QueryGetData(&formatetc) } {
                        Ok(_) => {
                            log::info!("Found working format '{}' with TYMED {}", format_name_str, tymed.0);
                            formats.push(formatetc);
                            break; // Found a working TYMED for this format
                        }
                        Err(e) => {
                            log::debug!("Format '{}' not available with TYMED {}: {}", format_name_str, tymed.0, e);
                            // Continue to try other TYMED values
                        }
                    }
                }
            }
            Err(e) => {
                log::warn!("Failed to register format '{}': {}", format_name_str, e);
            }
        }
    }
    
    log::info!("Found {} working Outlook formats", formats.len());
    formats
}

/// Safely enumerate formats with fallback for Outlook
pub fn enumerate_formats_safely(data_object: &IDataObject) -> NativeExtensionsResult<Vec<FORMATETC>> {
    safe_enum_format_etc(data_object)
}

pub fn log_outlook_format_detection(data_object: &IDataObject) {
    log::info!("Checking for Outlook email drag operation...");
    
    // Test individual format availability
    let test_formats = [
        ("RenPrivateMessages", CF_OUTLOOK_MSG),
        ("RenPrivateAttachments", CF_OUTLOOK_ATTACH),
        ("FileGroupDescriptor", CF_FILEDESCRIPTOR),
        ("FileContents", CF_FILECONTENTS),
        ("UniformResourceLocator", CF_UNIFORMRESOURCELOCATOR),
    ];
    
    for (name, format_bytes) in test_formats {
        match unsafe { RegisterClipboardFormatA(format_bytes.as_ptr() as _) } {
            Ok(format_id) => {
                let available = query_format_with_fallback_safe(data_object, format_id);
                log::info!("Format '{}' (ID: {}): {}", name, format_id, if available { "Available" } else { "Not available" });
            }
            Err(e) => {
                log::warn!("Failed to register format '{}': {}", name, e);
            }
        }
    }
}

pub fn extract_formats(object: &IDataObject) -> NativeExtensionsResult<Vec<FORMATETC>> {
    log::info!("Attempting to extract formats from data object");
    
    let result = enumerate_formats_safely(object);
    match &result {
        Ok(formats) => {
            log::info!("Successfully extracted {} formats", formats.len());
            for (i, format) in formats.iter().enumerate() {
                log::debug!("Format {}: cfFormat={}, tymed={}, lindex={}", 
                    i, format.cfFormat, format.tymed, format.lindex);
            }
        }
        Err(e) => {
            log::warn!("Format extraction failed: {}", e);
            log_outlook_format_detection(object);
        }
    }
    
    result
}

pub fn image_data_to_hbitmap(image: &ImageData) -> NativeExtensionsResult<HBITMAP> {
    let bitmap = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: image.width,
            biHeight: image.height,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB.0,
            biSizeImage: (image.width * image.height * 4) as u32,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: Default::default(),
    };

    unsafe {
        let dc = GetDC(HWND(0));

        let mut ptr = std::ptr::null_mut();

        let bitmap = CreateDIBSection(
            dc,
            &bitmap as *const _,
            DIB_RGB_COLORS,
            &mut ptr as *mut *mut _,
            HANDLE(0),
            0,
        )?;

        // Bitmap needs to be flipped and unpremultiplied

        let dst_stride = (image.width * 4) as isize;
        let ptr = ptr as *mut u8;
        for y in 0..image.height as isize {
            let src_line = image
                .data
                .as_ptr()
                .offset((image.height as isize - y - 1) * image.bytes_per_row as isize);

            let dst_line = ptr.offset(y * dst_stride);

            for x in (0..dst_stride).step_by(4) {
                let (r, g, b, a) = (
                    *src_line.offset(x) as i32,
                    *src_line.offset(x + 1) as i32,
                    *src_line.offset(x + 2) as i32,
                    *src_line.offset(x + 3) as i32,
                );

                // ByteFormat.rawStraightRgba already has unpremultiplied alpha
                // but channel order is different.

                *dst_line.offset(x) = b as u8;
                *dst_line.offset(x + 1) = g as u8;
                *dst_line.offset(x + 2) = r as u8;
                *dst_line.offset(x + 3) = a as u8;
            }
        }

        ReleaseDC(HWND(0), dc);

        Ok(bitmap)
    }
}

pub fn create_instance<T: ComInterface>(clsid: &GUID) -> windows::core::Result<T> {
    unsafe { CoCreateInstance(clsid, None, CLSCTX_ALL) }
}

impl From<NativeExtensionsError> for windows::core::Error {
    fn from(err: NativeExtensionsError) -> Self {
        windows::core::Error::new(E_UNEXPECTED, err.to_string().into())
    }
}

type GetDpiForMonitor = unsafe extern "system" fn(
    hmonitor: HMONITOR,
    dpitype: MONITOR_DPI_TYPE,
    dpix: *mut u32,
    dpiy: *mut u32,
) -> HRESULT;

type GetDpiForWindow = unsafe extern "system" fn(hwnd: HWND) -> u32;

struct DpiFunctions {
    get_dpi_for_window: Option<GetDpiForWindow>,
    get_dpi_for_monitor: Option<GetDpiForMonitor>,
}

impl DpiFunctions {
    fn new() -> Self {
        unsafe {
            let user_32 = LoadLibraryA(s!("user32")).unwrap();
            let shlib = LoadLibraryA(s!("Shcore.dll")).unwrap();
            Self {
                #[allow(clippy::missing_transmute_annotations)]
                get_dpi_for_window: std::mem::transmute(GetProcAddress(
                    user_32,
                    s!("GetDpiForWindow"),
                )),
                #[allow(clippy::missing_transmute_annotations)]
                get_dpi_for_monitor: std::mem::transmute(GetProcAddress(
                    shlib,
                    s!("GetDpiForMonitor"),
                )),
            }
        }
    }
}

static DPI_FUNCTIONS: Lazy<DpiFunctions> = Lazy::new(DpiFunctions::new);

pub fn get_dpi_for_window(hwnd: HWND) -> u32 {
    if let Some(get_dpi_for_window) = DPI_FUNCTIONS.get_dpi_for_window {
        return unsafe { get_dpi_for_window(hwnd) };
    }
    if let Some(get_dpi_for_monitor) = DPI_FUNCTIONS.get_dpi_for_monitor {
        let monitor = unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTOPRIMARY) };
        let mut dpi_x = 0u32;
        let mut dpi_y = 0u32;
        if unsafe {
            get_dpi_for_monitor(
                monitor,
                MDT_EFFECTIVE_DPI,
                &mut dpi_x as *mut _,
                &mut dpi_y as *mut _,
            )
        } == S_OK
        {
            return dpi_x;
        }
    }
    unsafe {
        let hdc = GetDC(hwnd);
        let dpi = GetDeviceCaps(hdc, LOGPIXELSX);
        ReleaseDC(hwnd, hdc);
        dpi as u32
    }
}

fn read_stream_fully_with<F: FnMut(&[u8]) -> bool>(
    stream: &IStream,
    mut fun: F,
) -> windows::core::Result<()> {
    let mut buf: [u8; 256 * 1024] = [0; 256 * 1024];
    loop {
        let mut num_read: u32 = 0;
        let res = unsafe {
            stream.Read(
                buf.as_mut_ptr() as *mut _,
                buf.len() as u32,
                Some(&mut num_read as *mut _),
            )
        };
        if res.is_err() {
            return Err(res.into());
        }

        if num_read == 0 {
            break;
        }
        if !fun(&buf[..num_read as usize]) {
            break;
        }
    }
    Ok(())
}

pub fn read_stream_fully(stream: &IStream) -> windows::core::Result<Vec<u8>> {
    let mut res = Vec::<u8>::new();
    read_stream_fully_with(stream, |b| {
        res.extend_from_slice(b);
        true
    })?;
    Ok(res)
}

pub fn copy_stream_to_file(stream: &IStream, path: &Path) -> NativeExtensionsResult<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;

    let mut res = Ok(());

    read_stream_fully_with(stream, |b| {
        let write_res = file.write_all(b);
        match write_res {
            Ok(_) => true,
            Err(err) => {
                res = Err(err.into());
                false
            }
        }
    })?;

    res
}

/// Check if format is available with fallback to Outlook formats
pub fn has_format_with_outlook_fallback(data_object: &IDataObject, format: u32) -> bool {
    // First try the requested format
    if query_format_with_fallback_safe(data_object, format) {
        return true;
    }
    
    // If it's CF_HDROP and not available, try Outlook-specific formats
    if format == CF_HDROP.0 as u32 {
        log::debug!("CF_HDROP not available, trying Outlook formats");
        
        // Try FileGroupDescriptor format
        if let Ok(fd_format) = unsafe { RegisterClipboardFormatA(CF_FILEDESCRIPTOR.as_ptr() as _) } {
            if query_format_with_fallback_safe(data_object, fd_format) {
                log::debug!("Found FileGroupDescriptor format as fallback for CF_HDROP");
                return true;
            }
        }
        
        // Try FileContents format
        if let Ok(fc_format) = unsafe { RegisterClipboardFormatA(CF_FILECONTENTS.as_ptr() as _) } {
            if query_format_with_fallback_safe(data_object, fc_format) {
                log::debug!("Found FileContents format as fallback for CF_HDROP");
                return true;
            }
        }
    }
    
    false
}

/// Extract data with fallback to Outlook formats
pub fn get_data_with_outlook_fallback(data_object: &IDataObject, format: u32) -> NativeExtensionsResult<Option<Vec<u8>>> {
    // First try the requested format
    let format_etc = make_format_with_tymed(format, TYMED(TYMED_ISTREAM.0 | TYMED_HGLOBAL.0));
    
    match safe_get_data(data_object, &format_etc) {
        Ok(Some(medium)) => {
            // Successfully got data, convert to Vec<u8>
            let res = unsafe {
                let mut medium = medium;
                let result = if medium.tymed == TYMED_ISTREAM.0 as u32 {
                    let stream = medium.u.pstm.as_ref().cloned();
                    if let Some(stream) = stream {
                        stream.Seek(0, STREAM_SEEK_SET, None)?;
                        read_stream_fully(&stream)
                    } else {
                        Ok(Vec::new())
                    }
                } else if medium.tymed == TYMED_HGLOBAL.0 as u32 {
                    let size = GlobalSize(medium.u.hGlobal);
                    let data = GlobalLock(medium.u.hGlobal);
                    let v = slice::from_raw_parts(data as *const u8, size);
                    let res: Vec<u8> = v.into();
                    GlobalUnlock(medium.u.hGlobal).ok();
                    Ok(res)
                } else {
                    Err(windows::core::Error::from(DV_E_FORMATETC))
                };
                ReleaseStgMedium(&mut medium as *mut STGMEDIUM);
                result
            };
            
            match res {
                Ok(data) => return Ok(Some(data)),
                Err(e) => {
                    log::debug!("Failed to extract data from medium: {}", e);
                }
            }
        }
        Ok(None) => {
            log::debug!("Format {} not available, trying fallbacks", format);
        }
        Err(e) => {
            log::debug!("Error getting format {}: {}", format, e);
        }
    }
    
    // If CF_HDROP failed, try Outlook-specific formats
    if format == CF_HDROP.0 as u32 {
        log::debug!("CF_HDROP not available, trying Outlook fallback formats");
        
        // Try FileGroupDescriptor + FileContents combination
        if let Ok(fd_format) = unsafe { RegisterClipboardFormatA(CF_FILEDESCRIPTOR.as_ptr() as _) } {
            let fd_format_etc = make_format_with_tymed(fd_format, TYMED(TYMED_ISTREAM.0 | TYMED_HGLOBAL.0));
            
            if let Ok(Some(_descriptor_data)) = safe_get_data(data_object, &fd_format_etc) {
                log::debug!("Found FileGroupDescriptor, this is an Outlook drag with virtual files");
                // For now, return empty data to indicate format is available but with no files
                // The actual file handling will be done by the FileContents format
                return Ok(Some(Vec::new()));
            }
        }
    }
    
    Ok(None)
}
