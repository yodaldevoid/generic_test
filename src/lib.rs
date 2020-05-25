#![feature(const_generics)]
#![allow(incomplete_features)]

#[derive(Eq, PartialEq)]
pub enum PortName {
    A,
}

pub struct Port<const N: PortName>;

impl<const N: PortName> Port<N> {
    pub fn new() -> Port<N> {
        Port
    }

    pub fn pin<const P: usize>(&self) -> Pin<N, P> {
        Pin { _port: self }
    }
}

pub struct Pin<'a, const N: PortName, const P: usize> {
    _port: &'a Port<N>,
}

pub struct AdcPin<'a, const N: PortName, const P: usize> {
    _pin: Pin<'a, N, P>,
}

pub trait ToAdcPin<'a, const N: PortName, const P: usize> {
    fn to_adc(self) -> AdcPin<'a, N, P>;
}

impl<'a> ToAdcPin<'a, {PortName::A}, 0> for Pin<'a, {PortName::A}, 0> {
    fn to_adc(self) -> AdcPin<'a, {PortName::A}, 0> {
        AdcPin { _pin: self }
    }
}

impl<'a> ToAdcPin<'a, {PortName::A}, 1> for Pin<'a, {PortName::A}, 1> {
    fn to_adc(self) -> AdcPin<'a, {PortName::A}, 1> {
        AdcPin { _pin: self }
    }
}
