use std::fmt;

//
/// `Messaging` - Allows direct, and multi-user encrypted messaging with ownership rights added so only
///             the expected users can edit, and delete messages.
///
/// `FileSystem` - Facilitates the creation of files and folders within a central directory tree (Index).
///              This index is managed internally and traversal of the directory as well as full listings,
///              deletion, and creation is provided within this module. Additionally uploading files to the filesystem.
///
/// `Accounts` - Creates a unique user accounts used to store core information about the user.
///            This can include simple things like usernames and status messages, but may also
///            include permissions, friends, and more.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Module {
    Messaging,
    FileSystem,
    Accounts,
    Unknown,
}


impl fmt::Display for Module {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        Module::Messaging => write!(f, "MESSAGING"),
        Module::FileSystem => write!(f, "FILESYSTEM"),
        Module::Accounts => write!(f, "ACCOUNTS"),
        Module::Unknown => write!(f, "UNKNOWN"),
      }
  }
}