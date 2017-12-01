use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // NOTE: this runs _before_ the build! So we are copying outdated stuff.
    // Only run when targeting wasm
    if env::var("CARGO_CFG_target_arch").unwrap() != "wasm32" {
        return;
    }

    // Copy the resulting wasm to the html folder
    let html_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("html");
    let target_wasm = Path::new(&env::var("OUT_DIR").unwrap()).join("../../../rocket.wasm");
    let copied_wasm = html_path.join("rocket.wasm");
    fs::copy(target_wasm, &copied_wasm).expect("Failed to copy");

    // Run wasm-gc
    Command::new("wasm-gc").arg(copied_wasm).arg(html_path.join("program.wasm"))
        .spawn().expect("wasm_gc failed to start")
        .wait().expect("wasm_gc crashed");
}
