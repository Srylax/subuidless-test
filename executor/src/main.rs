use anyhow::Result;
use std::env;
use std::io::{stdout, Write};

/// Parses the first argument as a `protocol::Syscalls` and executes the given Syscall
/// Return Values get written to stdout
fn main() -> Result<()> {
    let args = env::args().nth(1).expect("No Argument provided");
    let syscall: protocol::Syscalls = serde_json::from_str(&args)?; // Deserialize to Syscall
    if let Some(str) = syscall.execute()? {
        // Execute Syscall
        stdout().write_all(str.as_ref())?; // Write Response to stdout
    }
    Ok(())
}
