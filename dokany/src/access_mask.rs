use crate::sys;

bitflags::bitflags! {
    /// A value that defines standard, specific, and generic rights.
    #[repr(transparent)]
    #[derive(Debug)]
    pub struct AccessMask: sys::FILE_ACCESS_FLAGS {
        /// For a directory, the right to create a file in the directory.
        const FILE_ADD_FILE = sys::FILE_ADD_FILE;
        /// For a directory, the right to create a subdirectory.
        const FILE_ADD_SUBDIRECTORY = sys::FILE_ADD_SUBDIRECTORY;
        /// All possible access rights for a file.
        const FILE_ALL_ACCESS = sys::FILE_ALL_ACCESS;
        /// The right to read extended file attributes.
        const FILE_READ_EA = sys::FILE_READ_EA;
        /// For a file object, the right to append data to the file.
        ///
        /// (For local files, write operations will not overwrite existing data if this flag is specified without FILE_WRITE_DATA.)
        /// For a directory object, the right to create a subdirectory (FILE_ADD_SUBDIRECTORY).
        const FILE_APPEND_DATA = sys::FILE_APPEND_DATA;
        /// For a named pipe, the right to create a pipe.
        const FILE_CREATE_PIPE_INSTANCE = sys::FILE_CREATE_PIPE_INSTANCE;
        /// For a directory, the right to delete a directory and all the files it contains, including read-only files.
        const FILE_DELETE_CHILD = sys::FILE_DELETE_CHILD;
        /// For a native code file, the right to execute the file.
        ///
        /// This access right given to scripts may cause the script to be executable, depending on the script interpreter.
        const FILE_EXECUTE = sys::FILE_EXECUTE;
        /// For a directory, the right to list the contents of the directory.
        const FILE_LIST_DIRECTORY = sys::FILE_LIST_DIRECTORY;
        /// The right to read file attributes.
        const FILE_READ_ATTRIBUTES = sys::FILE_READ_ATTRIBUTES;
        /// For a file object, the right to read the corresponding file data.
        ///
        /// For a directory object, the right to read the corresponding directory data.
        const FILE_READ_DATA = sys::FILE_READ_DATA;
        /// For a directory, the right to traverse the directory.
        ///
        /// By default, users are assigned the BYPASS_TRAVERSE_CHECKING privilege,
        /// which ignores the FILE_TRAVERSE access right.
        /// See the remarks in File Security and Access Rights for more information.
        const FILE_TRAVERSE = sys::FILE_TRAVERSE;
        /// The right to write file attributes.
        const FILE_WRITE_ATTRIBUTES = sys::FILE_WRITE_ATTRIBUTES;
        /// For a file object, the right to write data to the file.
        /// For a directory object, the right to create a file in the directory (FILE_ADD_FILE).
        const FILE_WRITE_DATA = sys::FILE_WRITE_DATA;
        ///The right to write extended file attributes.
        const FILE_WRITE_EA = sys::FILE_WRITE_EA;
        /// Access system security (ACCESS_SYSTEM_SECURITY).
        ///
        /// It is used to indicate access to a system access control list (SACL).
        /// This type of access requires the calling process to have the SE_SECURITY_NAME (Manage auditing and security log) privilege.
        /// If this flag is set in the access mask of an audit access ACE (successful or unsuccessful access), the SACL access will be audited.
        const ACCESS_SYSTEM_SECURITY = sys::ACCESS_SYSTEM_SECURITY;
        /// Maximum allowed
        const MAXIMUM_ALLOWED = sys::MAXIMUM_ALLOWED;
        /// ?
        const GENERIC_ALL = sys::GENERIC_ALL;
        /// ?
        const GENERIC_EXECUTE = sys::GENERIC_EXECUTE;
        /// ?
        const GENERIC_WRITE = sys::GENERIC_WRITE;
        /// ?
        const GENERIC_READ = sys::GENERIC_READ;
        /// Delete access.
        const DELETE = sys::DELETE;
        /// Read access to the owner, group, and discretionary access control list (DACL) of the security descriptor.
        const READ_CONTROL = sys::READ_CONTROL;
        /// Write access to the DACL.
        const WRITE_DAC = sys::WRITE_DAC;
        /// Write access to owner.
        const WRITE_OWNER = sys::WRITE_OWNER;
        /// Synchronize access.
        const SYNCHRONIZE = sys::SYNCHRONIZE;
        /// ?
        const STANDARD_RIGHTS_REQUIRED = sys::STANDARD_RIGHTS_REQUIRED;
        /// ?
        const STANDARD_RIGHTS_READ = sys::STANDARD_RIGHTS_READ;
        /// ?
        const STANDARD_RIGHTS_WRITE = sys::STANDARD_RIGHTS_WRITE;
        /// ?
        const STANDARD_RIGHTS_EXECUTE = sys::STANDARD_RIGHTS_EXECUTE;
        /// ?
        const STANDARD_RIGHTS_ALL = sys::STANDARD_RIGHTS_ALL;
        /// ?
        const SPECIFIC_RIGHTS_ALL = sys::SPECIFIC_RIGHTS_ALL;
    }
}
