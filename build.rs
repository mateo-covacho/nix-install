use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let cargo_home = env::var("CARGO_HOME").unwrap_or_else(|_| {
        let home = env::var("HOME").expect("HOME environment variable is not set");
        format!("{}/.cargo", home)
    });

    let bin_dir = Path::new(&cargo_home).join("bin");

    fs::create_dir_all(&bin_dir).expect("Failed to create bin directory");

    let script_path = bin_dir.join("nix-install");
    fs::copy("scripts/nix-search-install.sh", &script_path).expect("Failed to copy script");
}
