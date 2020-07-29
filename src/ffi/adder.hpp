#ifndef ADDER_HPP
#define ADDER_HPP
#include <memory>

class Adder
{
    void *internal;

public:
    Adder();
    ~Adder();

    void add(int64_t value);
    int64_t tell() const;
};


//std::shared_ptr<Adder> create_adder();

#endif