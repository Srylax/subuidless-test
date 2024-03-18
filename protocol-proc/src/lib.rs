use copy_dir::copy_dir;
use docker_command::{BaseCommand, BuildOpt, Launcher};
use proc_macro::TokenStream;
use std::env;
use std::fs::{copy, File};
use std::io::Write;
use std::path::PathBuf;

#[proc_macro]
pub fn create_docker(_: TokenStream) -> TokenStream {
    let Ok(tmpdir) = env::var("CARGO_TARGET_TMPDIR") else {
        return TokenStream::new();
    };

    // panic!("{:#?}", env::vars());

    let tmp_path = PathBuf::from(tmpdir);
    let mut dockerfile = File::create(tmp_path.join("Dockerfile")).expect("Could not open File");
    dockerfile
        .write_all(include_bytes!("../Dockerfile"))
        .expect("Could not write Dockerfile");

    let _ = std::fs::remove_dir_all(tmp_path.join("client"));
    copy_dir(
        env::var("CARGO_MANIFEST_DIR").expect("Executor not found"),
        tmp_path.join("client"),
    )
    .expect("Could not copy dir");

    //TODO: Remove

    let _ = std::fs::remove_dir_all(tmp_path.join("protocol"));
    copy_dir(
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Executor not found"))
            .parent()
            .expect("No Parent")
            .join("protocol"),
        tmp_path.join("protocol"),
    )
    .expect("Could not copy protocol dir");

    let _ = std::fs::remove_dir_all(tmp_path.join("protocol-proc"));
    copy_dir(
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Executor not found"))
            .parent()
            .expect("No Parent")
            .join("protocol-proc"),
        tmp_path.join("protocol-proc"),
    )
    .expect("Could not copy protocol dir");

    Launcher::from(BaseCommand::Docker)
        .build(BuildOpt {
            build_args: vec![],
            context: tmp_path.clone(),
            dockerfile: Some(tmp_path.join("Dockerfile")),
            tag: Some("subuidless/executor".into()),
            ..Default::default()
        })
        .run()
        .expect("Could not build docker image");

    TokenStream::new()
}
