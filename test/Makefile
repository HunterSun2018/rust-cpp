
shared-release : test.cpp ffi
	c++ -O3 -std=c++17 test.cpp -o bin/test -Lbin -llibcpp -lffi -lpthread -ldl -Wl,-rpath=.,-rpath=bin
	strip bin/*

release :
	c++ -std=c++17 -O3 test.cpp -o bin/test -L../target/release/ -l:liblibcpp.a -lpthread -ldl
	strip bin/*

FFI_PATH = ../target/release/build/libcpp-1979f3aff13e3e3c/out/rust_cpp/cpp_closures.o

ffi : $(FFI_PATH)
	mkdir -p bin
	c++ -O2 -shared $(FFI_PATH) -o bin/libffi.so
	cp ../target/release/liblibcpp.so bin/ 

clean : 
	rm -rf bin