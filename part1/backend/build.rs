use std::path::Path; 
use std::process::Command;

const FRONTEND_DIR: &str = "../ui";

fn main() {
    println!("cargo:rerun-if-changed={}/static", FRONTEND_DIR);
    println!("cargo:rerun-if-changed={}/src", FRONTEND_DIR);
    println!("cargo:rerun-if-changed={}/index.html", FRONTEND_DIR);
    println!("cargo:rerun-if-changed={}/index.scss", FRONTEND_DIR);
    
    build_frontend(FRONTEND_DIR);
}

fn build_frontend<P: AsRef<Path>>(source: P) {
    Command::new("trunk")
        .args(&["build", "--release"])
        .current_dir(source.as_ref())
        .status()
        .expect("Failed to build Frontend");
}