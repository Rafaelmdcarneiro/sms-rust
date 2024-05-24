use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Flag {
    Carry = 1,
    AddSubtract = 2,
    ParityOverflow = 4,
    Unused1 = 8,
    HalfCarry = 16,
    Unused2 = 32,
    Zero = 64,
    Sign = 128,
}

impl Flag {
    pub fn set(self, register: &mut u8, value: bool) {
        let mask = self as u8;
        if value {
            *register |= mask;
        } else {
            *register &= !mask;
        };
    }

    pub fn get(self, register: &u8) -> bool {
        self.get_bit(register) > 0
    }

    pub fn get_bit(self, register: &u8) -> u8 {
        let mask = self as u8;
        *register & mask
    }

    pub fn all() -> [Flag; 8] {
        [
            Flag::Carry,
            Flag::AddSubtract,
            Flag::ParityOverflow,
            Flag::HalfCarry,
            Flag::Zero,
            Flag::Sign,
            Flag::Unused1,
            Flag::Unused2,
        ]
    }

    pub(crate) fn set_values(status: &mut u8, affected: &[Flag], values: &[(Flag, bool)]) {
        let map: HashMap<Flag, bool> = values.iter().cloned().collect();
        for flag in affected {
            match map.get(&flag) {
                Some(value) => flag.set(status, *value),
                None => {}
            }
        }
    }
}
