use crate::sys;
use crate::OptionFlags;

/// Options for a filesystem
pub struct Options {
    /// The ffi options struct
    pub(crate) options: sys::DOKAN_OPTIONS,
}

impl Options {
    /// Make an empty set of options
    pub fn new() -> Self {
        Self {
            options: sys::DOKAN_OPTIONS::new(),
        }
    }

    /// Set the requested version
    pub fn set_version(&mut self, version: u16) {
        self.options.Version = version;
    }

    /// Set the mount point
    pub fn set_mount_point(&mut self, mount_point: &'static [u16]) {
        self.options.MountPoint = mount_point.as_ptr();
    }

    /// Set the option flags
    pub fn set_option_flags(&mut self, flags: OptionFlags) {
        self.options.Options = flags.bits();
    }
}

impl Default for Options {
    fn default() -> Self {
        Self::new()
    }
}
