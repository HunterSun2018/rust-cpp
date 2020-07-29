
shared : test.cpp ffi
	c++ -std=c++17 test.cpp -o test -L../target/release/ -llibcpp -L. -lffi -lpthread -ldl
	strip *.so test

release :
	c++ -std=c++17 -O3 test.cpp -o test -L../target/release/ -l:liblibcpp.a -lpthread -ldl
	strip test

ffi : /home/hunter/Projects/Rust/rust-cpp/target/release/build/libcpp-1979f3aff13e3e3c/out/rust_cpp/cpp_closures.o
	c++ -shared /home/hunter/Projects/Rust/rust-cpp/target/release/build/libcpp-1979f3aff13e3e3c/out/rust_cpp/cpp_closures.o -o libffi.so
	cp ../target/release/liblibcpp.so ./ 

clean : 
	rm -f test libtest.so libffi.so liblibcpp.so