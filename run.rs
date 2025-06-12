use std::process::Command;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
/*
    let status = Command::new("cargo")
        .args(&["build", "--release"])
        .status()
        .expect("failed to build plugin");
    assert!(status.success());
*/
    let filename = format!("lib{}.so", env!("CARGO_PKG_NAME"));

    let plugin_path = Path::new("./target/release/");
    let dest_path = Path::new("./run/plugins/");

    fs::create_dir_all(dest_path).unwrap();

    fs::copy(
        plugin_path.join(&filename),
        dest_path.join(&filename)
    ).expect("failed to copy plugin");

    let status = Command::new("./pumpkin")
        .current_dir("./run")
        .status()
        .expect("failed to run main project");

    assert!(status.success());
}

