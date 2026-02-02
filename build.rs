//! Build script for kakebooR

fn main() {
    // Rust 2024 edition requires explicit check-cfg declarations
    println!("cargo::rustc-check-cfg=cfg(wasm)");
    println!("cargo::rustc-check-cfg=cfg(native)");
}
