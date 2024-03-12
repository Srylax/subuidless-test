use docker_command::{BaseCommand, BuildOpt, Launcher, RunOpt};
use protocol::FileStatDef;
use std::path::{Path, PathBuf};
#[test]
fn build_docker() -> anyhow::Result<()> {
    Launcher::from(BaseCommand::Docker)
        .build(BuildOpt {
            build_args: vec![],
            context: Path::new("../").into(),
            dockerfile: Some(Path::new("Dockerfile").into()),
            tag: Some("subuidless/executor".into()),
            ..Default::default()
        })
        .run()?;
    Ok(())
}
#[test]
fn lstatat() -> anyhow::Result<()> {
    let fstat = protocol::Syscalls::Fstatat {
        dirfd: None,
        pathname: PathBuf::from("/dev"),
        f: 0,
    };
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
    assert_eq!(fstat.st_dev, 95);
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
