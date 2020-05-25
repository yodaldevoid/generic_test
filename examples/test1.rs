#![feature(const_generics)]
#![allow(incomplete_features)]

use generic_test::*;

fn main() {
    let pin_a0: Pin<{PortName::A}, 0> = Pin;
    let _adc = pin_a0.to_adc();
}
