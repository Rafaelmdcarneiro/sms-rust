mod arithmetic_16bit;
mod arithmetic_8bit;
mod bitwise;
mod control;
mod exchange;
mod memory;
pub mod opcodes;
mod stack;

use vm::cpu::flags::Flag;
use vm::instructions::opcodes::Opcode;
use vm::machine::Machine;

impl Machine {
    pub fn execute(&mut self) {
        let opcode = Opcode::from(self.next_byte());
        match opcode {
            Opcode::Nop => self.nop(),

            Opcode::Exx => self.shadow_exchange_bc_de_hl(),
            Opcode::ExAFAF => self.shadow_exchange_af(),
            Opcode::ExDEHL => self.exhange_de_with_hl(),
            Opcode::ExVSPHL => self.exchage_memory_from_sp_with_hl(),

            Opcode::IncA => self.increment_register(|regs| &mut regs.a),
            Opcode::IncB => self.increment_register(|regs| &mut regs.b),
            Opcode::IncC => self.increment_register(|regs| &mut regs.c),
            Opcode::IncD => self.increment_register(|regs| &mut regs.d),
            Opcode::IncE => self.increment_register(|regs| &mut regs.e),
            Opcode::IncH => self.increment_register(|regs| &mut regs.h),
            Opcode::IncL => self.increment_register(|regs| &mut regs.l),

            Opcode::DecA => self.decrement_register(|regs| &mut regs.a),
            Opcode::DecB => self.decrement_register(|regs| &mut regs.b),
            Opcode::DecC => self.decrement_register(|regs| &mut regs.c),
            Opcode::DecD => self.decrement_register(|regs| &mut regs.d),
            Opcode::DecE => self.decrement_register(|regs| &mut regs.e),
            Opcode::DecH => self.decrement_register(|regs| &mut regs.h),
            Opcode::DecL => self.decrement_register(|regs| &mut regs.l),

            Opcode::IncBC => self.increment_register_wide(|regs| (&mut regs.b, &mut regs.c)),
            Opcode::IncDE => self.increment_register_wide(|regs| (&mut regs.d, &mut regs.e)),
            Opcode::IncHL => self.increment_register_wide(|regs| (&mut regs.h, &mut regs.l)),
            Opcode::IncSP => self.increment_register_wide(|regs| (&mut regs.s, &mut regs.p)),

            Opcode::DecBC => self.decrement_register_wide(|regs| (&mut regs.b, &mut regs.c)),
            Opcode::DecDE => self.decrement_register_wide(|regs| (&mut regs.d, &mut regs.e)),
            Opcode::DecHL => self.decrement_register_wide(|regs| (&mut regs.h, &mut regs.l)),
            Opcode::DecSP => self.decrement_register_wide(|regs| (&mut regs.s, &mut regs.p)),

            Opcode::AddA => self.add_register(|regs| regs.a),
            Opcode::AddB => self.add_register(|regs| regs.b),
            Opcode::AddC => self.add_register(|regs| regs.c),
            Opcode::AddD => self.add_register(|regs| regs.d),
            Opcode::AddE => self.add_register(|regs| regs.e),
            Opcode::AddH => self.add_register(|regs| regs.h),
            Opcode::AddL => self.add_register(|regs| regs.l),

            Opcode::SubA => self.subtract_register(|regs| regs.a),
            Opcode::SubB => self.subtract_register(|regs| regs.b),
            Opcode::SubC => self.subtract_register(|regs| regs.c),
            Opcode::SubD => self.subtract_register(|regs| regs.d),
            Opcode::SubE => self.subtract_register(|regs| regs.e),
            Opcode::SubH => self.subtract_register(|regs| regs.h),
            Opcode::SubL => self.subtract_register(|regs| regs.l),

            Opcode::AddHLBC => self.add_register_pair_to_hl(|regs| (regs.b, regs.c)),
            Opcode::AddHLDE => self.add_register_pair_to_hl(|regs| (regs.d, regs.e)),
            Opcode::AddHLHL => self.add_register_pair_to_hl(|regs| (regs.h, regs.l)),
            Opcode::AddHLSP => self.add_register_pair_to_hl(|regs| (regs.s, regs.p)),

            Opcode::AdcA => self.add_carry_register(|regs| regs.a),
            Opcode::AdcB => self.add_carry_register(|regs| regs.b),
            Opcode::AdcC => self.add_carry_register(|regs| regs.c),
            Opcode::AdcD => self.add_carry_register(|regs| regs.d),
            Opcode::AdcE => self.add_carry_register(|regs| regs.e),
            Opcode::AdcH => self.add_carry_register(|regs| regs.h),
            Opcode::AdcL => self.add_carry_register(|regs| regs.l),

            Opcode::SbcA => self.subtract_carry_register(|regs| regs.a),
            Opcode::SbcB => self.subtract_carry_register(|regs| regs.b),
            Opcode::SbcC => self.subtract_carry_register(|regs| regs.c),
            Opcode::SbcD => self.subtract_carry_register(|regs| regs.d),
            Opcode::SbcE => self.subtract_carry_register(|regs| regs.e),
            Opcode::SbcH => self.subtract_carry_register(|regs| regs.h),
            Opcode::SbcL => self.subtract_carry_register(|regs| regs.l),

            Opcode::JpXX => self.jump(|_| true),
            Opcode::JpNZXX => self.jump(|status| !Flag::Zero.get(status)),
            Opcode::JpZXX => self.jump(|status| Flag::Zero.get(status)),
            Opcode::JpNCXX => self.jump(|status| !Flag::Carry.get(status)),
            Opcode::JpCXX => self.jump(|status| Flag::Carry.get(status)),
            Opcode::JpPOXX => self.jump(|status| Flag::ParityOverflow.get(status)),
            Opcode::JpPEXX => self.jump(|status| !Flag::ParityOverflow.get(status)),
            Opcode::JpPXX => self.jump(|status| !Flag::Sign.get(status)),
            Opcode::JpMXX => self.jump(|status| Flag::Sign.get(status)),

            Opcode::CallXX => self.call(|_| true),
            Opcode::CallNZXX => self.call(|status| !Flag::Zero.get(status)),
            Opcode::CallZXX => self.call(|status| Flag::Zero.get(status)),
            Opcode::CallNCXX => self.call(|status| !Flag::Carry.get(status)),
            Opcode::CallCXX => self.call(|status| Flag::Carry.get(status)),
            Opcode::CallPOXX => self.call(|status| Flag::ParityOverflow.get(status)),
            Opcode::CallPEXX => self.call(|status| !Flag::ParityOverflow.get(status)),
            Opcode::CallPXX => self.call(|status| !Flag::Sign.get(status)),
            Opcode::CallMXX => self.call(|status| Flag::Sign.get(status)),

            Opcode::Ret => self.ret(),
            Opcode::RetNZ => self.ret_conditional(|status| !Flag::Zero.get(status)),
            Opcode::RetZ => self.ret_conditional(|status| Flag::Zero.get(status)),
            Opcode::RetNC => self.ret_conditional(|status| !Flag::Carry.get(status)),
            Opcode::RetC => self.ret_conditional(|status| Flag::Carry.get(status)),
            Opcode::RetPO => self.ret_conditional(|status| Flag::ParityOverflow.get(status)),
            Opcode::RetPE => self.ret_conditional(|status| !Flag::ParityOverflow.get(status)),
            Opcode::RetP => self.ret_conditional(|status| !Flag::Sign.get(status)),
            Opcode::RetM => self.ret_conditional(|status| Flag::Sign.get(status)),

            Opcode::LdBCXX => self.load_into_register_pair(|regs| (&mut regs.b, &mut regs.c)),
            Opcode::LdDEXX => self.load_into_register_pair(|regs| (&mut regs.d, &mut regs.e)),
            Opcode::LdHLXX => self.load_into_register_pair(|regs| (&mut regs.h, &mut regs.l)),
            Opcode::LdSPXX => self.load_into_register_pair(|regs| (&mut regs.s, &mut regs.p)),

            Opcode::LdVBCA => self.load_into_memory(|regs| regs.a, |regs| (regs.b, regs.c)),
            Opcode::LdVDEA => self.load_into_memory(|regs| regs.a, |regs| (regs.d, regs.e)),

            Opcode::LdBA => self.load_register_into_register(|regs| regs.a, |regs| &mut regs.b),
            Opcode::LdBB => self.load_register_into_register(|regs| regs.b, |regs| &mut regs.b),
            Opcode::LdBC => self.load_register_into_register(|regs| regs.c, |regs| &mut regs.b),
            Opcode::LdBD => self.load_register_into_register(|regs| regs.d, |regs| &mut regs.b),
            Opcode::LdBE => self.load_register_into_register(|regs| regs.e, |regs| &mut regs.b),
            Opcode::LdBH => self.load_register_into_register(|regs| regs.h, |regs| &mut regs.b),
            Opcode::LdBL => self.load_register_into_register(|regs| regs.l, |regs| &mut regs.b),
            Opcode::LdBHL => {
                self.load_memory_into_register(|regs| (regs.h, regs.l), |regs| &mut regs.b)
            }

            Opcode::LdCA => self.load_register_into_register(|regs| regs.a, |regs| &mut regs.c),
            Opcode::LdCB => self.load_register_into_register(|regs| regs.b, |regs| &mut regs.c),
            Opcode::LdCC => self.load_register_into_register(|regs| regs.c, |regs| &mut regs.c),
            Opcode::LdCD => self.load_register_into_register(|regs| regs.d, |regs| &mut regs.c),
            Opcode::LdCE => self.load_register_into_register(|regs| regs.e, |regs| &mut regs.c),
            Opcode::LdCH => self.load_register_into_register(|regs| regs.h, |regs| &mut regs.c),
            Opcode::LdCL => self.load_register_into_register(|regs| regs.l, |regs| &mut regs.c),
            Opcode::LdCHL => {
                self.load_memory_into_register(|regs| (regs.h, regs.l), |regs| &mut regs.c)
            }

            Opcode::LdDA => self.load_register_into_register(|regs| regs.a, |regs| &mut regs.d),
            Opcode::LdDB => self.load_register_into_register(|regs| regs.b, |regs| &mut regs.d),
            Opcode::LdDC => self.load_register_into_register(|regs| regs.c, |regs| &mut regs.d),
            Opcode::LdDD => self.load_register_into_register(|regs| regs.d, |regs| &mut regs.d),
            Opcode::LdDE => self.load_register_into_register(|regs| regs.e, |regs| &mut regs.d),
            Opcode::LdDH => self.load_register_into_register(|regs| regs.h, |regs| &mut regs.d),
            Opcode::LdDL => self.load_register_into_register(|regs| regs.l, |regs| &mut regs.d),
            Opcode::LdDHL => {
                self.load_memory_into_register(|regs| (regs.h, regs.l), |regs| &mut regs.d)
            }

            Opcode::LdEA => self.load_register_into_register(|regs| regs.a, |regs| &mut regs.e),
            Opcode::LdEB => self.load_register_into_register(|regs| regs.b, |regs| &mut regs.e),
            Opcode::LdEC => self.load_register_into_register(|regs| regs.c, |regs| &mut regs.e),
            Opcode::LdED => self.load_register_into_register(|regs| regs.d, |regs| &mut regs.e),
            Opcode::LdEE => self.load_register_into_register(|regs| regs.e, |regs| &mut regs.e),
            Opcode::LdEH => self.load_register_into_register(|regs| regs.h, |regs| &mut regs.e),
            Opcode::LdEL => self.load_register_into_register(|regs| regs.l, |regs| &mut regs.e),
            Opcode::LdEHL => {
                self.load_memory_into_register(|regs| (regs.h, regs.l), |regs| &mut regs.e)
            }

            Opcode::LdHA => self.load_register_into_register(|regs| regs.a, |regs| &mut regs.h),
            Opcode::LdHB => self.load_register_into_register(|regs| regs.b, |regs| &mut regs.h),
            Opcode::LdHC => self.load_register_into_register(|regs| regs.c, |regs| &mut regs.h),
            Opcode::LdHD => self.load_register_into_register(|regs| regs.d, |regs| &mut regs.h),
            Opcode::LdHE => self.load_register_into_register(|regs| regs.e, |regs| &mut regs.h),
            Opcode::LdHH => self.load_register_into_register(|regs| regs.h, |regs| &mut regs.h),
            Opcode::LdHL => self.load_register_into_register(|regs| regs.l, |regs| &mut regs.h),
            Opcode::LdHHL => {
                self.load_memory_into_register(|regs| (regs.h, regs.l), |regs| &mut regs.h)
            }

            Opcode::LdLA => self.load_register_into_register(|regs| regs.a, |regs| &mut regs.l),
            Opcode::LdLB => self.load_register_into_register(|regs| regs.b, |regs| &mut regs.l),
            Opcode::LdLC => self.load_register_into_register(|regs| regs.c, |regs| &mut regs.l),
            Opcode::LdLD => self.load_register_into_register(|regs| regs.d, |regs| &mut regs.l),
            Opcode::LdLE => self.load_register_into_register(|regs| regs.e, |regs| &mut regs.l),
            Opcode::LdLH => self.load_register_into_register(|regs| regs.h, |regs| &mut regs.l),
            Opcode::LdLL => self.load_register_into_register(|regs| regs.l, |regs| &mut regs.l),
            Opcode::LdLHL => {
                self.load_memory_into_register(|regs| (regs.h, regs.l), |regs| &mut regs.l)
            }

            Opcode::LdHLA => self.load_register_into_memory(|regs| regs.a, |regs| (regs.h, regs.l)),
            Opcode::LdHLB => self.load_register_into_memory(|regs| regs.b, |regs| (regs.h, regs.l)),
            Opcode::LdHLC => self.load_register_into_memory(|regs| regs.c, |regs| (regs.h, regs.l)),
            Opcode::LdHLD => self.load_register_into_memory(|regs| regs.d, |regs| (regs.h, regs.l)),
            Opcode::LdHLE => self.load_register_into_memory(|regs| regs.e, |regs| (regs.h, regs.l)),
            Opcode::LdHLH => self.load_register_into_memory(|regs| regs.h, |regs| (regs.h, regs.l)),
            Opcode::LdHLL => self.load_register_into_memory(|regs| regs.l, |regs| (regs.h, regs.l)),

            Opcode::LdAA => self.load_register_into_register(|regs| regs.a, |regs| &mut regs.a),
            Opcode::LdAB => self.load_register_into_register(|regs| regs.b, |regs| &mut regs.a),
            Opcode::LdAC => self.load_register_into_register(|regs| regs.c, |regs| &mut regs.a),
            Opcode::LdAD => self.load_register_into_register(|regs| regs.d, |regs| &mut regs.a),
            Opcode::LdAE => self.load_register_into_register(|regs| regs.e, |regs| &mut regs.a),
            Opcode::LdAH => self.load_register_into_register(|regs| regs.h, |regs| &mut regs.a),
            Opcode::LdAL => self.load_register_into_register(|regs| regs.l, |regs| &mut regs.a),
            Opcode::LdAHL => {
                self.load_memory_into_register(|regs| (regs.h, regs.l), |regs| &mut regs.a)
            }

            Opcode::LdAX => self.load_into_register(|regs| &mut regs.a),
            Opcode::LdBX => self.load_into_register(|regs| &mut regs.b),
            Opcode::LdCX => self.load_into_register(|regs| &mut regs.c),
            Opcode::LdDX => self.load_into_register(|regs| &mut regs.d),
            Opcode::LdEX => self.load_into_register(|regs| &mut regs.e),
            Opcode::LdHX => self.load_into_register(|regs| &mut regs.h),
            Opcode::LdLX => self.load_into_register(|regs| &mut regs.l),

            Opcode::LdAVBC => {
                self.load_memory_into_register(|regs| (regs.b, regs.c), |regs| &mut regs.a)
            }
            Opcode::LdAVDE => {
                self.load_memory_into_register(|regs| (regs.d, regs.e), |regs| &mut regs.a)
            }
            Opcode::LdVXXHL => self.load_wide_register_into_param_memory(|regs| (regs.h, regs.l)),
            Opcode::LdHLVXX => {
                self.load_param_memory_into_wide_register(|regs| (&mut regs.h, &mut regs.l))
            }
            Opcode::LdVXXA => self.load_register_into_param_memory(|regs| regs.a),
            Opcode::LdAVXX => self.load_param_memory_into_register(|regs| &mut regs.a),
            Opcode::LdVHLX => self.load_param_into_memory(|regs| (regs.h, regs.l)),

            Opcode::AndA => self.and_register(|regs| regs.a),
            Opcode::AndB => self.and_register(|regs| regs.b),
            Opcode::AndC => self.and_register(|regs| regs.c),
            Opcode::AndD => self.and_register(|regs| regs.d),
            Opcode::AndE => self.and_register(|regs| regs.e),
            Opcode::AndH => self.and_register(|regs| regs.h),
            Opcode::AndL => self.and_register(|regs| regs.l),
            Opcode::AndX => self.and_value(),

            Opcode::OrA => self.or_register(|regs| regs.a),
            Opcode::OrB => self.or_register(|regs| regs.b),
            Opcode::OrC => self.or_register(|regs| regs.c),
            Opcode::OrD => self.or_register(|regs| regs.d),
            Opcode::OrE => self.or_register(|regs| regs.e),
            Opcode::OrH => self.or_register(|regs| regs.h),
            Opcode::OrL => self.or_register(|regs| regs.l),
            Opcode::OrX => self.or_value(),

            Opcode::XorA => self.xor_register(|regs| regs.a),
            Opcode::XorB => self.xor_register(|regs| regs.b),
            Opcode::XorC => self.xor_register(|regs| regs.c),
            Opcode::XorD => self.xor_register(|regs| regs.d),
            Opcode::XorE => self.xor_register(|regs| regs.e),
            Opcode::XorH => self.xor_register(|regs| regs.h),
            Opcode::XorL => self.xor_register(|regs| regs.l),
            Opcode::XorX => self.xor_value(),

            Opcode::PushAF => self.push_to_stack(|regs| (regs.a, regs.f)),
            Opcode::PushBC => self.push_to_stack(|regs| (regs.b, regs.c)),
            Opcode::PushDE => self.push_to_stack(|regs| (regs.d, regs.e)),
            Opcode::PushHL => self.push_to_stack(|regs| (regs.h, regs.l)),

            Opcode::PopAF => self.pop_from_stack(|regs| (&mut regs.a, &mut regs.f)),
            Opcode::PopBC => self.pop_from_stack(|regs| (&mut regs.b, &mut regs.c)),
            Opcode::PopDE => self.pop_from_stack(|regs| (&mut regs.d, &mut regs.e)),
            Opcode::PopHL => self.pop_from_stack(|regs| (&mut regs.h, &mut regs.l)),

            Opcode::SCF => self.set_carry_flag(),
            Opcode::CCF => self.complement_carry_flag(),
            Opcode::CPL => self.complement_registers(|regs| &mut regs.a),
            Opcode::RLCA => self.rotate_accumulator_left(),

            Opcode::Halt => self.halt(),
        }
    }

    fn next_byte(&mut self) -> u8 {
        let pc = self.cpu.state.program_counter;
        let val = self.ram.read_u8(pc);
        let (result, overflow) = pc.overflowing_add(1);
        if overflow {
            self.cpu.halt();
        } else {
            self.cpu.state.program_counter = result;
        }
        val
    }

    fn next_byte_pair(&mut self) -> (u8, u8) {
        let low = self.next_byte();
        let high = self.next_byte();
        (high, low)
    }

    fn next_word(&mut self) -> u16 {
        let low = self.next_byte() as u16;
        let high = self.next_byte() as u16;
        (high << 8) | low
    }

    pub fn clock(&mut self, _tstates: u8) {
        // TODO: Something.
    }
}
