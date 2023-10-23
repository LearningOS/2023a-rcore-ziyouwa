use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let linker_script_path = PathBuf::from(manifest_dir).join("src/linker.ld");

    println!("cargo:rustc-link-arg=-T{}", linker_script_path.display());
}
