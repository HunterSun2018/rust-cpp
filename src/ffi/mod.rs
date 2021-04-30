use crate::adder::Adder;
use cpp::cpp;

cpp! {{
#include "adder.hpp"

namespace test
{
    class AdderImpl : public Adder
    {
        void *internal;
    
    public:
        AdderImpl() {
            this->internal =
                rust!(Adder_constructor [] -> *mut Adder as "void *" {
                    let b = Box::new(Adder::default());
                    Box::into_raw(b)
                });
        }

        virtual ~AdderImpl()
        {
            rust!(Adder_destructor [internal: *mut Adder as "void *"] {
                let _b = unsafe {
                    Box::from_raw(internal)
                };
            });
        }

        virtual void
        add(int64_t value) override
        {
            rust!(Adder_add [
                internal: &mut Adder as "void *",
                value: i64 as "int64_t"
            ] {
                internal.add(value);
            });
        }

        virtual int64_t
        tell() const override
        {
            return rust!(Adder_tell [
                internal: &mut Adder as "void *"
            ] -> i64 as "int64_t" {
                internal.tell()
            });
        }

        virtual int 
        make() override
        {
            return 0;
        }

        virtual Account
        create_account() override
        {
            return Account();
        }

        Account create_account1()
        {
            return Account();
        }

    };

    std::shared_ptr<Adder> 
    Adder::create()
    {
        return std::make_shared<AdderImpl>();
    }
}

}}
