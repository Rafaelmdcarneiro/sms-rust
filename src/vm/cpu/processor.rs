use vm::cpu::alu;
use vm::cpu::registers::Registers;
use vm::cpu::state::State;

pub struct Processor {
    pub state: State,
    halted: bool,
}

impl Processor {
    pub fn new() -> Processor {
        Processor {
            state: State::new(),
            halted: true,
        }
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }
    pub fn halt(&mut self) {
        self.halted = true;
    }
    pub fn unhalt(&mut self) {
        self.halted = false;
    }

    pub fn goto(&mut self, address: u16) {
        self.state.program_counter = address;
    }

    pub fn get_register(&self, selector: fn(&Registers) -> u8) -> u8 {
        selector(&self.state.registers)
    }

    pub fn get_register_pair(&self, selector: fn(&Registers) -> (u8, u8)) -> u16 {
        let (high, low) = selector(&self.state.registers);
        alu::get_word(high, low)
    }
}
