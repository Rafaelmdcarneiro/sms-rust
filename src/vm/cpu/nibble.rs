use num::One;
use num::Zero;
use std::ops::Add;
use std::ops::Mul;

#[derive(Copy, Clone, PartialEq)]
pub(crate) struct Nibble {
    value: u8,
}

impl Nibble {
    pub fn from_u8(value: u8) -> Nibble {
        Nibble {
            value: value & 0x0F,
        }
    }

    pub fn from_u8_high(value: u8) -> Nibble {
        Nibble::from_u8((value & 0xF0) >> 4)
    }

    pub fn u8_from_nibbles(high: Nibble, low: Nibble) -> u8 {
        (high.value << 4) | low.value
    }

    pub fn overflowing_add(self, rhs: Nibble) -> (Nibble, bool) {
        let ext_result = self.value + rhs.value;
        let result = Nibble::from_u8(ext_result);
        let overflow = Nibble::from_u8_high(ext_result).value != 0;
        (result, overflow)
    }
}

impl Add for Nibble {
    type Output = Nibble;
    fn add(self, other: Nibble) -> Nibble {
        Nibble {
            value: self.value + other.value,
        }
    }
}

impl Mul for Nibble {
    type Output = Nibble;
    fn mul(self, other: Nibble) -> Nibble {
        Nibble {
            value: self.value * other.value,
        }
    }
}

impl One for Nibble {
    fn one() -> Nibble {
        Nibble { value: 1 }
    }
}

impl Zero for Nibble {
    fn zero() -> Nibble {
        Nibble { value: 0 }
    }

    fn is_zero(&self) -> bool {
        self.value == 0
    }
}
