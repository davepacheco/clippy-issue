use std::fmt;

pub struct Foo {}

impl fmt::Debug for Foo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "test\n")
    }
}
