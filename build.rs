use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!(
        "cargo:rustc-link-search=/home/arch/inspireface-rs/InspireFace/build/inspireface-linux/InspireFace/lib"
    );
    println!("cargo:rustc-link-lib=dylib=InspireFace");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=bz2");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("/home/arch/inspireface-rs/InspireFace/build/inspireface-linux/InspireFace/include/herror.h")
        .header("/home/arch/inspireface-rs/InspireFace/build/inspireface-linux/InspireFace/include/intypedef.h")
        .header("/home/arch/inspireface-rs/InspireFace/build/inspireface-linux/InspireFace/include/inspireface.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src/");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
