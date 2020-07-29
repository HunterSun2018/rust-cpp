//use cpp_build;
extern crate cpp_build;

fn main() {
    let include_path = "src/ffi";
    cpp_build::Config::new()
        .include(include_path)
        .flag("-std=c++17")
        .flag_if_supported("shared")
        .pic(true)
        .build("src/ffi/mod.rs");

    //cpp_build::build("src/ffi/mod.rs");
}
