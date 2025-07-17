# DV_E_FORMATETC Error Fix Summary

## Problem
The application was experiencing frequent Windows error "Ungültige FORMATETC-Struktur (0x80040064)" (DV_E_FORMATETC) during drag and drop operations. This error occurs when:

1. `IDataObject::GetData` is called with unsupported format/medium combinations
2. `IDataObject::EnumFormatEtc` fails on certain data sources
3. Format enumeration returns formats that aren't actually available

## Solution Implemented

### 1. Safe COM Call Wrappers (common.rs)
Added two new safe wrapper functions:

- **`safe_get_data()`**: Always calls `QueryGetData` before `GetData` to prevent DV_E_FORMATETC
- **`safe_enum_format_etc()`**: Gracefully handles enumeration failures with fallback to empty format list

### 2. Enhanced Error Handling in Data Retrieval (reader.rs)
Updated multiple functions to use safe wrappers:

- **`data_object_formats_raw()`**: Now uses `safe_enum_format_etc` instead of direct format enumeration
- **`with_hdrop()`**: Updated to use `safe_get_data` for CF_HDROP format retrieval
- **`with_file_descriptors()`**: Updated to use `safe_get_data` for CFSTR_FILEDESCRIPTOR format retrieval  
- **`medium_for_virtual_file()`**: Updated to use `safe_get_data` for CFSTR_FILECONTENTS format retrieval
- **`generate_png()`**: Updated to use `safe_get_data` for CF_DIB/CF_DIBV5 format retrieval
- **`get_data_for_item_sync()`**: Updated to use `safe_get_data` for general format retrieval

### 3. Improved Logging
- DV_E_FORMATETC errors are now logged at debug level instead of error/warn level
- Added debug logging for format enumeration results
- Graceful degradation when formats are unavailable

## Files Modified
- `src/win32/common.rs`: Added safe wrapper functions
- `src/win32/reader.rs`: Updated all data retrieval calls to use safe wrappers

## Expected Behavior
After these changes:
- DV_E_FORMATETC errors should no longer appear as ERROR logs
- The application should gracefully handle unsupported formats
- Drag and drop operations should work more reliably
- Standard file drops should continue to work normally
- Outlook drags should work without throwing errors

## Testing Recommendations
1. **Explorer → App**: Verify file drops still work with correct paths
2. **Outlook → App**: Confirm no error logs and graceful handling  
3. **App → App**: Verify local dragging functionality remains intact
4. **Various Sources**: Test with different drag sources to ensure compatibility
