use cpp::cpp;
use crate::adder::Adder;

cpp! {{
#include "adder.hpp"

Adder::Adder() {
    this->internal =
        rust!(Adder_constructor [] -> *mut Adder as "void *" {
            let b = Box::new(Adder::default());
            Box::into_raw(b)
        });
}

Adder::~Adder() {
    rust!(Adder_destructor [internal: *mut Adder as "void *"] {
        let _b = unsafe {
            Box::from_raw(internal)
        };
    });
}

void Adder::add(int64_t value) {
    rust!(Adder_add [
        internal: &mut Adder as "void *",
        value: i64 as "int64_t"
    ] {
        internal.add(value);
    });
}

int64_t Adder::tell() const {
    return rust!(Adder_tell [
        internal: &mut Adder as "void *"
    ] -> i64 as "int64_t" {
        internal.tell()
    });
}

std::shared_ptr<Adder> create_adder()
{
    return std::make_shared<Adder>();
}

}}
