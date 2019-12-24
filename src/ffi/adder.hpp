#ifndef ADDER_HPP
#define ADDER_HPP

class Adder
{
    void *internal;

public:
    Adder();
    ~Adder();

    void add(int64_t value);
    int64_t tell() const;
};

#endif