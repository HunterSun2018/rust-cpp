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
2. check folder "native=/home/hunter/Projects/Rust/rust-cpp/target/release/build/libcpp-1979f3aff13e3e3c/out/rust_cpp" -l static=rust_cpp_generated"
3. run command to generate libffi.so "c++ -shared /home/hunter/Projects/Rust/rust-cpp/target/release/build/libcpp-1979f3aff13e3e3c/out/rust_cpp/cpp_closures.o -o libffi.so"
4. link with libffi.so 