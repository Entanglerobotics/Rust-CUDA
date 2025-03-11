use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    // Set the directory where OptiX is installed
    let optix_dir = "/opt/optix"; // Change to actual path

    // Define the output path early
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_path.join("bindings.rs");

    // Generate bindings first and write to file immediately
    let bindings_result = bindgen::Builder::default()
        .header(format!("{}/include/optix_stubs.h", optix_dir))
        .clang_arg(format!("-I{}/include", optix_dir))
        .allowlist_function("optix.*")
        .allowlist_type("Optix.*")
        .allowlist_var("OPTIX.*")
        .wrap_unsafe_ops(true)
        .wrap_static_fns(true)
        .generate();

    // Write to the file regardless of later steps
    match bindings_result {
        Ok(bindings) => {
            bindings
                .write_to_file(&bindings_path)
                .expect("Couldn't write bindings!");
            println!("Bindings written to: {:?}", bindings_path);
        }
        Err(e) => {
            // Log the error and continue
            eprintln!("Failed to generate bindings: {:?}", e);
            // Optionally, write an empty bindings file for inspection
            fs::write(&bindings_path, "// Failed to generate bindings").expect("Couldn't write placeholder file!");
        }
    }

    // Link the OptiX library
    println!("cargo:rustc-link-lib=optix");
    println!("cargo:rustc-link-search=native={}/lib64", optix_dir);
}
