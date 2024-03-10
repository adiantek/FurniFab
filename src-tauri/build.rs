use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_triple = std::env::var("TARGET")?;
    let path = format!("../lib/{target_triple}");
    let path = Path::new(&path).canonicalize()?;
    let path_str = path.to_str().ok_or("Cannot convert path to string")?;

    println!("cargo:rustc-link-search={path_str}");
    println!("cargo:rustc-link-lib=python3api");

    tauri_build::build();

    Ok(())
}
