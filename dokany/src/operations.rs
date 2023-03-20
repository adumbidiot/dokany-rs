use crate::sys;
use crate::GlobalContext;

/// Function trampolines
pub(crate) static OPERATIONS: sys::DOKAN_OPERATIONS = sys::DOKAN_OPERATIONS {
    Mounted: Some(mounted_callback),
    ..sys::DOKAN_OPERATIONS::new()
};

unsafe extern "stdcall" fn mounted_callback(
    _mount_point: sys::LPCWSTR,
    dokan_file_info: sys::PDOKAN_FILE_INFO,
) -> sys::NTSTATUS {
    let result = std::panic::catch_unwind(|| {
        debug_assert!(!dokan_file_info.is_null());
        let options = (*dokan_file_info).DokanOptions;
        debug_assert!(!options.is_null());
        let options = &*options;
        debug_assert!(options.GlobalContext != 0);
        let global_context = &mut *(options.GlobalContext as *mut GlobalContext);

        global_context.filesystem.mounted()
    });

    match result {
        Ok(code) => code,
        Err(_e) => {
            // TODO: abort causes STATUS_STACK_BUFFER_OVERRUN.
            // std::process::abort();

            sys::STATUS_INTERNAL_ERROR
        }
    }

    /*
    let mount_point = slice_from_c_wstr_ptr(mount_point);
    */
}
