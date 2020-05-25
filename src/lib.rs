#![feature(const_generics)]
#![allow(incomplete_features)]

#[derive(Eq, PartialEq)]
pub enum PortName {
    A,
}

pub struct Pin<const N: PortName>;

pub struct AdcPin<const N: PortName> {
    _pin: Pin<N>,
}

pub trait ToAdcPin<const N: PortName> {
    fn to_adc(self) -> AdcPin<N>;
}

impl ToAdcPin<{PortName::A}> for Pin<{PortName::A}> {
    fn to_adc(self) -> AdcPin<{PortName::A}> {
        AdcPin { _pin: self }
    }
}
