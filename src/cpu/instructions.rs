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

// ------------------------------------------------------------------
// ARITHMETIC INSTRUCTIONS

pub fn adc_op(cpu: &mut CPU, value: u8) -> () {
    let carry: u16 = if cpu.p.contains(StatusRegister::CARRY) { 1 } else { 0 };
    let sum = u16::from(cpu.a) + u16::from(value) + carry;
    let result = sum as u8;

    // Set flags
    cpu.set_zn_flags(result);
    cpu.p.set(StatusRegister::CARRY, sum > 0xFF);
    cpu.p.set(
        StatusRegister::OVERFLOW, 
        ((result ^ cpu.a) & (result ^ value) & 0x80) != 0
    );

    cpu.a = result;
}

// Just does ADC with an inverted value
pub fn sbc_op(cpu: &mut CPU, value: u8) -> () {
    adc_op(cpu, !value);
}

pub fn inc_op(cpu: &mut CPU) -> () {
    cpu.alu_buffer = cpu.alu_buffer.wrapping_add(1);
    cpu.set_zn_flags(cpu.alu_buffer);
}

pub fn inx_op(cpu: &mut CPU) -> () {
    cpu.x = cpu.x.wrapping_add(1);
    cpu.set_zn_flags(cpu.x);
}

pub fn iny_op(cpu: &mut CPU) -> () {
    cpu.y = cpu.y.wrapping_add(1);
    cpu.set_zn_flags(cpu.y);
}

pub fn dec_op(cpu: &mut CPU) -> () {
    cpu.alu_buffer = cpu.alu_buffer.wrapping_sub(1);
    cpu.set_zn_flags(cpu.alu_buffer);
}

pub fn dex_op(cpu: &mut CPU) -> () {
    cpu.x = cpu.x.wrapping_sub(1);
    cpu.set_zn_flags(cpu.x);
}

pub fn dey_op(cpu: &mut CPU) -> () {
    cpu.y = cpu.y.wrapping_sub(1);
    cpu.set_zn_flags(cpu.y);
}

// ------------------------------------------------------------------
// BITWISE INSTRUCTIONS

pub fn and_op(cpu: &mut CPU, value: u8) -> () {
    cpu.a = cpu.a & value;
    cpu.set_zn_flags(cpu.a);
}

pub fn ora_op(cpu: &mut CPU, value: u8) -> () {
    cpu.a = cpu.a | value;
    cpu.set_zn_flags(cpu.a);
}

pub fn eor_op(cpu: &mut CPU, value: u8) -> () {
    cpu.a = cpu.a ^ value;
    cpu.set_zn_flags(cpu.a);
}

pub fn bit_op(cpu: &mut CPU, value: u8) -> () {
    let result = cpu.a & value;
    
    // Flags are set a bit weirder here
    cpu.p.set(StatusRegister::ZERO, result == 0);
    cpu.p.set(StatusRegister::OVERFLOW, (value & 0x40) != 0);
    cpu.p.set(StatusRegister::NEGATIVE, (value & 0x80) != 0);
}

