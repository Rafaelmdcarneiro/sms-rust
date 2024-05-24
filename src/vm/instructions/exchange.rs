use std::mem;
use vm::cpu::registers::Registers;
use vm::machine::Machine;

impl Machine {
    pub(crate) fn shadow_exchange_af(&mut self) {
        self.exchange_with_shadow(vec![|regs| &mut regs.a, |regs| &mut regs.f]);
        self.clock(4);
    }

    pub(crate) fn shadow_exchange_bc_de_hl(&mut self) {
        self.exchange_with_shadow(vec![
            |regs| &mut regs.b,
            |regs| &mut regs.c,
            |regs| &mut regs.d,
            |regs| &mut regs.e,
            |regs| &mut regs.h,
            |regs| &mut regs.l,
        ]);
        self.clock(4);
    }

    pub(crate) fn exhange_de_with_hl(&mut self) {
        self.exchange(vec![
            |regs| (&mut regs.d, &mut regs.h),
            |regs| (&mut regs.e, &mut regs.l),
        ]);
        self.clock(4);
    }

    pub(crate) fn exchage_memory_from_sp_with_hl(&mut self) {
        {
            let reg = &mut self.cpu.state.registers;
            let low_address = (reg.s as u16) << 8 | reg.p as u16;
            let high_address = low_address + 1;
            self.ram.write_u8(low_address, reg.l);
            self.ram.write_u8(high_address, reg.h);
        }
        self.clock(19);
    }

    fn exchange(&mut self, selectors: Vec<fn(&mut Registers) -> (&mut u8, &mut u8)>) {
        let reg = &mut self.cpu.state.registers;
        for s in selectors {
            let (r1, r2) = s(reg);
            mem::swap(r1, r2);
        }
    }

    fn exchange_with_shadow(&mut self, selectors: Vec<fn(&mut Registers) -> &mut u8>) {
        let reg = &mut self.cpu.state.registers;
        let alt = &mut self.cpu.state.alt_registers;
        for s in selectors {
            mem::swap(s(reg), s(alt));
        }
    }
}
