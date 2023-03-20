use crate::sys;

/// Function trampolines
pub(crate) static OPERATIONS: sys::DOKAN_OPERATIONS = sys::DOKAN_OPERATIONS {
    ..sys::DOKAN_OPERATIONS::new()
};
