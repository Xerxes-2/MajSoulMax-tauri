use std::env;
use std::io::Result;
use std::path::Path;

fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();
    let manifest = env::var("CARGO_MANIFEST_DIR").unwrap();
    std::process::Command::new("go")
        .args(["install", "github.com/EndlessCheng/mahjong-helper@latest"])
        .env("GOBIN", &out_dir)
        .status()?;
    std::fs::copy(
        Path::new(&out_dir).join("mahjong-helper"),
        Path::new(&manifest).join(format!("binaries/mahjong-helper-{}", target)),
    )?;
    tauri_build::build();
    Ok(())
}
