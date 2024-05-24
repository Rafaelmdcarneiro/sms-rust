use vm::cpu::alu;
use vm::cpu::flags::Flag;
use vm::cpu::operation::Operation;
use vm::cpu::registers::Registers;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn increment_register_wide(
        &mut self,
        target: fn(&mut Registers) -> (&mut u8, &mut u8),
    ) {
        self.operate_on_register_pair(Operation::Add, target, (0, 1), &[]);
        self.clock(6);
    }

    pub(crate) fn decrement_register_wide(
        &mut self,
        target: fn(&mut Registers) -> (&mut u8, &mut u8),
    ) {
        self.operate_on_register_pair(Operation::Subtract, target, (0, 1), &[]);
        self.clock(6);
    }

    pub(crate) fn add_register_pair_to_hl(&mut self, selector: fn(&Registers) -> (u8, u8)) {
        self.add_register_pair(|regs| (&mut regs.h, &mut regs.l), selector);
    }

    fn add_register_pair(
        &mut self,
        target: fn(&mut Registers) -> (&mut u8, &mut u8),
        selector: fn(&Registers) -> (u8, u8),
    ) {
        let operand = selector(&self.cpu.state.registers);
        self.operate_on_register_pair(
            Operation::Add,
            target,
            operand,
            &[Flag::Carry, Flag::HalfCarry, Flag::AddSubtract],
        );
        self.clock(11);
    }

    fn operate_on_register_pair(
        &mut self,
        operation: Operation,
        target: fn(&mut Registers) -> (&mut u8, &mut u8),
        operand: (u8, u8),
        affected_flags: &[Flag],
    ) {
        let op1 = self.cpu.state.registers.get_word(target);
        let op2 = operation.maybe_negate(alu::get_word_from_tuple(operand));
        let result = alu::add_words(op1, op2);
        self.cpu.state.registers.assign_word(target, result.value);
        Flag::set_values(
            &mut self.cpu.state.status,
            affected_flags,
            &[
                (Flag::Zero, result.value == 0x0000),
                (Flag::Sign, result.value > 0x7FFF),
                (Flag::HalfCarry, result.half_carry),
                (Flag::ParityOverflow, result.overflow),
                (Flag::AddSubtract, operation == Operation::Subtract),
                (Flag::Carry, result.carry),
            ],
        );
    }
}
