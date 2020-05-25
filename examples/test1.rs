#![feature(const_generics)]
#![allow(incomplete_features)]

use generic_test::*;

fn main() {
    let mut sim = Sim::new();
    let port_a = sim.port::<{PortName::A}>();
    let pin_a0 = port_a.pin::<0>();
    let _adc = pin_a0.to_adc();
}
