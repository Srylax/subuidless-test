use anyhow::{anyhow, Result};
use nix::fcntl::AtFlags;
use nix::sys::stat::{fstatat, FileStat};
use nix::unistd::{fchownat, Gid, Uid};
use serde::{Deserialize, Serialize, Serializer};
use std::os::fd::RawFd;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Syscalls {
    Fstatat {
        dirfd: Option<RawFd>,
        pathname: PathBuf,
        f: i32,
    },
    Fchownat {
        dirfd: Option<RawFd>,
        path: PathBuf,
        owner: Option<u32>,
        group: Option<u32>,
        flag: i32,
    },
}

impl Syscalls {
    /// Executes the Syscall by mapping the given options to the `nix` crate.
    /// If the Syscall has a return value it gets serialized with `serde_json` and returned as a JSON String
    pub fn execute(self) -> Result<Option<String>> {
        Ok(match self {
            Syscalls::Fstatat { dirfd, pathname, f } => {
                let fstat = fstatat(
                    dirfd,
                    &pathname,
                    AtFlags::from_bits(f).ok_or(anyhow!("Unknown Bits set"))?,
                )?;
                Some(serde_json::to_string(&FileStatDef::from(fstat))?)
            }
            Syscalls::Fchownat {
                dirfd,
                path,
                owner,
                group,
                flag,
            } => {
                fchownat(
                    dirfd,
                    &path,
                    owner.map(Uid::from_raw),
                    group.map(Gid::from_raw),
                    AtFlags::from_bits(flag).ok_or(anyhow!("Unknown Bits set"))?,
                )?;
                None
            }
        })
    }
}

/// Copy of `nix` `FileStat` for serde
#[derive(Debug, Serialize, Deserialize)]
pub struct FileStatDef {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u32,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i32,
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
