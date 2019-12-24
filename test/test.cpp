#include <iostream>
#include "../src/ffi/adder.hpp"

using namespace std;

int main(int argc, char * argv[])
{
    Adder adder;

    adder.add(10);

    cout << "adder tell me : " << adder.tell() << endl;

    return 0;
}
