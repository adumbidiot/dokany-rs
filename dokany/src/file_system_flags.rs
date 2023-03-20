use crate::sys;

bitflags::bitflags! {
    /// Capability flags for a filesystem.
    ///
    /// FILE_FILE_COMPRESSION and FILE_VOL_IS_COMPRESSED are mutually exclusive
    #[repr(transparent)]
    pub struct FileSystemFlags: sys::DWORD {
        /// The specified volume supports case-sensitive file names.
        const CASE_SENSITIVE_SEARCH = sys::FILE_CASE_SENSITIVE_SEARCH;
        /// The specified volume supports preserved case of file names when it places a name on disk.
        const CASE_PRESERVED_NAMES = sys::FILE_CASE_PRESERVED_NAMES;
        /// The specified volume supports Unicode in file names as they appear on disk.
        const UNICODE_ON_DISK = sys::FILE_UNICODE_ON_DISK;
        /// The specified volume preserves and enforces access control lists (ACL).
        ///
        /// For example, the NTFS file system preserves and enforces ACLs, and the FAT file system does not.
        const PERSISTENT_ACLS = sys::FILE_PERSISTENT_ACLS;
        /// The specified volume supports file-based compression.
        const FILE_COMPRESSION = sys::FILE_FILE_COMPRESSION;
        /// The specified volume supports disk quotas.
        const VOLUME_QUOTAS = sys::FILE_VOLUME_QUOTAS;
        /// The specified volume supports sparse files.
        const SUPPORTS_SPARSE_FILES = sys::FILE_SUPPORTS_SPARSE_FILES;
        /// The specified volume supports reparse points.
        const SUPPORTS_REPARSE_POINTS = sys::FILE_SUPPORTS_REPARSE_POINTS;
        /// ?
        const SUPPORTS_REMOTE_STORAGE = sys::FILE_SUPPORTS_REMOTE_STORAGE;
        /// ?
        const RETURNS_CLEANUP_RESULT_INFO = sys::FILE_RETURNS_CLEANUP_RESULT_INFO;
        /// ?
        const SUPPORTS_POSIX_UNLINK_RENAME = sys::FILE_SUPPORTS_POSIX_UNLINK_RENAME;
        /// The specified volume is a compressed volume, for example, a DoubleSpace volume.
        const VOLUME_IS_COMPRESSED = sys::FILE_VOLUME_IS_COMPRESSED;
        /// The specified volume supports object identifiers.
        const SUPPORTS_OBJECT_IDS = sys::FILE_SUPPORTS_OBJECT_IDS;
        /// The specified volume supports the Encrypted File System (EFS). For more information, see File Encryption.
        const SUPPORTS_ENCRYPTION = sys::FILE_SUPPORTS_ENCRYPTION;
        /// The specified volume supports named streams.
        const NAMED_STREAMS = sys::FILE_NAMED_STREAMS;
        /// The specified volume is read-only.
        const READ_ONLY_VOLUME = sys::FILE_READ_ONLY_VOLUME;
        /// The specified volume supports a single sequential write.
        const SEQUENTIAL_WRITE_ONCE = sys::FILE_SEQUENTIAL_WRITE_ONCE;
        /// The specified volume supports transactions. For more information, see About KTM.
        const SUPPORTS_TRANSACTIONS = sys::FILE_SUPPORTS_TRANSACTIONS;
        /// The specified volume supports hard links. For more information, see Hard Links and Junctions.
        const FILE_SUPPORTS_HARD_LINKS = sys::FILE_SUPPORTS_HARD_LINKS;
        /// The specified volume supports extended attributes.
        ///
        /// An extended attribute is a piece of application-specific metadata that an application can associate with a file and is not part of the file's data.
        const SUPPORTS_EXTENDED_ATTRIBUTES = sys::FILE_SUPPORTS_EXTENDED_ATTRIBUTES;
        /// The file system supports open by FileID. For more information, see FILE_ID_BOTH_DIR_INFO.
        const SUPPORTS_OPEN_BY_FILE_ID = sys::FILE_SUPPORTS_OPEN_BY_FILE_ID;
        /// The specified volume supports update sequence number (USN) journals. For more information, see Change Journal Records.
        const SUPPORTS_USN_JOURNAL = sys::FILE_SUPPORTS_USN_JOURNAL;
        /// ?
        const SUPPORTS_INTEGRITY_STREAMS = sys::FILE_SUPPORTS_INTEGRITY_STREAMS;
        /// The specified volume supports sharing logical clusters between files on the same volume.
        ///
        /// The file system reallocates on writes to shared clusters.
        /// Indicates that FSCTL_DUPLICATE_EXTENTS_TO_FILE is a supported operation.
        const SUPPORTS_BLOCK_REFCOUNTING = sys::FILE_SUPPORTS_BLOCK_REFCOUNTING;
        /// ?
        const SUPPORTS_SPARSE_VDL = sys::FILE_SUPPORTS_SPARSE_VDL;
        /// The specified volume is a direct access (DAX) volume.
        const DAX_VOLUME = sys::FILE_DAX_VOLUME;
        /// ?
        const SUPPORTS_GHOSTING = sys::FILE_SUPPORTS_GHOSTING;
    }
}
