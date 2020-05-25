#![feature(const_generics)]
#![allow(incomplete_features)]

#[derive(Eq, PartialEq)]
pub enum PortName {
    A,
}

pub struct Pin<const N: PortName, const P: usize>;

pub struct AdcPin<const N: PortName, const P: usize> {
    _pin: Pin<N, P>,
}

pub trait ToAdcPin<const N: PortName, const P: usize> {
    fn to_adc(self) -> AdcPin<N, P>;
}

impl ToAdcPin<{PortName::A}, 0> for Pin<{PortName::A}, 0> {
    fn to_adc(self) -> AdcPin<{PortName::A}, 0> {
        AdcPin { _pin: self }
    }
}

impl ToAdcPin<{PortName::A}, 1> for Pin<{PortName::A}, 1> {
    fn to_adc(self) -> AdcPin<{PortName::A}, 1> {
        AdcPin { _pin: self }
    }
}
