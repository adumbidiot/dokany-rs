use crate::sys;
use crate::GlobalContext;

/// Function trampolines
pub(crate) static OPERATIONS: sys::DOKAN_OPERATIONS = sys::DOKAN_OPERATIONS {
    Mounted: Some(mounted_callback),
    Unmounted: Some(unmounted_callback),
    ..sys::DOKAN_OPERATIONS::new()
};

unsafe fn slice_from_c_wstr_ptr<'a>(ptr: *const u16) -> &'a [u16] {
    let len = libc::wcslen(ptr);
    std::slice::from_raw_parts(ptr, len)
}

unsafe fn extract_global_context<'a>(
    dokan_file_info: sys::PDOKAN_FILE_INFO,
) -> &'a mut GlobalContext {
    debug_assert!(!dokan_file_info.is_null());
    let options = (*dokan_file_info).DokanOptions;
    debug_assert!(!options.is_null());
    let options = &*options;
    debug_assert!(options.GlobalContext != 0);
    &mut *(options.GlobalContext as *mut GlobalContext)
}

unsafe extern "stdcall" fn mounted_callback(
    mount_point: sys::LPCWSTR,
    dokan_file_info: sys::PDOKAN_FILE_INFO,
) -> sys::NTSTATUS {
    let result = std::panic::catch_unwind(|| {
        let global_context = extract_global_context(dokan_file_info);
        let mount_point = slice_from_c_wstr_ptr(mount_point);

        global_context.filesystem.mounted(mount_point)
    });

    match result {
        Ok(code) => code,
        Err(_e) => sys::STATUS_INTERNAL_ERROR,
    }
}

unsafe extern "stdcall" fn unmounted_callback(
    dokan_file_info: sys::PDOKAN_FILE_INFO,
) -> sys::NTSTATUS {
    let result = std::panic::catch_unwind(|| {
        let global_context = extract_global_context(dokan_file_info);

        global_context.filesystem.unmounted()
    });

    match result {
        Ok(code) => code,
        Err(_e) => sys::STATUS_INTERNAL_ERROR,
    }
}
