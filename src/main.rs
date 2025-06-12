use std::fs;
use std::path::Path;
use std::process::Command;

#[tokio::main]
async fn main() {
    let filename = format!("lib{}.so", env!("CARGO_PKG_NAME"));

    let plugin_path = Path::new("./target/").join(env!("BUILD_PROFILE"));
    let dest_path = Path::new("./run/plugins/");

    fs::create_dir_all(dest_path).unwrap();

    fs::copy(plugin_path.join(&filename), dest_path.join(&filename))
        .expect("failed to copy plugin");

    let status = Command::new("./pumpkin")
        .current_dir("./run")
        .status()
        .expect("failed to run main project");

    assert!(status.success());
}
