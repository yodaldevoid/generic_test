#![feature(const_generics)]
#![allow(incomplete_features)]

use generic_test::*;

fn main() {
    let pin_a: Pin<{PortName::A}> = Pin;
    pin_a.do_thing();
}
