#[cfg(test)]
mod tests {
    use program::Program;
    use vm::cpu::alu;
    use vm::cpu::flags::Flag;
    use vm::cpu::registers::Registers;
    use vm::instructions::opcodes::Opcode;
    use vm::machine::Machine;

    fn new_vm(regs: fn(&mut Registers), stream: Vec<Opcode>, start: u16) -> Machine {
        let mut vm = Machine::new();
        let mut p = Program::new();
        p.add_vector(stream.iter().map(|i| *i as u8).collect());
        vm.load_at(&p, start);
        regs(&mut vm.cpu.state.registers);
        vm.cpu.goto(start);
        vm
    }

    fn run_program(regs: fn(&mut Registers), stream: Vec<Opcode>) -> Machine {
        let mut vm = new_vm(regs, stream, 0);
        vm.start();
        vm
    }

    fn run_program_from_default_state(stream: Vec<Opcode>) -> Machine {
        run_program(|_| {}, stream)
    }

    #[test]
    fn nibbles() {
        for iteration in 0..256 {
            let i = iteration as u8;
            let r = alu::add_octets(i, 1);
            assert_eq!(i.wrapping_add(1), r.value, "At {}.", i);
            assert_eq!(i == 0xFF, r.carry, "At {}.", i);
            assert_eq!((i & 0x0F) == 0x0F, r.half_carry, "At {}.", i);
            assert_eq!(i == 0x7F, r.overflow, "At {}.", i);
        }
    }

    #[test]
    fn words() {
        for iteration in 0..65536 {
            let i = iteration as u16;
            let r = alu::add_words(i, 1);
            assert_eq!(i.wrapping_add(1), r.value, "At {}.", i);
            assert_eq!(i == 0xFFFF, r.carry, "At {}.", i);
            assert_eq!((i & 0x0FFF) == 0x0FFF, r.half_carry, "At {}.", i);
            assert_eq!(i == 0x7FFF, r.overflow, "At {}.", i);
        }
    }

    #[test]
    fn increment() {
        let range = || 0..256;
        let mut vm = new_vm(|_| {}, range().map(|_| Opcode::IncA).collect(), 0);
        for iteration in range() {
            let i = iteration as u8;
            let a = vm.cpu.get_register(|regs| regs.a);
            let h = Flag::HalfCarry.get(&vm.cpu.state.status);
            let s = Flag::Sign.get(&vm.cpu.state.status);
            let ov = Flag::ParityOverflow.get(&vm.cpu.state.status);
            assert_eq!(i, a);
            assert_eq!(i >= 0x80, s, "At value {}.", i);
            assert_eq!(i == 0x80, ov, "At value {}.", i);
            if i > 0 {
                assert_eq!(i & 0x0F == 0, h, "At value {}.", i);
            }
            vm.execute();
        }
    }

    #[test]
    fn increment_pair() {
        let range = || 0..65536;
        let mut vm = new_vm(|_| {}, range().map(|_| Opcode::IncBC).collect(), 0);
        for iteration in range() {
            let i = iteration as u16;
            let bc = vm.cpu.get_register_pair(|regs| (regs.b, regs.c));
            assert_eq!(i, bc);
            vm.execute();
        }
    }

    #[test]
    fn add() {
        let mut vm = run_program(
            |regs| {
                regs.a = 0x7E;
                regs.b = 0x01;
            },
            vec![Opcode::AddB, Opcode::Halt],
        );

        assert_eq!(vm.cpu.get_register(|regs| regs.a), 0x7F);
        assert!(!Flag::ParityOverflow.get(&vm.cpu.state.status));
        assert!(!Flag::Sign.get(&vm.cpu.state.status));
        assert!(!Flag::Carry.get(&vm.cpu.state.status));

        vm.start_at(0);
        assert_eq!(vm.cpu.get_register(|regs| regs.a), 0x80);
        assert!(Flag::ParityOverflow.get(&vm.cpu.state.status));
        assert!(Flag::Sign.get(&vm.cpu.state.status));
        assert!(!Flag::Carry.get(&vm.cpu.state.status));

        vm.start_at(0);
        assert_eq!(vm.cpu.get_register(|regs| regs.a), 0x81);
        assert!(!Flag::ParityOverflow.get(&vm.cpu.state.status));
        assert!(Flag::Sign.get(&vm.cpu.state.status));
        assert!(!Flag::Carry.get(&vm.cpu.state.status));

        vm.cpu.state.registers.a = 0xFF;
        vm.start_at(0);
        assert_eq!(vm.cpu.get_register(|regs| regs.a), 0x00);
        assert!(!Flag::ParityOverflow.get(&vm.cpu.state.status));
        assert!(!Flag::Sign.get(&vm.cpu.state.status));
        assert!(Flag::Carry.get(&vm.cpu.state.status));
    }

