pub mod cpubus;
pub mod instruction_template;
pub mod instructions;

#[cfg(test)]
use crate::{cartridge::Cartridge, ppu::{PPU, ppubus::PPUBus}};
/***
 * Business logic for the 6502 CPU
 */
use crate::util::logger::Logger;
use cpubus::CPUBus;
use bitflags::bitflags;
use std::{cell::RefCell, error::Error, rc::Rc};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatusRegister: u8 {
        const CARRY     = 0b00000001;
        const ZERO      = 0b00000010;
        const INTERRUPT = 0b00000100;
        const DECIMAL   = 0b00001000;
        const BREAK     = 0b00010000;
        const OVERFLOW  = 0b01000000;
        const NEGATIVE  = 0b10000000;
    }
}

#[derive(Debug)]
pub struct CPU {
    /* Registers */
    pub a: u8, /* Accumulator */
    pub x: u8, /* X-Indexed register */
    pub y: u8, /* Y-Indexed register */
    pub pc: u16, /* Program Counter */
    pub sp: u8, /* Stack Pointer */
    pub p: StatusRegister, /* Status register */

    /* Instruction specific */
    curr_opcode: Option<fn(&mut CPU) -> Result<(), String>>,  /* opcode that is being done (in function form) */
    pub cycle_step: u8, /* cycle that opcode is on */
    pub low_buffer: u8, /* low read buffer when grabbing value */
    pub high_buffer: u8, /* high read buffer when grabbing value */
    pub page_boundary_cross: bool, /* checks if crossed page boundary */
    pub target_addr: u16, /* store addr for later use during write instructions */
    pub alu_buffer: u8, /* buffer used to perform ops with ALU */

    /* Debug */
    pub cycle_count: i32,
    pub instructions_ran: i32,
    pub logger: Option<Logger>,

    /* Wiring */
    pub bus: CPUBus,
}

