use num::One;
use std::ops::Add;
use std::ops::Not;
use vm::cpu::alu;

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Operation {
    Add = 0,
    Subtract = 1,
}

impl Operation {
    pub fn maybe_negate<T: Add<Output = T> + Not<Output = T> + One>(self, value: T) -> T {
        if self == Operation::Add {
            value
        } else {
            alu::negate(value)
        }
    }
}
