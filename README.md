# rust-cpp
Export C++ API form a Rust lib 

# Build
cd rust-cpp

cargo build --release

cd test

make release

# Run
cd test

./test 

# how to link wtih shared library
1. run "cargo build --relesae -v"
2. check if file "../target/release/build/libcpp-1979f3aff13e3e3c/out/rust_cpp/cpp_closures.o" exists
3. link file cpp_closures.o into c++ program