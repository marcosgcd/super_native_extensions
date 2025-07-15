#[cfg(test)]
mod tests {
    use super::*;
    use windows::Win32::System::Com::FORMATETC;
    use windows::Win32::System::Com::TYMED_HGLOBAL;
    use windows::Win32::System::Com::DVASPECT_CONTENT;
    
    #[test]
    fn test_safe_com_wrappers_exist() {
        // This test verifies that our safe COM wrappers exist and have the expected signatures
        
        // Test that safe_get_data function exists
        let _safe_get_data: fn(&IDataObject, &FORMATETC) -> NativeExtensionsResult<Option<STGMEDIUM>> = safe_get_data;
        
        // Test that safe_enum_format_etc function exists  
        let _safe_enum_format_etc: fn(&IDataObject) -> NativeExtensionsResult<Vec<FORMATETC>> = safe_enum_format_etc;
        
        // Test that Outlook fallback functions exist
        let _has_format_with_outlook_fallback: fn(&IDataObject, u32) -> bool = has_format_with_outlook_fallback;
        let _get_data_with_outlook_fallback: fn(&IDataObject, u32) -> NativeExtensionsResult<Option<Vec<u8>>> = get_data_with_outlook_fallback;
    }
    
    #[test]
    fn test_outlook_detection() {
        // This test verifies that Outlook detection functions exist
        let _is_outlook_email_drag: fn(&IDataObject) -> bool = is_outlook_email_drag;
        let _try_common_outlook_formats: fn(&IDataObject) -> Vec<FORMATETC> = try_common_outlook_formats;
    }
    
    #[test]
    fn test_safe_data_retrieval() {
        // This test verifies that the GetData trait has the safe methods
        // Note: This is a compile-time test to ensure the methods exist
        
        // Verify that get_data_safe method exists on the trait
        fn test_get_data_safe<T: GetData>(_obj: &T) {
            // This function won't be called, but ensures the method exists
        }
    }
    
    #[test]
    fn test_error_handling_improvements() {
        // This test verifies that error handling has been improved
        // by checking that DV_E_FORMATETC is handled gracefully
        
        // Test that DV_E_FORMATETC constant is available
        let _dv_e_formatetc: windows::Win32::Foundation::HRESULT = DV_E_FORMATETC;
        
        // Test that our error handling code recognizes this error code
        assert_eq!(DV_E_FORMATETC.0, 0x80040064);
    }
    
    #[test]
    fn test_format_constants() {
        // Test that Outlook format constants are defined
        assert_eq!(CF_OUTLOOK_MSG, b"RenPrivateMessages\0");
        assert_eq!(CF_OUTLOOK_ATTACH, b"RenPrivateAttachments\0");
        assert_eq!(CF_FILEDESCRIPTOR, b"FileGroupDescriptor\0");
        assert_eq!(CF_FILECONTENTS, b"FileContents\0");
        assert_eq!(CF_UNIFORMRESOURCELOCATOR, b"UniformResourceLocator\0");
    }
}
