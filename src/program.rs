use vm::instructions::opcodes::Opcode;

pub struct Program {
    bin: Vec<u8>,
}

impl Program {
    pub fn new() -> Program {
        Program { bin: Vec::new() }
    }

    pub fn raw(&self) -> &Vec<u8> {
        &self.bin
    }

    pub fn add(&mut self, opcode: Opcode) {
        self.bin.push(opcode as u8);
    }

    pub fn add_param(&mut self, opcode: Opcode, parameter: u8) {
        self.bin.push(opcode as u8);
        self.bin.push(parameter);
    }

    pub fn add_param_word(&mut self, opcode: Opcode, parameter: u16) {
        let low = parameter as u8;
        let high = (parameter >> 0x08) as u8;
        self.bin.push(opcode as u8);
        self.bin.push(low);
        self.bin.push(high);
    }

    pub fn add_params(&mut self, opcode: Opcode, parameter_1: u8, parameter_2: u8) {
        self.bin.push(opcode as u8);
        self.bin.push(parameter_1);
        self.bin.push(parameter_2);
    }

    pub fn add_vector(&mut self, mut parameters: Vec<u8>) {
        self.bin.append(&mut parameters);
    }
}