    #[test]
    fn increment_wide() {
        let mut vm = run_program(
            |regs| {
                regs.b = 0x00;
                regs.c = 0xFE;
            },
            vec![Opcode::IncBC, Opcode::Halt],
        );
        assert_eq!(vm.cpu.get_register(|regs| regs.b), 0x00);
        assert_eq!(vm.cpu.get_register(|regs| regs.c), 0xFF);

        vm.start_at(0);
        assert_eq!(vm.cpu.get_register(|regs| regs.b), 0x01);
        assert_eq!(vm.cpu.get_register(|regs| regs.c), 0x00);
    }

    #[test]
    fn decrement_wide() {
        let mut vm = run_program(
            |regs| {
                regs.b = 0x01;
                regs.c = 0x00;
            },
            vec![Opcode::DecBC, Opcode::Halt],
        );
        assert_eq!(vm.cpu.get_register(|regs| regs.b), 0x00);
        assert_eq!(vm.cpu.get_register(|regs| regs.c), 0xFF);

        vm.start_at(0);
        assert_eq!(vm.cpu.get_register(|regs| regs.b), 0x00);
        assert_eq!(vm.cpu.get_register(|regs| regs.c), 0xFE);
    }

    fn jump_test_flag(opcode: Opcode, param: u16, flag: Flag, flag_value: bool, expected: u16) {
        let mut vm = Machine::new();
        let mut p = Program::new();
        p.add_param_word(opcode, param);
        p.add(Opcode::Halt);
        p.add(Opcode::Halt);
        vm.load(&p);
        flag.set(&mut vm.cpu.state.status, flag_value);
        vm.start();
        assert_eq!(vm.cpu.state.program_counter, expected);
    }

    #[test]
    fn jump() {
        jump_test_flag(Opcode::JpXX, 0x04, Flag::Unused1, true, 0x05);
        jump_test_flag(Opcode::JpNZXX, 0x04, Flag::Zero, false, 0x05);
        jump_test_flag(Opcode::JpNZXX, 0x04, Flag::Zero, true, 0x04);
        jump_test_flag(Opcode::JpZXX, 0x04, Flag::Zero, true, 0x05);
        jump_test_flag(Opcode::JpZXX, 0x04, Flag::Zero, false, 0x04);
        jump_test_flag(Opcode::JpNCXX, 0x04, Flag::Carry, false, 0x05);
        jump_test_flag(Opcode::JpNCXX, 0x04, Flag::Carry, true, 0x04);
        jump_test_flag(Opcode::JpCXX, 0x04, Flag::Carry, true, 0x05);
        jump_test_flag(Opcode::JpCXX, 0x04, Flag::Carry, false, 0x04);
        jump_test_flag(Opcode::JpPOXX, 0x04, Flag::ParityOverflow, true, 0x05);
        jump_test_flag(Opcode::JpPOXX, 0x04, Flag::ParityOverflow, false, 0x04);
        jump_test_flag(Opcode::JpPEXX, 0x04, Flag::ParityOverflow, false, 0x05);
        jump_test_flag(Opcode::JpPEXX, 0x04, Flag::ParityOverflow, true, 0x04);
        jump_test_flag(Opcode::JpPXX, 0x04, Flag::Sign, false, 0x05);
        jump_test_flag(Opcode::JpPXX, 0x04, Flag::Sign, true, 0x04);
        jump_test_flag(Opcode::JpMXX, 0x04, Flag::Sign, true, 0x05);
        jump_test_flag(Opcode::JpMXX, 0x04, Flag::Sign, false, 0x04);
    }

    #[test]
    fn load() {
        let mut vm = Machine::new();
        let mut p = Program::new();
        p.add(Opcode::LdBC);
        vm.load(&p);

        vm.cpu.state.registers.b = 0;
        vm.cpu.state.registers.c = 20;

        vm.start();

        assert_eq!(vm.cpu.state.registers.b, 20);
    }

    #[test]
    fn load_param() {
        let mut vm = Machine::new();
        let mut p = Program::new();
        p.add_param(Opcode::LdBX, 42);
        vm.load(&p);

        vm.cpu.state.registers.b = 0;

        vm.start();

        assert_eq!(vm.cpu.state.registers.b, 42);
    }
}
