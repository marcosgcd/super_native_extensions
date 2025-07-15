# Handling DV_E_FORMATETC in Drag-and-Drop

## Problem
DV_E_FORMATETC (0x80040064) errors were causing noisy logging and failures in drag-and-drop operations, particularly with Outlook email drags. The error occurs when:
1. IDataObject::GetData is called with unsupported format/medium combinations
2. IDataObject::EnumFormatEtc fails on certain data sources
3. Format enumeration returns formats that aren't actually available

## Solution Approach

### 1. Safe COM Call Wrappers
- **safe_get_data()**: Always calls QueryGetData before GetData to prevent DV_E_FORMATETC
- **safe_enum_format_etc()**: Gracefully handles enumeration failures with fallback formats
- **Error Handling**: DV_E_FORMATETC is treated as expected behavior, not an error

### 2. Outlook-Specific Handling
- **Detection**: `is_outlook_email_drag()` identifies Outlook drag operations
- **Fallback Formats**: When CF_HDROP fails, try FileGroupDescriptor + FileContents
- **Empty Responses**: Return empty format lists instead of failing

### 3. Logging Improvements
- **Debug Level**: DV_E_FORMATETC errors logged at debug level only
- **Contextual Info**: Include format IDs and TYMED values in trace logs
- **Graceful Degradation**: Continue operation even when formats fail

## Implementation Details

### Safe Wrapper Pattern
```rust
pub fn safe_get_data(obj: &IDataObject, fmt: &FORMATETC) -> NativeExtensionsResult<Option<STGMEDIUM>> {
    unsafe {
        // Check availability first
        if obj.QueryGetData(fmt).is_err() {
            return Ok(None);
        }
        
        // Only call GetData if format is supported
        match obj.GetData(fmt) {
            Ok(medium) => Ok(Some(medium)),
            Err(e) if e.code().0 == 0x80040064 => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
```

### Outlook Format Detection
```rust
pub fn is_outlook_email_drag(data_object: &IDataObject) -> bool {
    // Check for Outlook-specific formats
    let msg_fmt = RegisterClipboardFormatA(b"RenPrivateMessages\0");
    let desc_fmt = RegisterClipboardFormatA(b"FileGroupDescriptor\0");
    
    query_format_with_fallback_safe(data_object, msg_fmt) || 
    query_format_with_fallback_safe(data_object, desc_fmt)
}
```

### Error Level Guidelines
- **Debug**: DV_E_FORMATETC, expected format unavailability
- **Info**: Outlook drag detection, fallback format usage
- **Warn**: Unexpected COM errors, configuration issues
- **Error**: Critical failures that prevent operation

## Testing Strategy

1. **Regression Testing**: Verify Explorer → App file drops still work
2. **Outlook Testing**: Confirm Outlook → App drags don't spam logs
3. **Local Drag Testing**: Ensure App → App dragging remains functional
4. **Error Injection**: Test with malformed data objects

## Benefits

- **Reduced Log Noise**: No more DV_E_FORMATETC spam in logs
- **Better Compatibility**: Improved support for various drag sources
- **Graceful Degradation**: Operations continue even when formats fail
- **Maintainability**: Centralized error handling and logging
