use std::env;
use std::path::PathBuf;

fn main() {
    // Get the current directory
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let utils_dir = PathBuf::from(&manifest_dir).join("src/utils");

    // Create paths to your files
    let c_file = utils_dir.join("snow-3g.c");
    let h_file = utils_dir.join("snow-3g.h");

    // Tell Cargo to rerun the build script if these files change
    println!("cargo:rerun-if-changed={}", c_file.display());
    println!("cargo:rerun-if-changed={}", h_file.display());

    // Build the C file
    cc::Build::new()
        .file(c_file)
        .include(&utils_dir)
        .warnings(true)
        .opt_level(2)
        .define("u8", "uint8_t")
        .define("u32", "uint32_t")
        .define("u64", "uint64_t")
        .compile("snow3g");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        // The input header file
        .header(h_file.to_str().unwrap())
        // Tell bindgen where to find headers
        .clang_arg(format!("-I{}", utils_dir.display()))
        .clang_arg("-Du8=uint8_t")
        .clang_arg("-Du32=uint32_t")
        .clang_arg("-Du64=uint64_t")
        .generate_inline_functions(true)
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}