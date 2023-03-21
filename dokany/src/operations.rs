use crate::sys;
use crate::GlobalContext;
use crate::WriteWideCStringCell;
use std::mem::MaybeUninit;

/// Function trampolines
pub(crate) static OPERATIONS: sys::DOKAN_OPERATIONS = sys::DOKAN_OPERATIONS {
    ZwCreateFile: Some(create_file_callback),
    GetVolumeInformation: Some(get_volume_information_callback),
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

unsafe extern "stdcall" fn create_file_callback(
    file_name: sys::LPCWSTR,
    _security_context: sys::PDOKAN_IO_SECURITY_CONTEXT,
    _desired_access: sys::ACCESS_MASK,
    _file_attributes: sys::ULONG,
    _share_access: sys::ULONG,
    _create_disposition: sys::ULONG,
    _create_options: sys::ULONG,
    dokan_file_info: sys::PDOKAN_FILE_INFO,
) -> sys::NTSTATUS {
    let result = std::panic::catch_unwind(|| {
        let global_context = extract_global_context(dokan_file_info);
        let file_name = slice_from_c_wstr_ptr(file_name);

        global_context.filesystem.create_file(file_name)
    });

    match result {
        Ok(code) => code,
        Err(_e) => sys::STATUS_INTERNAL_ERROR,
    }
}

unsafe extern "stdcall" fn get_volume_information_callback(
    volume_name_buffer: sys::LPWSTR,
    volume_name_size: sys::DWORD,
    volume_serial_number: sys::LPDWORD,
    maximum_component_length: sys::LPDWORD,
    file_system_flags: sys::LPDWORD,
    file_system_name_buffer: sys::LPWSTR,
    file_system_name_size: sys::DWORD,
    dokan_file_info: sys::PDOKAN_FILE_INFO,
) -> sys::NTSTATUS {
    let result = std::panic::catch_unwind(|| {
        let global_context = extract_global_context(dokan_file_info);

        let volume_name = std::slice::from_raw_parts_mut(
            volume_name_buffer.cast::<MaybeUninit<u16>>(),
            volume_name_size.try_into().unwrap(),
        );
        volume_name[0].write(0);
        let volume_name = WriteWideCStringCell::new(volume_name);

        // Defaults chosen from memfs example
        volume_serial_number.write(0x19831116);
        maximum_component_length.write(255);

        file_system_flags.write(0);

        let file_system_name = std::slice::from_raw_parts_mut(
            file_system_name_buffer.cast::<MaybeUninit<u16>>(),
            file_system_name_size.try_into().unwrap(),
        );
        file_system_name[0].write(0);
        let file_system_name = WriteWideCStringCell::new(file_system_name);

        global_context.filesystem.get_volume_information(
            volume_name,
            &mut *volume_serial_number,
            &mut *maximum_component_length,
            &mut *file_system_flags.cast(),
            file_system_name,
        )
    });

    match result {
        Ok(code) => code,
        Err(_e) => sys::STATUS_INTERNAL_ERROR,
    }
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
