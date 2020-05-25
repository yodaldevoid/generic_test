#![feature(const_generics)]
#![allow(incomplete_features)]

use generic_test::*;

fn main() {
    let pin_a: Pin<{PortName::A}> = Pin;
    let _adc = pin_a.to_adc();
}
