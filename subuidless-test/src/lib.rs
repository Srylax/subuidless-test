
use std::ffi::{OsString};
use anyhow::Result;
use std::path::Path;

use docker_command::command_run::{ ErrorKind, Output};
use docker_command::{BaseCommand, Launcher, RunOpt};
pub use subuidless_test_proc::create_docker;

pub extern crate serde;
pub extern crate proptest_derive;
pub extern crate proptest;
pub extern crate docker_command;
pub extern crate typetag;
pub extern crate serde_json;
pub extern crate anyhow;

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

        $test_name:ident($struct_value:ident, ($left:ident,$right:ident): $de_type:ty) $compare:block
    ) => {

        #[derive(Debug, $crate::proptest_derive::Arbitrary, $crate::serde::Serialize, $crate::serde::Deserialize, PartialEq)]
        struct $struct_name {
            $(
                $(#[$field_meta])*
                $field_name : $field_type,
            )*
        }

        // Implementation des Syscall Traits
        #[$crate::typetag::serde]
        impl $crate::Syscall for $struct_name {
            fn execute(&$self) -> $crate::anyhow::Result<Option<String>> {
                Ok(Some($crate::serde_json::to_string(&$syscall)?))
            }
        }

        proptest! {
            #[test]
            #[ignore]
            fn $test_name($struct_value: $struct_name) {
                // Arrange
                let syscall: &dyn $crate::Syscall = &$struct_value;
                let args_string = $crate::serde_json::to_string(&syscall).expect("Could not serialize");

                let mut args: Vec<std::ffi::OsString> = option_env!("SUBUIDLESS_ARGS")
                    .map(|args|$crate::serde_json::from_str(args)
                    .expect("Invalid Docker Arguments set"))
                    .unwrap_or_default();

                args.push((&args_string).into());

                // Act
                let left = $crate::exec_docker(args).expect("Could not Execute Docker");
                let right = $crate::exec_docker(vec![args_string.into()]).expect("Could not Execute Docker");

                // Assert
                $crate::proptest::prop_assert_eq!(left.status, right.status);
                if left.status.success() {
                    let $left: $de_type = $crate::serde_json::from_slice(left.stdout.as_slice()).expect("Could not deserialize despite command success");
                    let $right: $de_type = $crate::serde_json::from_slice(right.stdout.as_slice()).expect("Could not deserialize despite command success");
                    $compare?;
                } else {
                    $crate::proptest::prop_assert_eq!(left, right);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! executor {
    () => {
        /// Parses the first argument as a `protocol::Syscalls` and executes the given Syscall
        /// Return Values get written to stdout
        fn main() -> Result<(), Box<dyn std::error::Error>> {
            let args = std::env::args().nth(1).expect("No Argument provided");
            let syscall: Box<dyn $crate::Syscall> = $crate::serde_json::from_str(&args)?; // Deserialize to Syscall
            if let Some(str) = syscall.execute()? {
                // Execute Syscall
                std::io::Write::write_all(&mut std::io::stdout(), str.as_ref())?;// Write Response to stdout
            }
            Ok(())
        }
    };
}


pub fn exec_docker(args: Vec<OsString>) -> Result<Output, ErrorKind> {
    Launcher::from(BaseCommand::Docker)
        .run(RunOpt {
            image: "subuidless/executor:latest".to_owned(),
            remove: true,
            command: Some(Path::new("executor").into()),
            ..Default::default()
        }).add_args(args)
        .enable_capture()
        .disable_check()
        .run()
        .map_err(|err|err.kind)
}

#[typetag::serde(tag = "type")]
pub trait Syscall {
    fn execute(&self) -> Result<Option<String>>;
}
