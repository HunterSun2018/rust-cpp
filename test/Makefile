
test : test.cpp
	c++ -std=c++11 test.cpp -o test -L../target/debug/ -l:liblibcpp.a -lpthread -ldl

release :
	c++ -std=c++17 -O3 test.cpp -o test -L../target/release/ -l:liblibcpp.a -lpthread -ldl
	strip test
	
clean : 
	rm -f test libtest.so