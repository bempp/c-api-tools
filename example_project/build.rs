extern crate cbindgen;

use std::env;
use std::process::Command;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    Command::new("rustup")
        .current_dir(&crate_dir)
        .env_remove("RUSTC")
        .args([
            "run",
            "nightly",
            "cbindgen",
            "--config",
            "cbindgen.toml",
            "--output",
            "bindings.h",
        ])
        .output()
        .unwrap();
}
