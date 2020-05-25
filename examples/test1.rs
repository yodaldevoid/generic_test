#![feature(const_generics)]
#![allow(incomplete_features)]

use generic_test::*;

fn main() {
    let port_a: Port<{PortName::A}> = Port::new();
    let pin_a0 = port_a.pin::<0>();
    let _adc = pin_a0.to_adc();
}
