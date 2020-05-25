#![feature(const_generics)]
#![allow(incomplete_features)]

#[derive(Eq, PartialEq)]
pub enum PortName {
    A,
}

pub struct Pin<const N: PortName>;

pub trait DoThing<const N: PortName> {
    fn do_thing(self);
}

impl DoThing<{PortName::A}> for Pin<{PortName::A}> {
    fn do_thing(self) {}
}
