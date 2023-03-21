use std::ffi::OsStr;
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::path::PathBuf;

/// Implemented for types that can be converted into wide char arrays.
pub trait AsWide {
    type Iter: Iterator<Item = u16>;

    /// Get an iterator over wide chars.
    ///
    /// Depending on where this is passed,
    /// it is a logic error to have a NUL wide char.
    fn as_wide(&self) -> Self::Iter;
}

impl<'a> AsWide for &'a str {
    type Iter = std::str::EncodeUtf16<'a>;

    fn as_wide(&self) -> Self::Iter {
        self.encode_utf16()
    }
}

impl<'a> AsWide for &'a String {
    type Iter = std::str::EncodeUtf16<'a>;

    fn as_wide(&self) -> Self::Iter {
        self.as_str().encode_utf16()
    }
}

impl<'a> AsWide for &'a [u16] {
    type Iter = std::iter::Copied<std::slice::Iter<'a, u16>>;

    fn as_wide(&self) -> Self::Iter {
        self.iter().copied()
    }
}

impl<'a> AsWide for &'a Vec<u16> {
    type Iter = std::iter::Copied<std::slice::Iter<'a, u16>>;

    fn as_wide(&self) -> Self::Iter {
        self.iter().copied()
    }
}

impl<'a> AsWide for &'a OsStr {
    type Iter = std::os::windows::ffi::EncodeWide<'a>;

    fn as_wide(&self) -> Self::Iter {
        self.encode_wide()
    }
}

impl<'a> AsWide for &'a OsString {
    type Iter = std::os::windows::ffi::EncodeWide<'a>;

    fn as_wide(&self) -> Self::Iter {
        self.encode_wide()
    }
}

impl<'a> AsWide for &'a Path {
    type Iter = std::os::windows::ffi::EncodeWide<'a>;

    fn as_wide(&self) -> Self::Iter {
        self.as_os_str().encode_wide()
    }
}

impl<'a> AsWide for &'a PathBuf {
    type Iter = std::os::windows::ffi::EncodeWide<'a>;

    fn as_wide(&self) -> Self::Iter {
        self.as_os_str().encode_wide()
    }
}
