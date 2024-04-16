use copy_dir::copy_dir;
use docker_command::{BaseCommand, BuildOpt, Launcher};
use proc_macro::TokenStream;
use std::env;
use std::env::current_dir;
use std::fs::{create_dir_all, File, remove_dir};
use std::io::Write;
use std::path::PathBuf;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, LitStr, Token};

#[proc_macro]
pub fn create_docker(input: TokenStream) -> TokenStream {
    let Ok(tmpdir) = env::var("CARGO_TARGET_TMPDIR") else {
        return TokenStream::new();
    };

    let values = parse_macro_input!(input with Punctuated::<LitStr, Token![,]>::parse_terminated);

    let mut paths = values.iter().map(LitStr::value);

    let tmp_path = PathBuf::from(tmpdir);
    let mut dockerfile = File::create(tmp_path.join("Dockerfile")).expect("Could not open File");

    let bin_dir = paths.next().expect("No Binary dir specified");
    let copy_dirs = paths.clone().map(|path|format!("COPY {path} {path}")).collect::<Vec<String>>().join("\n");

    let docker_content = format!("
    FROM rust:slim-buster
    WORKDIR /usr/src/executor
    {copy_dirs}
    WORKDIR /usr/src/executor/{bin_dir}
    RUN cargo install --bin executor --path .");

    dockerfile
        .write_all(docker_content.as_bytes())
        .expect("Could not write Dockerfile");

    for path in paths {
        let tmp_path = tmp_path.join(&path);
        if tmp_path.is_dir() {
            let _ = std::fs::remove_dir_all(&tmp_path);
        } else {
            let _ = std::fs::remove_file(&tmp_path);
        }
        let path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Executor not found"))
            .parent()
            .expect("No Parent")
            .join(path);
        if path.is_dir() {
            create_dir_all(&tmp_path).expect("Could not create target dir");
            remove_dir(&tmp_path).expect(&format!("Could not remove target dir"));

            copy_dir(&path, &tmp_path).expect(&format!("Could not copy dir. CWD: {:?}, path: {:?}, tmp_path: {:?}", current_dir().expect("No current dir"),path,tmp_path));
        } else { 
            std::fs::copy(&path, &tmp_path).expect(&format!("Could not copy file. CWD: {:?}, path: {:?}, tmp_path: {:?}", current_dir().expect("No current dir"),path,tmp_path));
        }

    }

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
