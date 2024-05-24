use vm::cpu::flags::Flag;
use vm::cpu::registers::Registers;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn set_carry_flag(&mut self) {
        {
            let status = &mut self.cpu.state.status;
            Flag::Carry.set(status, true);
            Flag::HalfCarry.set(status, false);
            Flag::AddSubtract.set(status, false);
        }
        self.clock(4);
    }

    pub(crate) fn complement_carry_flag(&mut self) {
        {
            let previous = Flag::Carry.get(&self.cpu.state.status);
            let status = &mut self.cpu.state.status;
            Flag::Carry.set(status, !previous);
            Flag::HalfCarry.set(status, previous);
            Flag::AddSubtract.set(status, false);
        }
        self.clock(4);
    }

    pub(crate) fn complement_registers(&mut self, selector: fn(&mut Registers) -> &mut u8) {
        *selector(&mut self.cpu.state.registers) = *selector(&mut self.cpu.state.registers);
        Flag::AddSubtract.set(&mut self.cpu.state.status, true);
        Flag::HalfCarry.set(&mut self.cpu.state.status, true);
        self.clock(4);
    }

    pub(crate) fn and_register(&mut self, selector: fn(&Registers) -> u8) {
        self.bitwise_with_register(selector, |a, b| a & b, true);
    }

    pub(crate) fn or_register(&mut self, selector: fn(&Registers) -> u8) {
        self.bitwise_with_register(selector, |a, b| a | b, false);
    }

    pub(crate) fn xor_register(&mut self, selector: fn(&Registers) -> u8) {
        self.bitwise_with_register(selector, |a, b| a ^ b, false);
    }

    pub(crate) fn and_value(&mut self) {
        self.bitwise_with_value(|a, b| a & b, true);
    }

    pub(crate) fn or_value(&mut self) {
        self.bitwise_with_value(|a, b| a | b, false);
    }

    pub(crate) fn xor_value(&mut self) {
        self.bitwise_with_value(|a, b| a ^ b, false);
    }

    fn bitwise_with_register(
        &mut self,
        selector: fn(&Registers) -> u8,
        operation: fn(u8, u8) -> u8,
        half_carry_value: bool,
    ) {
        let operand = selector(&self.cpu.state.registers);
        self.bitwise_operation(operand, operation, half_carry_value);
        self.clock(4);
    }

    fn bitwise_with_value(&mut self, operation: fn(u8, u8) -> u8, half_carry_value: bool) {
        let operand = self.next_byte();
        self.bitwise_operation(operand, operation, half_carry_value);
        self.clock(7);
    }

    fn bitwise_operation(
        &mut self,
        operand: u8,
        operation: fn(u8, u8) -> u8,
        half_carry_value: bool,
    ) {
        let op1 = self.cpu.state.registers.a;
        let op2 = operand;
        let result = operation(op1, op2);
        let parity = (0..8).fold(0, |acc, b| acc + (result >> b) & 1) % 2 == 0;

        let status = &mut self.cpu.state.status;
        Flag::ParityOverflow.set(status, parity);
        Flag::Carry.set(status, false);
        Flag::HalfCarry.set(status, half_carry_value);
        Flag::AddSubtract.set(status, false);
        Flag::Zero.set(status, result == 0x00);
        Flag::Sign.set(status, result > 0x7F);
    }

    pub(crate) fn rotate_accumulator_left(&mut self) {
        let old_value = self.cpu.state.registers.a as u16;
        let second_most_significant_bit = if old_value & 0x40 != 0 { 1 } else { 0 };
        let new_value = (old_value << 1) + second_most_significant_bit;
        self.cpu.state.registers.a = new_value as u8;
        Flag::Carry.set(&mut self.cpu.state.status, second_most_significant_bit == 1);
        Flag::HalfCarry.set(&mut self.cpu.state.status, false);
        Flag::AddSubtract.set(&mut self.cpu.state.status, false);
        self.clock(4);
    }
}