impl CPU {
    pub fn new(cpu_bus: CPUBus, debug_mode: bool) -> Result<Self, String> {
        let logger = if debug_mode {
            Some(Logger::new()?)
        } else {
            None
        };

        Ok(Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0xFFFC,
            sp: 0xFD,
            p: StatusRegister::INTERRUPT,

            curr_opcode: None,
            cycle_step: 1,
            low_buffer: 0,
            high_buffer: 0,
            page_boundary_cross: false,
            target_addr: 0,
            alu_buffer: 0,

            cycle_count: 7,
            instructions_ran: 0,
            logger: logger,

            bus: cpu_bus
        })
    }

    pub fn tick(&mut self) -> Result<(), String> {
        // TODO stub: Polling PPU's NMI wire

        // Choose the opcode if on first cycle step
        if self.cycle_step == 1 {
            let opcode = self.read(self.pc)?;
            self.curr_opcode = self.choose_opcode(opcode);
            // Debug
            if let Some(logger) = &mut self.logger {
                logger.reset_cpu_state(self.pc, self.a, self.x, self.y, self.p.bits() | 0x20, self.sp, self.cycle_count);
                logger.add_opcode(opcode);
            }

            self.increment_pc();
            self.step_cycle();
        } else {
            // Run through the instruction
            if self.curr_opcode.is_none() {
                panic!("Cycle step incremented even though opcode not selected");
            }
            
            // Step thru the instruction
            let op = self.curr_opcode.unwrap();
            op(self)?;
        }

        // TODO: deal with interrup

        Ok(())
    }

    ///
    /// Creates the instruction given opcode.
    /// 
    fn choose_opcode(&mut self, opcode: u8) -> Option<fn(&mut CPU) -> Result<(), String>> {
        match opcode {
            0x00 => Some(|cpu| instructions::brk_op(cpu)),
            0x01 => Some(|cpu| instruction_template::indexed_indirect_addr_r(instructions::ora_op)(cpu)),
            0x02 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x03 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x04 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x05 => Some(|cpu| instruction_template::zero_page_addr_r(instructions::ora_op)(cpu)),
            0x06 => Some(|cpu| instruction_template::zero_page_addr_rmw(instructions::asl_op)(cpu)),
            0x07 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x08 => Some(|cpu| instructions::php_op(cpu)),
            0x09 => Some(|cpu| instruction_template::immediate_addr(instructions::ora_op)(cpu)),
            0x0A => Some(|cpu| instruction_template::accumulator_addr(instructions::asl_op)(cpu)),
            0x0B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x0C => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x0D => Some(|cpu| instruction_template::absolute_addr_r(instructions::ora_op)(cpu)),
            0x0E => Some(|cpu| instruction_template::absolute_addr_rmw(instructions::asl_op)(cpu)),
            0x0F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x10 => Some(|cpu| instruction_template::relative_addr(instructions::bpl_op)(cpu)),
            0x11 => Some(|cpu| instruction_template::indirect_indexed_addr_r(instructions::ora_op)(cpu)),
            0x12 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x13 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x14 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x15 => Some(|cpu| instruction_template::zero_page_indexed_x_addr_r(instructions::ora_op)(cpu)),
            0x16 => Some(|cpu| instruction_template::zero_page_indexed_x_addr_rmw(instructions::asl_op)(cpu)),
            0x17 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x18 => Some(|cpu| instruction_template::implied_addr(instructions::clc_op)(cpu)),
            0x19 => Some(|cpu| instruction_template::absolute_indexed_y_addr_r(instructions::ora_op)(cpu)),
            0x1A => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x1B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x1C => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x1D => Some(|cpu| instruction_template::absolute_indexed_x_addr_r(instructions::ora_op)(cpu)),
            0x1E => Some(|cpu| instruction_template::absolute_indexed_x_addr_rmw(instructions::asl_op)(cpu)),
            0x1F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x20 => Some(|cpu| instructions::jsr_op(cpu)),
            0x21 => Some(|cpu| instruction_template::indexed_indirect_addr_r(instructions::and_op)(cpu)),
            0x22 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x23 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x24 => Some(|cpu| instruction_template::zero_page_addr_r(instructions::bit_op)(cpu)),
            0x25 => Some(|cpu| instruction_template::zero_page_addr_r(instructions::and_op)(cpu)),
            0x26 => Some(|cpu| instruction_template::zero_page_addr_rmw(instructions::rol_op)(cpu)),
            0x27 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x28 => Some(|cpu| instructions::plp_op(cpu)),
            0x29 => Some(|cpu| instruction_template::immediate_addr(instructions::and_op)(cpu)),
            0x2A => Some(|cpu| instruction_template::accumulator_addr(instructions::rol_op)(cpu)),
            0x2B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x2C => Some(|cpu| instruction_template::absolute_addr_r(instructions::bit_op)(cpu)),
            0x2D => Some(|cpu| instruction_template::absolute_addr_r(instructions::and_op)(cpu)),
            0x2E => Some(|cpu| instruction_template::absolute_addr_rmw(instructions::rol_op)(cpu)),
            0x2F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x30 => Some(|cpu| instruction_template::relative_addr(instructions::bmi_op)(cpu)),
            0x31 => Some(|cpu| instruction_template::indirect_indexed_addr_r(instructions::and_op)(cpu)),
            0x32 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x33 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x34 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x35 => Some(|cpu| instruction_template::zero_page_indexed_x_addr_r(instructions::and_op)(cpu)),
            0x36 => Some(|cpu| instruction_template::zero_page_indexed_x_addr_rmw(instructions::rol_op)(cpu)),
            0x37 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x38 => Some(|cpu| instruction_template::implied_addr(instructions::sec_op)(cpu)),
            0x39 => Some(|cpu| instruction_template::absolute_indexed_y_addr_r(instructions::and_op)(cpu)),
            0x3A => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x3B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x3C => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x3D => Some(|cpu| instruction_template::absolute_indexed_x_addr_r(instructions::and_op)(cpu)),
            0x3E => Some(|cpu| instruction_template::absolute_indexed_x_addr_rmw(instructions::rol_op)(cpu)),
            0x3F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x40 => Some(|cpu| instructions::rti_op(cpu)),
            0x41 => Some(|cpu| instruction_template::indexed_indirect_addr_r(instructions::eor_op)(cpu)),
            0x42 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x43 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x44 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x45 => Some(|cpu| instruction_template::zero_page_addr_r(instructions::eor_op)(cpu)),
            0x46 => Some(|cpu| instruction_template::zero_page_addr_rmw(instructions::lsr_op)(cpu)),
            0x47 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x48 => Some(|cpu| instructions::pha_op(cpu)),
            0x49 => Some(|cpu| instruction_template::immediate_addr(instructions::eor_op)(cpu)),
            0x4A => Some(|cpu| instruction_template::accumulator_addr(instructions::lsr_op)(cpu)),
            0x4B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x4C => Some(|cpu| instructions::jmp_op_absolute(cpu)),
            0x4D => Some(|cpu| instruction_template::absolute_addr_r(instructions::eor_op)(cpu)),
            0x4E => Some(|cpu| instruction_template::absolute_addr_rmw(instructions::lsr_op)(cpu)),
            0x4F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x50 => Some(|cpu| instruction_template::relative_addr(instructions::bvc_op)(cpu)),
            0x51 => Some(|cpu| instruction_template::indirect_indexed_addr_r(instructions::eor_op)(cpu)),
            0x52 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x53 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x54 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x55 => Some(|cpu| instruction_template::zero_page_indexed_x_addr_r(instructions::eor_op)(cpu)),
            0x56 => Some(|cpu| instruction_template::zero_page_indexed_x_addr_rmw(instructions::lsr_op)(cpu)),
            0x57 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x58 => Some(|cpu| instruction_template::implied_addr(instructions::cli_op)(cpu)),
            0x59 => Some(|cpu| instruction_template::absolute_indexed_y_addr_r(instructions::eor_op)(cpu)),
            0x5A => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x5B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x5C => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x5D => Some(|cpu| instruction_template::absolute_indexed_x_addr_r(instructions::eor_op)(cpu)),
            0x5E => Some(|cpu| instruction_template::absolute_indexed_x_addr_rmw(instructions::lsr_op)(cpu)),
            0x5F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x60 => Some(|cpu| instructions::rts_op(cpu)),
            0x61 => Some(|cpu| instruction_template::indexed_indirect_addr_r(instructions::adc_op)(cpu)),
            0x62 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x63 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x64 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x65 => Some(|cpu| instruction_template::zero_page_addr_r(instructions::adc_op)(cpu)),
            0x66 => Some(|cpu| instruction_template::zero_page_addr_rmw(instructions::ror_op)(cpu)),
            0x67 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x68 => Some(|cpu| instructions::pla_op(cpu)),
            0x69 => Some(|cpu| instruction_template::immediate_addr(instructions::adc_op)(cpu)),
            0x6A => Some(|cpu| instruction_template::accumulator_addr(instructions::ror_op)(cpu)),
            0x6B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x6C => Some(|cpu| instructions::jmp_op_indirect(cpu)),
            0x6D => Some(|cpu| instruction_template::absolute_addr_r(instructions::adc_op)(cpu)),
            0x6E => Some(|cpu| instruction_template::absolute_addr_rmw(instructions::ror_op)(cpu)),
            0x6F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x70 => Some(|cpu| instruction_template::relative_addr(instructions::bvs_op)(cpu)),
            0x71 => Some(|cpu| instruction_template::indirect_indexed_addr_r(instructions::adc_op)(cpu)),
            0x72 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x73 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x74 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x75 => Some(|cpu| instruction_template::zero_page_indexed_x_addr_r(instructions::adc_op)(cpu)),
            0x76 => Some(|cpu| instruction_template::zero_page_indexed_x_addr_rmw(instructions::ror_op)(cpu)),
            0x77 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x78 => Some(|cpu| instruction_template::implied_addr(instructions::sei_op)(cpu)),
            0x79 => Some(|cpu| instruction_template::absolute_indexed_y_addr_r(instructions::adc_op)(cpu)),
            0x7A => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x7B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x7C => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x7D => Some(|cpu| instruction_template::absolute_indexed_x_addr_r(instructions::adc_op)(cpu)),
            0x7E => Some(|cpu| instruction_template::absolute_indexed_x_addr_rmw(instructions::ror_op)(cpu)),
            0x7F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x80 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x81 => Some(|cpu| instruction_template::indexed_indirect_addr_w(instructions::sta_op)(cpu)),
            0x82 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x83 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x84 => Some(|cpu| instruction_template::zero_page_addr_w(instructions::sty_op)(cpu)),
            0x85 => Some(|cpu| instruction_template::zero_page_addr_w(instructions::sta_op)(cpu)),
            0x86 => Some(|cpu| instruction_template::zero_page_addr_w(instructions::stx_op)(cpu)),
            0x87 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x88 => Some(|cpu| instruction_template::implied_addr(instructions::dey_op)(cpu)),
            0x89 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x8A => Some(|cpu| instruction_template::implied_addr(instructions::txa_op)(cpu)),
            0x8B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x8C => Some(|cpu| instruction_template::absolute_addr_w(instructions::sty_op)(cpu)),
            0x8D => Some(|cpu| instruction_template::absolute_addr_w(instructions::sta_op)(cpu)),
            0x8E => Some(|cpu| instruction_template::absolute_addr_w(instructions::stx_op)(cpu)),
            0x8F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x90 => Some(|cpu| instruction_template::relative_addr(instructions::bcc_op)(cpu)),
            0x91 => Some(|cpu| instruction_template::indirect_indexed_addr_w(instructions::sta_op)(cpu)),
            0x92 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x93 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x94 => Some(|cpu| instruction_template::zero_page_indexed_x_addr_w(instructions::sty_op)(cpu)),
            0x95 => Some(|cpu| instruction_template::zero_page_indexed_x_addr_w(instructions::sta_op)(cpu)),
            0x96 => Some(|cpu| instruction_template::zero_page_indexed_y_addr_w(instructions::stx_op)(cpu)),
            0x97 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x98 => Some(|cpu| instruction_template::implied_addr(instructions::tya_op)(cpu)),
            0x99 => Some(|cpu| instruction_template::absolute_indexed_y_addr_w(instructions::sta_op)(cpu)),
            0x9A => Some(|cpu| instruction_template::implied_addr(instructions::txs_op)(cpu)),
            0x9B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x9C => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x9D => Some(|cpu| instruction_template::absolute_indexed_x_addr_w(instructions::sta_op)(cpu)),
            0x9E => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x9F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xA0 => Some(|cpu| instruction_template::immediate_addr(instructions::ldy_op)(cpu)),
            0xA1 => Some(|cpu| instruction_template::indexed_indirect_addr_r(instructions::lda_op)(cpu)),
            0xA2 => Some(|cpu| instruction_template::immediate_addr(instructions::ldx_op)(cpu)),
            0xA3 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xA4 => Some(|cpu| instruction_template::zero_page_addr_r(instructions::ldy_op)(cpu)),
            0xA5 => Some(|cpu| instruction_template::zero_page_addr_r(instructions::lda_op)(cpu)),
            0xA6 => Some(|cpu| instruction_template::zero_page_addr_r(instructions::ldx_op)(cpu)),
            0xA7 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xA8 => Some(|cpu| instruction_template::implied_addr(instructions::tay_op)(cpu)),
            0xA9 => Some(|cpu| instruction_template::immediate_addr(instructions::lda_op)(cpu)),
            0xAA => Some(|cpu| instruction_template::implied_addr(instructions::tax_op)(cpu)),
            0xAB => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xAC => Some(|cpu| instruction_template::absolute_addr_r(instructions::ldy_op)(cpu)),
            0xAD => Some(|cpu| instruction_template::absolute_addr_r(instructions::lda_op)(cpu)),
            0xAE => Some(|cpu| instruction_template::absolute_addr_r(instructions::ldx_op)(cpu)),
            0xAF => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xB0 => Some(|cpu| instruction_template::relative_addr(instructions::bcs_op)(cpu)),
            0xB1 => Some(|cpu| instruction_template::indirect_indexed_addr_r(instructions::lda_op)(cpu)),
            0xB2 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xB3 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xB4 => Some(|cpu| instruction_template::zero_page_indexed_x_addr_r(instructions::ldy_op)(cpu)),
            0xB5 => Some(|cpu| instruction_template::zero_page_indexed_x_addr_r(instructions::lda_op)(cpu)),
            0xB6 => Some(|cpu| instruction_template::zero_page_indexed_y_addr_r(instructions::ldx_op)(cpu)),
            0xB7 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xB8 => Some(|cpu| instruction_template::implied_addr(instructions::clv_op)(cpu)),
            0xB9 => Some(|cpu| instruction_template::absolute_indexed_y_addr_r(instructions::lda_op)(cpu)),
            0xBA => Some(|cpu| instruction_template::implied_addr(instructions::tsx_op)(cpu)),
            0xBB => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xBC => Some(|cpu| instruction_template::absolute_indexed_x_addr_r(instructions::ldy_op)(cpu)),
            0xBD => Some(|cpu| instruction_template::absolute_indexed_x_addr_r(instructions::lda_op)(cpu)),
            0xBE => Some(|cpu| instruction_template::absolute_indexed_y_addr_r(instructions::ldx_op)(cpu)),
            0xBF => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xC0 => Some(|cpu| instruction_template::immediate_addr(instructions::cpy_op)(cpu)),
            0xC1 => Some(|cpu| instruction_template::indexed_indirect_addr_r(instructions::cmp_op)(cpu)),
            0xC2 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xC3 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xC4 => Some(|cpu| instruction_template::zero_page_addr_r(instructions::cpy_op)(cpu)),
            0xC5 => Some(|cpu| instruction_template::zero_page_addr_r(instructions::cmp_op)(cpu)),
            0xC6 => Some(|cpu| instruction_template::zero_page_addr_rmw(instructions::dec_op)(cpu)),
            0xC7 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xC8 => Some(|cpu| instruction_template::implied_addr(instructions::iny_op)(cpu)),
            0xC9 => Some(|cpu| instruction_template::immediate_addr(instructions::cmp_op)(cpu)),
            0xCA => Some(|cpu| instruction_template::implied_addr(instructions::dex_op)(cpu)),
            0xCB => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xCC => Some(|cpu| instruction_template::absolute_addr_r(instructions::cpy_op)(cpu)),
            0xCD => Some(|cpu| instruction_template::absolute_addr_r(instructions::cmp_op)(cpu)),
            0xCE => Some(|cpu| instruction_template::absolute_addr_rmw(instructions::dec_op)(cpu)),
            0xCF => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xD0 => Some(|cpu| instruction_template::relative_addr(instructions::bne_op)(cpu)),
            0xD1 => Some(|cpu| instruction_template::indirect_indexed_addr_r(instructions::cmp_op)(cpu)),
            0xD2 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xD3 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xD4 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xD5 => Some(|cpu| instruction_template::zero_page_indexed_x_addr_r(instructions::cmp_op)(cpu)),
            0xD6 => Some(|cpu| instruction_template::zero_page_indexed_x_addr_rmw(instructions::dec_op)(cpu)),
            0xD7 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xD8 => Some(|cpu| instruction_template::implied_addr(instructions::cld_op)(cpu)),
            0xD9 => Some(|cpu| instruction_template::absolute_indexed_y_addr_r(instructions::cmp_op)(cpu)),
            0xDA => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xDB => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xDC => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xDD => Some(|cpu| instruction_template::absolute_indexed_x_addr_r(instructions::cmp_op)(cpu)),
            0xDE => Some(|cpu| instruction_template::absolute_indexed_x_addr_rmw(instructions::dec_op)(cpu)),
            0xDF => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xE0 => Some(|cpu| instruction_template::immediate_addr(instructions::cpx_op)(cpu)),
            0xE1 => Some(|cpu| instruction_template::indexed_indirect_addr_r(instructions::sbc_op)(cpu)),
            0xE2 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xE3 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xE4 => Some(|cpu| instruction_template::zero_page_addr_r(instructions::cpx_op)(cpu)),
            0xE5 => Some(|cpu| instruction_template::zero_page_addr_r(instructions::sbc_op)(cpu)),
            0xE6 => Some(|cpu| instruction_template::zero_page_addr_rmw(instructions::inc_op)(cpu)),
            0xE7 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xE8 => Some(|cpu| instruction_template::implied_addr(instructions::inx_op)(cpu)),
            0xE9 => Some(|cpu| instruction_template::immediate_addr(instructions::sbc_op)(cpu)),
            0xEA => Some(|cpu| instruction_template::implied_addr(instructions::nop_op)(cpu)),
            0xEB => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xEC => Some(|cpu| instruction_template::absolute_addr_r(instructions::cpx_op)(cpu)),
            0xED => Some(|cpu| instruction_template::absolute_addr_r(instructions::sbc_op)(cpu)),
            0xEE => Some(|cpu| instruction_template::absolute_addr_rmw(instructions::inc_op)(cpu)),
            0xEF => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xF0 => Some(|cpu| instruction_template::relative_addr(instructions::beq_op)(cpu)),
            0xF1 => Some(|cpu| instruction_template::indirect_indexed_addr_r(instructions::sbc_op)(cpu)),
            0xF2 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xF3 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xF4 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xF5 => Some(|cpu| instruction_template::zero_page_indexed_x_addr_r(instructions::sbc_op)(cpu)),
            0xF6 => Some(|cpu| instruction_template::zero_page_indexed_x_addr_rmw(instructions::inc_op)(cpu)),
            0xF7 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xF8 => Some(|cpu| instruction_template::implied_addr(instructions::sed_op)(cpu)),
            0xF9 => Some(|cpu| instruction_template::absolute_indexed_y_addr_r(instructions::sbc_op)(cpu)),
            0xFA => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xFB => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xFC => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xFD => Some(|cpu| instruction_template::absolute_indexed_x_addr_r(instructions::sbc_op)(cpu)),
            0xFE => Some(|cpu| instruction_template::absolute_indexed_x_addr_rmw(instructions::inc_op)(cpu)),
            0xFF => todo!("Unimplemented opcode: {:#04X}", opcode),
        }
    }
}

