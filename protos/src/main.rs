use std::{env, path::Path, fs};

fn main() -> std::io::Result<()> {
    let current_dir = env::current_dir().unwrap();
    println!("{:?}", current_dir);
    let base_path = Path::new(&current_dir).join("./protos");

    let mut proto_files = vec![];
    let mut proto_paths = vec![];
    proto_paths.push(base_path.clone());

    for entry in fs::read_dir(&base_path).unwrap() {
        let entry = entry.unwrap();
        let md = entry.metadata().unwrap();
        if md.is_dir() {
            proto_paths.push(entry.path());
            continue;
        }
    }

    for proto_path in proto_paths {
        for entry in fs::read_dir(&proto_path).unwrap() {
            let entry = entry.unwrap();
            if entry.metadata().unwrap().is_file() && entry.path().extension().unwrap() == "proto" {
                proto_files.push(entry.path().as_os_str().to_os_string())
            }
        }
    }

    tonic_build::configure()
        .out_dir("../harpiya-domain/src")
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_client(false)
        .build_server(true)
        .compile(
            proto_files.as_slice(),
            &[&base_path],
        )
        .unwrap();

    Ok(())
}
