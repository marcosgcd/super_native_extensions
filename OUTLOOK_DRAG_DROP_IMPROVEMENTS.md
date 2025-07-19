## Summary of Changes Made for Outlook Email Drag and Drop

I've made comprehensive improvements to handle Outlook email drag and drop when using modern Outlook with web rendering:

### Key Issues Identified
1. **Modern Outlook Web Rendering**: Outlook now uses Chromium-based web rendering internally, exposing different clipboard formats than traditional COM-based formats
2. **Missing CFSTR_FILECONTENTS**: The traditional `CFSTR_FILECONTENTS` format is not available 
3. **Web Browser Formats**: Instead of email formats, we see formats like `"DragContext", "DragImageBits", "chromium/x-renderer-taint", "NativeShell_CF_15", "Chromium Web Custom MIME Data Format"`

### Major Improvements Made

#### 1. Enhanced Outlook Detection (`probably_outlook_message`)
- Added detection for modern Outlook web rendering patterns
- Look for combinations of `NativeShell_CF_15` + `Chromium Web Custom MIME Data Format` + `DragContext`
- Added content analysis to detect actual email data in web formats

#### 2. Content Extraction (`try_extract_outlook_web_content`)
- Added ability to extract text/HTML content from web clipboard formats
- Create proper EML files from extracted content
- Support for both HTML and plain text email content
- Fallback to minimal EML if no content can be extracted

#### 3. Enhanced Format Detection (`get_formats_for_item_sync`)
- Automatically add email formats (`message/rfc822`, `application/vnd.ms-outlook`) for detected Outlook content
- Add text formats when text content is available
- Prioritize synthesized email formats

#### 4. Improved File Naming
- Changed from `.msg` to `.eml` for better cross-platform compatibility
- EML format handles HTML content better than MSG
- Enhanced fallback naming logic

#### 5. Virtual File Handling
- Enhanced `can_copy_virtual_file_for_item` to handle synthesized email files
- Updated `copy_virtual_file_for_item` to create email files from web content
- Added `copy_outlook_web_email_file` method for web-to-file conversion

### Expected Behavior Changes

When dragging an email from modern Outlook:

1. **Detection**: The code will now detect the drag as an Outlook email based on the web rendering patterns
2. **Format Support**: Available formats will include `message/rfc822` and `text/html` even if not natively present
3. **Content Extraction**: The code will attempt to extract actual email content (HTML or text) from the web formats
4. **File Creation**: An `.eml` file will be created with the extracted email content
5. **Fallback**: If no content can be extracted, a minimal EML file will be created to indicate the email source

### Testing Instructions

1. Rebuild the Rust library: `cargo build` in the `rust/` directory
2. Rebuild the Flutter app
3. Try dragging an email from Outlook desktop application
4. Check the logs for detection messages starting with "current version 5"
5. Verify that an `.eml` file is created with email content

The logs should now show:
- "Detected Outlook message: traditional=false, modern_web=true, web_with_content=true"
- "Found X bytes of text/HTML content for email"
- "Successfully extracted email content from Outlook web formats"