// Instruction specific functions
impl CPU {
    ///
    /// Steps through a cycle in the instruction. Increments cycle_step
    /// 
    pub fn step_cycle(&mut self) -> () {
        self.cycle_step += 1;
        self.cycle_count += 1;
    }

    ///
    /// Concludes the current instruction being ran. Resets the
    /// current opcode being ran and the cycle step counter. Resets
    /// all necessary buffers.
    /// 
    pub fn finish_instruction(&mut self) -> Result<(), String> {
        self.curr_opcode = None;
        self.cycle_step = 1;
        self.low_buffer = 0;
        self.high_buffer = 0;
        self.page_boundary_cross = false;
        self.target_addr = 0;
        self.alu_buffer = 0;
        self.cycle_count += 1;

        /* Debug */
        self.instructions_ran += 1;
        if let Some(logger) = &mut self.logger {
            logger.log()?;
        }
        Ok(())
    }

    ///
    /// Sets the zero and negative flags. Common for a lot
    /// of instructions from NES CPU
    /// 
    pub fn set_zn_flags(&mut self, result: u8) -> () {
        self.p.set(StatusRegister::ZERO, result == 0);
        self.p.set(StatusRegister::NEGATIVE, result & 0x80 != 0);
    }

    pub fn set_cmp_flags(&mut self, register: u8, result: u8, value: u8) -> () {
        self.p.set(StatusRegister::CARRY, register >= value);
        self.p.set(StatusRegister::ZERO, register == value);
        self.p.set(StatusRegister::NEGATIVE, result & 0x80 != 0);
    }
}

