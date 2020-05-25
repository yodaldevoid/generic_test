#![feature(const_generics)]
#![allow(incomplete_features)]

#[derive(Eq, PartialEq)]
pub enum PortName {
    A,
}

pub struct Pin<const N: PortName>;

pub trait DoThing {
    fn do_thing(self);
}

impl DoThing for Pin<{PortName::A}> {
    fn do_thing(self) {}
}
