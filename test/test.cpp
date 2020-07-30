#include <iostream>
#include "../src/ffi/adder.hpp"

using namespace std;

int main(int argc, char * argv[])
{
    using namespace test;

    auto adder = Adder::create();

    adder->add(10);

    auto account = adder->create_account();

    cout << "adder tell me : " << adder->tell() << endl;

    return 0;
}
