#include <iostream>

class Adder
{
    void *internal;

public:
    Adder();

    ~Adder();

    void add(int64_t value);

    int64_t tell() const;
};

Adder::Adder()
{
}
Adder::~Adder()
{
}

void Adder::add(int64_t value)
{
}

int64_t Adder::tell() const
{
    return 0;
}