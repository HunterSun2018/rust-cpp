use cpp_build;

fn main() {
    let include_path = "src/ffi";
    cpp_build::Config::new()
        .include(include_path)
        .build("src/lib.rs");

    //cpp_build::build("src/ffi/mod.rs");
}
