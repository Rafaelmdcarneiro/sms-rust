use vm::cpu::alu;

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub s: u8,
    pub p: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            s: 0xFF,
            p: 0xFF,
        }
    }

    pub fn u8s_to_u16(high: u8, low: u8) -> u16 {
        ((high as u16) << 0x08) | (low as u16)
    }

    pub fn u16_to_u8s(value: u16) -> (u8, u8) {
        let low = value as u8;
        let high = (value >> 8) as u8;
        (high, low)
    }

    pub(crate) fn assign_bytes(
        &mut self,
        target: fn(&mut Registers) -> (&mut u8, &mut u8),
        value: (u8, u8),
    ) {
        let reg = target(self);
        *reg.0 = value.0;
        *reg.1 = value.1;
    }

    pub(crate) fn assign_word(
        &mut self,
        target: fn(&mut Registers) -> (&mut u8, &mut u8),
        value: u16,
    ) {
        self.assign_bytes(target, alu::get_octets(value));
    }

    pub(crate) fn get_word(&mut self, target: fn(&mut Registers) -> (&mut u8, &mut u8)) -> u16 {
        let (high, low) = target(self);
        alu::get_word(*high, *low)
    }
}
