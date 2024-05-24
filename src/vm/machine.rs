use program::Program;
use vm::cpu::processor::Processor;
use vm::ram::memory::Memory;

pub struct Machine {
    pub cpu: Processor,
    pub ram: Memory,
}

impl Machine {
    pub fn new() -> Machine {
        Machine {
            cpu: Processor::new(),
            ram: Memory::new(),
        }
    }

    pub fn load_at(&mut self, program: &Program, start_address: u16) -> bool {
        let end = start_address as u32 + program.raw().len() as u32;
        let will_fit = end <= 65536;
        let mut address = start_address;
        if will_fit {
            for value in program.raw() {
                self.ram.write_u8(address, *value);
                address = address.wrapping_add(1);
            }
        }
        will_fit
    }

    pub fn load(&mut self, program: &Program) -> bool {
        self.load_at(program, 0)
    }

    pub fn start_at(&mut self, address: u16) {
        self.cpu.halt();
        self.cpu.goto(address);
        self.cpu.unhalt();
        while !self.cpu.is_halted() {
            self.execute();
        }
    }

    pub fn start(&mut self) {
        self.start_at(0);
    }
}
