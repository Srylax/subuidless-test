use copy_dir::copy_dir;
use docker_command::{BaseCommand, BuildOpt, Launcher};
use proc_macro::TokenStream;
use std::env;
use std::fs::File;
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

    let paths = values.iter().map(|lit_str| lit_str.value());

    let tmp_path = PathBuf::from(tmpdir);
    let mut dockerfile = File::create(tmp_path.join("Dockerfile")).expect("Could not open File");
    dockerfile
        .write_all(include_bytes!("../Dockerfile"))
        .expect("Could not write Dockerfile");

    for path in paths {
        let _ = std::fs::remove_dir_all(tmp_path.join(&path));
        copy_dir(
            PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Executor not found"))
                .parent()
                .expect("No Parent")
                .join(&path),
            tmp_path.join(&path),
        )
        .expect("Could not copy protocol dir");
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
