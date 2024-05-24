use vm::cpu::registers::Registers;

pub struct Memory {
    data: [u8; 65536],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { data: [0; 65536] }
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        let low = self.read_u8(address) as u16;
        let high = self.read_u8(address + 1) as u16;
        (high << 8) | low
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    pub fn write_u16(&mut self, address: u16, value: u16) {
        let (low, high) = Registers::u16_to_u8s(value);
        self.write_u8(address, low);
        self.write_u8(address + 1, high);
    }
}
