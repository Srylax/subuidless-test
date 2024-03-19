use serde::Serialize;
#[macro_export]
macro_rules! executor {
    () => {
        protocol_proc::create_docker!();
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

#[macro_export]
macro_rules! docker_test {
    (
        $struct_name:ident {
            $(
             $field_name:ident : $field_type:ty
            ),*
        },
        $self:ident $syscall:block
    ) => {
        #[derive(Serialize, Deserialize)]
        pub struct $struct_name {
            $(
                $field_name : $field_type,
            )*
        }
        #[typetag::serde]
        impl protocol::Syscall for $struct_name {
            fn execute(&$self) -> anyhow::Result<Option<String>> {
                Ok(Some(serde_json::to_string(&$syscall)?))
            }
        }
    };
}
