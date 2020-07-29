CXX_FLAGS = -O3 -std=c++17
CPP_FFI = ../target/release/build/libcpp-1979f3aff13e3e3c/out/rust_cpp/cpp_closures.o
RUST_LIB_PATH = ../target/release/

shared-release : test.cpp $(CPP_FFI)
	c++ test.cpp $(CPP_FFI) -o test $(CXX_FLAGS) -L$(RUST_LIB_PATH) -llibcpp -lpthread -ldl -Wl,-rpath=.
	cp $(RUST_LIB_PATH)/liblibcpp.so .
	strip test liblibcpp.so

release : test.cpp
	c++ test.cpp -o test $(CXX_FLAGS) -L$(RUST_LIB_PATH) -l:liblibcpp.a -lpthread -ldl
	strip test

clean : 
	rm -rf test liblibcpp.so