// Separate functions for navigating pointers and
// simplifying semantics
impl CPU {
    #[inline]
    pub fn increment_pc(&mut self) -> () {
        self.pc = self.pc.wrapping_add(1);
    }

    #[inline]
    pub fn increment_sp(&mut self) -> () {
        self.sp = self.sp.wrapping_add(1);
    }

    #[inline]
    pub fn decrement_sp(&mut self) -> () {
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn read(&self, address: u16) -> Result<u8, String> {
        self.bus.read(address)
            .map_err(|err| format!("Error reading from CPU VRAM at {address:#06X}: {err}"))
    }

    pub fn write(&mut self, address: u16, value: u8) -> Result<(), String> {
        self.bus.write(address, value)
            .map_err(|err| format!("Error writing to CPU VRAM at {address:#06X}: {err}"))
    }

    /// Fetches the byte at current program counter, and increments program counter
    pub fn fetch(&mut self) -> Result<u8, String> {
        let byte = self.read(self.pc)?;
        if let Some(logger) = &mut self.logger {
            logger.add_opcode(byte);
        }
        self.increment_pc();
        Ok(byte)
    }
}

#[cfg(test)]
pub(crate) fn test_cartridge() -> Rc<RefCell<Cartridge>> {
    test_cartridge_with_prg(&[])
}

#[cfg(test)]
pub(crate) fn test_cartridge_with_prg(
    prg_rom: &[u8],
) -> Rc<RefCell<Cartridge>> {
    const PRG_BANK: usize = 16 * 1024;
    const CHR_BANK: usize = 8 * 1024;

    let prg_banks = (prg_rom.len().max(1) + PRG_BANK - 1) / PRG_BANK;
    assert!(prg_banks <= 2, "NROM PRG ROM is at most 32KB");

    let mut rom = vec![0u8; 16 + prg_banks * PRG_BANK + CHR_BANK];
    rom[..4].copy_from_slice(b"NES\x1A");
    rom[4] = prg_banks as u8;
    rom[5] = 1;
    rom[16..16 + prg_rom.len()].copy_from_slice(prg_rom);

    Rc::new(RefCell::new(
        Cartridge::new(&rom).unwrap(),
    ))
}

#[cfg(test)]
pub(crate) fn test_ppu(
    cartridge: &Rc<RefCell<Cartridge>>,
) -> Rc<RefCell<PPU>> {
    let bus = PPUBus::new(Rc::clone(cartridge));
    Rc::new(RefCell::new(PPU::new(bus).unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::cpubus::CPUBus;

    fn run_until(cpu: &mut CPU, end_pc: u16) {
        for _ in 0..500 {
            if cpu.pc == end_pc && cpu.cycle_step == 1 {
                return;
            }
            cpu.tick().unwrap();
        }
        panic!(
            "did not reach {end_pc:#06X} (pc={:#06X}, cycle_step={})",
            cpu.pc, cpu.cycle_step
        );
    }

    #[test]
    fn test_tick_instruction_sequence() {
        // Program lives in NROM PRG ROM at $8000 so opcode fetches go through the
        // cartridge, not internal RAM. Workspace is $50-$53 (internal RAM) plus
        // $6000 (PRG RAM). Subroutine is at $8100 so it cannot overlap the program.
        //
        // LDA #$10 / TAX / TAY / INX / INY / TXA / CLC / ADC #$01 / STA $50 / LDA $50
        // ASL A / LSR A / ORA #$80 / AND #$F0 / EOR #$10 / SEC / SBC #$01
        // PHA / LDA #$00 / PLA / STX $0052 / STY $0053 / CMP #$7F
        // BNE (not taken) / NOP / NOP / BEQ (taken, skips LDA #$AA)
        // LDX #$05 / INC $50 / DEC $50 / BIT $50 / DEY / DEX
        // JSR $8100 / NOP / NOP / LDA #$77 / STA $6000 / LDA $6000
        let program = [
            0xA9, 0x10, // LDA #$10
            0xAA,       // TAX
            0xA8,       // TAY
            0xE8,       // INX
            0xC8,       // INY
            0x8A,       // TXA
            0x18,       // CLC
            0x69, 0x01, // ADC #$01
            0x85, 0x50, // STA $50
            0xA5, 0x50, // LDA $50
            0x0A,       // ASL A
            0x4A,       // LSR A
            0x09, 0x80, // ORA #$80
            0x29, 0xF0, // AND #$F0
            0x49, 0x10, // EOR #$10
            0x38,       // SEC
            0xE9, 0x01, // SBC #$01
            0x48,       // PHA
            0xA9, 0x00, // LDA #$00
            0x68,       // PLA
            0x8E, 0x52, 0x00, // STX $0052
            0x8C, 0x53, 0x00, // STY $0053
            0xC9, 0x7F, // CMP #$7F
            0xD0, 0x02, // BNE +2 (not taken)
            0xEA,       // NOP
            0xEA,       // NOP
            0xF0, 0x02, // BEQ +2 (taken)
            0xA9, 0xAA, // LDA #$AA (skipped)
            0xA2, 0x05, // LDX #$05
            0xE6, 0x50, // INC $50
            0xC6, 0x50, // DEC $50
            0x24, 0x50, // BIT $50
            0x88,       // DEY
            0xCA,       // DEX
            0x20, 0x00, 0x81, // JSR $8100
            0xEA,       // NOP
            0xEA,       // NOP
            0xA9, 0x77, // LDA #$77
            0x8D, 0x00, 0x60, // STA $6000
            0xAD, 0x00, 0x60, // LDA $6000
        ];

        let mut prg_rom = vec![0u8; 16 * 1024];
        prg_rom[..program.len()].copy_from_slice(&program);

        // Subroutine at $8100: LDA #$42 / STA $51 / RTS
        prg_rom[0x0100] = 0xA9;
        prg_rom[0x0101] = 0x42;
        prg_rom[0x0102] = 0x85;
        prg_rom[0x0103] = 0x51;
        prg_rom[0x0104] = 0x60;

        let cart = test_cartridge_with_prg(&prg_rom);
        let bus = CPUBus::new(Rc::clone(&cart), test_ppu(&cart));
        let mut cpu = CPU::new(bus, false).unwrap();

        // Fetch must come from PRG ROM. A CPU write to $8000 is an NROM no-op, so
        // this would stay LDA #$10 only if the cart mapping is real.
        assert_eq!(cpu.read(0x8000).unwrap(), 0xA9);
        cpu.write(0x8000, 0x00).unwrap();
        assert_eq!(cpu.read(0x8000).unwrap(), 0xA9);

        cpu.pc = 0x8000;
        run_until(&mut cpu, 0x8044);

        // A=$77 from the PRG-RAM load; X/Y after INX/INY, LDX #$05, DEX, DEY
        assert_eq!(cpu.a, 0x77);
        assert_eq!(cpu.x, 0x04);
        assert_eq!(cpu.y, 0x10);
        assert_eq!(cpu.sp, 0xFD);
        assert_eq!(cpu.pc, 0x8044);
        assert_eq!(cpu.cycle_step, 1);

        // $50 ended at $12 (STA $12, INC, DEC). $51 written by the JSR target.
        // $52/$53 captured X/Y before LDX #$05.
        assert_eq!(cpu.read(0x0050).unwrap(), 0x12);
        assert_eq!(cpu.read(0x0051).unwrap(), 0x42);
        assert_eq!(cpu.read(0x0052).unwrap(), 0x11);
        assert_eq!(cpu.read(0x0053).unwrap(), 0x11);

        // $6000 must land in the mapper, not only look correct through CPUBus.
        assert_eq!(cpu.read(0x6000).unwrap(), 0x77);
        assert_eq!(cart.borrow().cpu_read(0x6000).unwrap(), 0x77);

        // CMP left C set. LDA #$77 left Z and N clear. BIT $50 cleared V.
        assert!(cpu.p.contains(StatusRegister::CARRY));
        assert!(!cpu.p.contains(StatusRegister::ZERO));
        assert!(!cpu.p.contains(StatusRegister::NEGATIVE));
        assert!(!cpu.p.contains(StatusRegister::OVERFLOW));
    }
}