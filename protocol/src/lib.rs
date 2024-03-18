pub mod executor;

use anyhow::Result;
use nix::sys::stat::FileStat;
use serde::{Deserialize, Serialize};

pub use docker_command;
use nix::libc::{blksize_t, nlink_t};
pub use protocol_proc;

#[typetag::serde(tag = "type")]
pub trait Syscall {
    fn execute(&self) -> Result<Option<String>>;
}

/// Copy of `nix` `FileStat` for serde
#[derive(Debug, Serialize, Deserialize)]
pub struct FileStatDef {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: nlink_t,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: blksize_t,
    pub st_blocks: i64,
    pub st_atime: i64,
    pub st_atime_nsec: i64,
    pub st_mtime: i64,
    pub st_mtime_nsec: i64,
    pub st_ctime: i64,
    pub st_ctime_nsec: i64,
}

impl From<FileStat> for FileStatDef {
    fn from(value: FileStat) -> Self {
        Self {
            st_dev: value.st_dev,
            st_ino: value.st_ino,
            st_nlink: value.st_nlink,
            st_mode: value.st_mode,
            st_uid: value.st_uid,
            st_gid: value.st_gid,
            st_rdev: value.st_rdev,
            st_size: value.st_size,
            st_blksize: value.st_blksize,
            st_blocks: value.st_blocks,
            st_atime: value.st_atime,
            st_atime_nsec: value.st_atime_nsec,
            st_mtime: value.st_mtime,
            st_mtime_nsec: value.st_mtime_nsec,
            st_ctime: value.st_ctime,
            st_ctime_nsec: value.st_ctime_nsec,
        }
    }
}
