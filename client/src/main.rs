use anyhow::Result;
use docker_command::{BaseCommand, Launcher, RunOpt};
use protocol::{FileStatDef, Syscall};
use std::path::Path;

#[macro_use]
extern crate timeit;

fn main() -> Result<()> {
    // let fstat = protocol::Syscalls::Fstatat {
    //     dirfd: None,
    //     pathname: PathBuf::from("/dev"),
    //     f: 0,
    // };
    // let args_string = serde_json::to_string(&fstat)?;
    // let a: &dyn Syscall = &client::Fstatat {};
    // let args_string = serde_json::to_string(a)?;
    // let syscall: Box<dyn Syscall> = serde_json::from_str(&args_string)?; // Deserialize to Syscall
    //
    // let command = Launcher::from(BaseCommand::Docker)
    //     .run(RunOpt {
    //         image: "subuidless/executor:latest".to_string(),
    //         remove: true,
    //         command: Some(Path::new("executor").into()),
    //         args: vec![args_string.into()],
    //         ..Default::default()
    //     })
    //     .enable_capture()
    //     .run()?;
    //
    // let fstat: FileStatDef = serde_json::from_slice(command.stdout.as_slice())?;
    // println!("{:?}", fstat);
    Ok(())
}
