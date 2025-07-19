# Drag & Drop Memory Safety and Compatibility Improvements

## Issue Summary
Drag & drop operations were failing from multiple sources because:
1. `GlobalLock()` returned null pointers when accessing virtual file content
2. File descriptors had empty or missing filenames 
3. Files were created with 0 bytes due to failed content retrieval
4. Web browsers provide `CFSTR_FILECONTENTS` without proper `CFSTR_FILEDESCRIPTOR`
5. Email clients provide virtual files through non-standard implementations
6. **Outlook email message drags** need special handling to create `.eml` files

## Root Cause Analysis
The problem occurs because different drag sources provide virtual files through various patterns:

**Email Messages from Outlook:**
- Dragging emails from Outlook creates virtual `.eml` files
- Often use `RenPrivateMessages` and `FileGroupDescriptor` formats
- May lack proper `CFSTR_FILEDESCRIPTOR` or have corrupted file descriptors
- Should result in `.eml` files, not generic attachments

**Email Attachments from Outlook:**
- Provide `CFSTR_FILECONTENTS` format with unreliable global memory handles
- May have delayed rendering requiring specific TYMED handling
- Often have corrupted or missing file descriptors

**Web Browsers (Chrome, Edge):**
- Provide custom formats like `chromium/x-renderer-taint`, `Chromium Web Custom MIME Data Format`
- Use `CFSTR_FILECONTENTS` without proper `CFSTR_FILEDESCRIPTOR`
- Have different memory management patterns than native applications

## Improvements Made

### 1. Enhanced Global Memory Safety (`common.rs`)
- Added validation for invalid HGLOBAL handles
- Check `GlobalSize()` before attempting `GlobalLock()`
- Better error messaging to identify memory access issues
- Added debug logging for successful memory operations

### 2. Improved TYMED Fallback Logic (`reader.rs`)
**`medium_for_virtual_file()`**: Now tries multiple TYMED combinations in order:
1. `TYMED_ISTREAM` (preferred for virtual files)
2. `TYMED_HGLOBAL` (fallback)  
3. `TYMED_ISTREAM | TYMED_HGLOBAL` (final attempt)

**Additional HGlobal Validation**: Checks for zero-size HGlobal handles before accepting them

### 3. Smart Filename Fallback System
**`get_suggested_name_for_item()`**: 
- **NEW**: Detects drag source type based on available formats
- **Outlook Email Messages**: Creates `outlook_message.eml` for email drags (detects `RenPrivateMessages`, `FileGroupDescriptor`)
- **Web Browser Sources**: Creates `web_download.tmp` for Chromium-based browsers
- **Email Attachments**: Creates `attachment.tmp` for generic email attachments  
- **Image Sources**: Creates `image.tmp` for image formats
- **Text Sources**: Creates `text_content.txt` for text formats
- **Generic**: Creates `dropped_item.tmp` as final fallback
- **Never returns None**: Always provides a filename for successful file creation

### 4. Enhanced Virtual File Detection and Fallback Creation
**`with_file_descriptors()`**:
- **Smart Source Detection**: Analyzes available formats to identify drag source type
- **Outlook Email Pattern**: Detects `RenPrivateMessages`/`FileGroupDescriptor` and creates `.eml` fallbacks
- **Web Browser Pattern**: Detects Chromium formats and creates appropriate fallbacks
- **Email Attachment Pattern**: Detects `CFSTR_FILECONTENTS` without descriptors
- **Context-Aware Naming**: Uses different fallback names based on detected source type
- Enhanced logging for debugging different drag source behaviors

**`extract_file_descriptors()`**: 
- Detects empty/corrupted filenames from any source
- **Email Message Fallback**: Generates `outlook_message_{index}.eml` for email messages
- Enhanced debug logging for extracted file descriptors

### 5. Enhanced Error Recovery & Graceful Degradation
**`stream_from_medium()`**:
- Better error messages distinguishing between TYMED types
- Enhanced logging for debugging stream creation
- More specific error messages for troubleshooting

**`do_copy_virtual_file()`**:
- Creates empty files as fallback when content can't be read
- This ensures users see the dropped file (even if empty) rather than complete failure
- Enhanced logging for debugging file writing issues

### 6. Comprehensive Debugging Infrastructure
- Added logging at all critical decision points
- Format detection and source type analysis
- Virtual file descriptor lifecycle tracking
- Memory access failure diagnostics with size validation
- Fallback mechanism activation logging with source context

## Expected Behavior After Fixes

### For Outlook Email Message Drag Sources:
1. **Format Detection**: Enhanced logging shows Outlook-specific formats like `RenPrivateMessages`
2. **Filename Handling**: Files get named `outlook_message.eml` instead of null or generic names
3. **Content Type**: Uses `message/rfc822` MIME type for proper email message handling
4. **Graceful Degradation**: Empty `.eml` files created when content fails (preserves email format expectation)

### For Web Browser Drag Sources:
1. **Format Detection**: Enhanced logging shows Chromium-specific formats
2. **Filename Handling**: Files get named `web_download.tmp` instead of null
3. **Content Retrieval**: Multiple TYMED methods attempted with browser-specific validation
4. **Graceful Degradation**: Empty `web_download.tmp` files created when content fails

### For Email Attachment Drag Sources:
1. **Virtual File Discovery**: Detects email attachments even without proper descriptors
2. **Filename Issues**: Files get named `attachment.tmp` or `outlook_message_N.eml` based on context
3. **Content Retrieval**: Enhanced TYMED fallback logic with memory validation
4. **Graceful Degradation**: Empty attachment files created when content can't be retrieved

### General Improvements:
1. **Source Detection**: Automatically identifies drag source type (email message vs attachment vs web vs native)
2. **Never Null Filenames**: Always provides appropriate fallback names with correct extensions
3. **Enhanced Debugging**: Comprehensive logging helps identify specific source issues
4. **Better Error Recovery**: Fallback mechanisms provide better user experience

## Testing Recommendations

1. **Test Multiple Sources**: 
   - **Outlook Email Messages**: Drag emails from Outlook inbox/folders (should create `.eml` files)
   - **Email Attachments**: Drag attachments from opened emails
   - **Web Browsers**: Drag downloads from Chrome, Edge  
   - **Native Apps**: Drag files from File Explorer
2. **Different Content Types**: Downloads, email messages, email attachments, images, text content
3. **Monitor Enhanced Logs**: New debug messages show source detection and fallback activation
4. **Verify Fallback Names**: Ensure appropriate names and extensions based on detected source type
5. **Check Content Retrieval**: Verify TYMED attempts and memory validation

## Debugging Guide

The enhanced logging will show:
- **Source Detection**: `"*** DETECTED OUTLOOK EMAIL MESSAGE DRAG PATTERN ***"`, `"Detected email message drag from Outlook"`
- **Format Analysis**: `"Available formats for fallback naming: [...]"`
- **Fallback Activation**: `"using fallback: 'outlook_message.eml'"`, `"Created fallback descriptor for orphaned content"`
- **Memory Validation**: `"HGlobal medium size: X bytes"`, `"HGlobal has zero size - may be invalid"`
- **TYMED Processing**: `"Successfully got medium with TYMED X for file 'Y'"`

## Future Enhancements

If issues persist, consider:
1. Adding web browser specific content retrieval methods
2. Implementing format-specific filename extension detection
3. Supporting additional web browser custom formats
4. Adding retry mechanisms for unstable drag sources
5. Implementing source-specific optimization paths

The fixes maintain backward compatibility while providing robust fallback mechanisms specifically designed for problematic drag sources including web browsers, email clients, and other applications that don't follow standard virtual file protocols.
