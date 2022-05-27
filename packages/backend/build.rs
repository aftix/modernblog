use dotenv::dotenv;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    dotenv().ok();

    let api_path = env::var("FRONTEND_PATH").unwrap_or_else(|_| String::from("localhost:8080"));
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("api.rs");
    fs::write(
        &dest_path,
        &format!("const FRONTEND_PATH: &str = \"{}\";", api_path),
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=API_PATH");
}
