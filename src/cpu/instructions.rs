/**
 * Instructions for the MOS 6502 CPU
 */

use crate::cpu::{self, cpu::StatusRegister, instruction_template};
use super::cpu::CPU;

// ==================================================================
// INSTRUCTIONS FOR THE MOS 6502 CPU
//
// Each instruction that is supported by this CPU will
// be implemented in this file. Note that the NES version
// of the CPU has some quirks (e.g. doesn't include decimal mode)
// so this file is specific to that.
//===================================================================

// ------------------------------------------------------------------
// MEMORY ACCESS INSTRUCTIONS

pub fn lda_op(cpu: &mut CPU, value: u8) -> () {
    cpu.a = value;
    cpu.set_zn_flags(cpu.a);
}

pub fn ldx_op(cpu: &mut CPU, value: u8) -> () {
    cpu.x = value;
    cpu.set_zn_flags(cpu.x);
}

pub fn ldy_op(cpu: &mut CPU, value: u8) -> () {
    cpu.y = value;
    cpu.set_zn_flags(cpu.y);
}

pub fn sta_op(cpu: &mut CPU) -> u8 {
    cpu.a
}

pub fn stx_op(cpu: &mut CPU) -> u8 {
    cpu.x
}

pub fn sty_op(cpu: &mut CPU) -> u8 {
    cpu.y
}


