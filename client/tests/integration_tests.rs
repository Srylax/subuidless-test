use docker_command::{BaseCommand, Launcher, RunOpt};
use std::path::{Path, PathBuf};

use nix::fcntl::AtFlags;
use nix::sys::stat::fstatat;
use protocol::{FileStatDef, Syscall};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Fstatat;
#[typetag::serde]
impl protocol::Syscall for Fstatat {
    fn execute(&self) -> anyhow::Result<Option<String>> {
        let stat = fstatat(None, "/dev", AtFlags::empty())?;
        let stat = FileStatDef::from(stat);
        Ok(Some(serde_json::to_string(&stat)?))
    }
}

#[test]
fn lstatat() -> anyhow::Result<()> {
    let fstat: &dyn Syscall = &Fstatat {};
    let args_string = serde_json::to_string(&fstat)?;

    let command = Launcher::from(BaseCommand::Docker)
        .run(RunOpt {
            image: "subuidless/executor:latest".to_string(),
            remove: true,
            command: Some(Path::new("executor").into()),
            args: vec![args_string.into()],
            ..Default::default()
        })
        .enable_capture()
        .run()?;

    let fstat: FileStatDef = serde_json::from_slice(command.stdout.as_slice())?;
    assert_eq!(fstat.st_ino, 1);
    assert_eq!(fstat.st_nlink, 5);
    assert_eq!(fstat.st_mode, 16877);
    assert_eq!(fstat.st_uid, 0);
    assert_eq!(fstat.st_gid, 0);
    assert_eq!(fstat.st_rdev, 0);
    assert_eq!(fstat.st_size, 340);
    assert_eq!(fstat.st_blocks, 0);
    Ok(())
}
