#ifndef ADDER_HPP
#define ADDER_HPP
#include <memory>
#include <array>

namespace test
{
    const size_t ADDRESS_LENGTH = 16;
    using Address = std::array<uint8_t, ADDRESS_LENGTH>;

    struct Account
    {
        uint64_t index;
        Address address;
    };

    class Adder
    {

    public:
        static std::shared_ptr<Adder>
        create();

        virtual ~Adder(){};

        virtual void
        add(int64_t value) = 0;

        virtual int64_t
        tell() const = 0;

        virtual int
        make() = 0;

        virtual Account
        create_account() = 0;
    };

} // namespace test

#endif