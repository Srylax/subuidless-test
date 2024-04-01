
use std::ffi::{OsString};
use anyhow::Result;
use nix::sys::stat::FileStat;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub use docker_command;
use docker_command::command_run::Output;
use docker_command::{BaseCommand, Launcher, RunOpt};
use nix::libc::{blksize_t, nlink_t};
pub use protocol_proc;

#[macro_export]
macro_rules! syscall {
    (
        $struct_name:ident {
            $(
             $(#[$field_meta:meta])*
             $field_name:ident : $field_type:ty
            ),*
        },
        $self:ident $syscall:block,

        $test_name:ident($struct_value:ident, ($left:ident,$right:ident): $de_type:ident) $compare:block
    ) => {

        #[derive(Debug, Arbitrary, Serialize, Deserialize, PartialEq)]
        struct $struct_name {
            $(
                $(#[$field_meta])*
                $field_name : $field_type,
            )*
        }

        // Implementation des Syscall Traits
        #[typetag::serde]
        impl protocol::Syscall for $struct_name {
            fn execute(&$self) -> anyhow::Result<Option<String>> {
                Ok(Some(serde_json::to_string(&$syscall)?))
            }
        }

        proptest! {
            #[test]
            fn $test_name($struct_value: $struct_name) {
                // Arrange
                let syscall: &dyn Syscall = &$struct_value;
                let args_string = serde_json::to_string(&syscall).expect("Could not serialize");

                let mut args:Vec<std::ffi::OsString> = option_env!("SUBUIDLESS_ARGS")
                    .map(|args|serde_json::from_str(args)
                    .expect("Invalid Docker Arguments set"))
                    .unwrap_or_default();

                args.push((&args_string).into());

                // Act
                let left = exec_docker(args);
                let right = exec_docker(vec![args_string.into()]);

                // Assert
                prop_assert_eq!(left.status, right.status);
                if left.status.success() {
                    let $left: $de_type = serde_json::from_slice(left.stdout.as_slice()).expect("Could not deserialize despite command success");
                    let $right: $de_type = serde_json::from_slice(right.stdout.as_slice()).expect("Could not deserialize despite command success");
                    $compare?;
                } else {
                    prop_assert_eq!(left, right);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! executor {
    () => {
        use protocol::Syscall;
        use std::env;
        use std::error::Error;
        use std::io::{stdout, Write};

        use protocol::protocol_proc;

        /// Parses the first argument as a `protocol::Syscalls` and executes the given Syscall
        /// Return Values get written to stdout
        fn main() -> Result<(), Box<dyn Error>> {
            let args = env::args().nth(1).expect("No Argument provided");
            let syscall: Box<dyn Syscall> = serde_json::from_str(&args)?; // Deserialize to Syscall
            if let Some(str) = syscall.execute()? {
                // Execute Syscall
                stdout().write_all(str.as_ref())?; // Write Response to stdout
            }
            Ok(())
        }
    };
}


pub fn exec_docker(args: Vec<OsString>) -> Output {
    Launcher::from(BaseCommand::Docker)
        .run(RunOpt {
            image: "subuidless/executor:latest".to_string(),
            remove: true,
            command: Some(Path::new("executor").into()),
            args,
            ..Default::default()
        })
        .enable_capture()
        .disable_check()
        .run()
        .expect("Could not run docker")
}

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
