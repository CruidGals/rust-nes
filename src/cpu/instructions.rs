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

pub fn inc_op(cpu: &mut CPU, value: u8) -> u8 {
    let result = value.wrapping_add(1);
    cpu.set_zn_flags(result);
    result
}

pub fn inx_op(cpu: &mut CPU) -> () {
    cpu.x = cpu.x.wrapping_add(1);
    cpu.set_zn_flags(cpu.x);
}

pub fn iny_op(cpu: &mut CPU) -> () {
    cpu.y = cpu.y.wrapping_add(1);
    cpu.set_zn_flags(cpu.y);
}

pub fn dec_op(cpu: &mut CPU, value: u8) -> u8 {
    let result = value.wrapping_sub(1);
    cpu.set_zn_flags(result);
    result
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

// ------------------------------------------------------------------
// COMPARE INSTRUCTIONS

pub fn cmp_op(cpu: &mut CPU, value: u8) -> () {
    let result = cpu.a - value;
    cpu.set_cmp_flags(cpu.a, result, value);
}

pub fn cpx_op(cpu: &mut CPU, value: u8) -> () {
    let result = cpu.x - value;
    cpu.set_cmp_flags(cpu.x, result, value);
}

pub fn cpy_op(cpu: &mut CPU, value: u8) -> () {
    let result = cpu.y - value;
    cpu.set_cmp_flags(cpu.y, result, value);
}

// ------------------------------------------------------------------
// BRANCH INSTRUCTIONS

pub fn bcc_op(cpu: &mut CPU) -> bool {
    !cpu.p.contains(StatusRegister::CARRY)
}

pub fn bcs_op(cpu: &mut CPU) -> bool {
    cpu.p.contains(StatusRegister::CARRY)
}

pub fn bne_op(cpu: &mut CPU) -> bool {
    !cpu.p.contains(StatusRegister::ZERO)
}

pub fn beq_op(cpu: &mut CPU) -> bool {
    cpu.p.contains(StatusRegister::ZERO)
}

pub fn bpl_op(cpu: &mut CPU) -> bool {
    !cpu.p.contains(StatusRegister::NEGATIVE)
}

pub fn bmi_op(cpu: &mut CPU) -> bool {
    cpu.p.contains(StatusRegister::NEGATIVE)
}

pub fn bvc_op(cpu: &mut CPU) -> bool {
    !cpu.p.contains(StatusRegister::OVERFLOW)
}

pub fn bvs_op(cpu: &mut CPU) -> bool {
    cpu.p.contains(StatusRegister::OVERFLOW)
}

// ------------------------------------------------------------------
// SHIFT INSTRUCTIONS

pub fn asl_op(cpu: &mut CPU, value: u8) -> u8 {
    let result = value << 1;
    
    cpu.p.set(StatusRegister::CARRY, (value & 0x80) != 0);
    cpu.set_zn_flags(result);
    result
}

pub fn lsr_op(cpu: &mut CPU, value: u8) -> u8 {
    let result = value >> 1;
    
    cpu.p.set(StatusRegister::CARRY, (value & 0x01) != 0);
    cpu.set_zn_flags(result);
    result
}

pub fn rol_op(cpu: &mut CPU, value: u8) -> u8 {
    // Rotate value left while moving MSB to LSB
    let carry = if  cpu.p.contains(StatusRegister::CARRY) { 1 } else { 0 };
    let result = (value << 1) | carry;
    
    cpu.p.set(StatusRegister::CARRY, (value & 0x80) != 0);
    cpu.set_zn_flags(result);
    result
}

pub fn ror_op(cpu: &mut CPU, value: u8) -> u8 {
    // Rotate value left while moving LSB to MSB
    let carry = if  cpu.p.contains(StatusRegister::CARRY) { 1 } else { 0 };
    let result = (value >> 1) | (carry << 7);
    
    cpu.p.set(StatusRegister::CARRY, (value & 0x01) != 0);
    cpu.set_zn_flags(result);
    result
}

// ------------------------------------------------------------------
// TRANSFER INSTRUCTIONS

pub fn tax_op(cpu: &mut CPU) -> () {
    cpu.x = cpu.a;
    cpu.set_zn_flags(cpu.x);
}

pub fn tay_op(cpu: &mut CPU) -> () {
    cpu.y = cpu.a;
    cpu.set_zn_flags(cpu.y);
}

pub fn txa_op(cpu: &mut CPU) -> () {
    cpu.a = cpu.x;
    cpu.set_zn_flags(cpu.a);
}

pub fn tya_op(cpu: &mut CPU) -> () {
    cpu.a = cpu.y;
    cpu.set_zn_flags(cpu.a);
}

pub fn tsx_op(cpu: &mut CPU) -> () {
    cpu.x = cpu.sp;
    cpu.set_zn_flags(cpu.x);
}
pub fn txs_op(cpu: &mut CPU) -> () {
    cpu.sp = cpu.x;
}

// ------------------------------------------------------------------
// FLAG INSTRUCTIONS

pub fn clc_op(cpu: &mut CPU) -> () {
    cpu.p.remove(StatusRegister::CARRY);
}

pub fn sec_op(cpu: &mut CPU) -> () {
    cpu.p.set(StatusRegister::CARRY, true);
}

pub fn cli_op(cpu: &mut CPU) -> () {
    cpu.p.remove(StatusRegister::INTERRUPT);
}

pub fn sei_op(cpu: &mut CPU) -> () {
    cpu.p.set(StatusRegister::INTERRUPT, true);
}

pub fn cld_op(cpu: &mut CPU) -> () {
    cpu.p.remove(StatusRegister::DECIMAL);
}

pub fn sed_op(cpu: &mut CPU) -> () {
    cpu.p.set(StatusRegister::DECIMAL, true);
}

pub fn clv_op(cpu: &mut CPU) -> () {
    cpu.p.remove(StatusRegister::OVERFLOW);
}

// STACK INSTRUCTIONS

// These instructions don't have a template to go off of,
// so these are the raw implementations

pub fn pha_op(cpu: &mut CPU) -> Result<(), String> {
    match cpu.cycle_step {
        2 => {
            cpu.bus.read(cpu.pc)
                .map_err(|err| format!("Problem reading from CPU VRAM: {err}"))?;
            cpu.step_cycle();
            Ok(())
        }
        3 => {
            cpu.target_addr = 0x0100 | u16::from(cpu.sp);
            cpu.bus.write(cpu.target_addr, cpu.a)
                .map_err(|err| format!("Error writing to CPU VRAM: {err}"))?;
            cpu.sp = cpu.sp.wrapping_sub(1);
            cpu.finish_instruction();
            Ok(())
        }
        _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
    }
}

pub fn php_op(cpu: &mut CPU) -> Result<(), String> {
    match cpu.cycle_step {
        2 => {
            cpu.bus.read(cpu.pc)
                .map_err(|err| format!("Problem reading from CPU VRAM: {err}"))?;
            cpu.step_cycle();
            Ok(())
        }
        3 => {
            cpu.target_addr = 0x0100 | u16::from(cpu.sp);

            // Set before pushing to stack
            cpu.p.set(StatusRegister::BREAK, true);

            cpu.bus.write(cpu.target_addr, cpu.p.bits())
                .map_err(|err| format!("Error writing to CPU VRAM: {err}"))?;
            cpu.sp = cpu.sp.wrapping_sub(1);
            cpu.finish_instruction();
            Ok(())
        }
        _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
    }
}

pub fn pla_op(cpu: &mut CPU) -> Result<(), String> {
    match cpu.cycle_step {
        2 => {
            cpu.bus.read(cpu.pc)
                .map_err(|err| format!("Problem reading from CPU VRAM: {err}"))?;
            cpu.step_cycle();
            Ok(())
        }
        3 => {
            cpu.sp = cpu.sp.wrapping_add(1);
            cpu.step_cycle();
            Ok(())
        }
        4 => {
            cpu.target_addr = 0x0100 | u16::from(cpu.sp);
            cpu.a = cpu.bus.read(cpu.target_addr)
                        .map_err(|err| format!("Problem reading from CPU VRAM: {err}"))?;
            cpu.set_zn_flags(cpu.a);
            cpu.finish_instruction();
            Ok(())
        }
        _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
    }
}

pub fn plp_op(cpu: &mut CPU) -> Result<(), String> {
    match cpu.cycle_step {
        2 => {
            cpu.bus.read(cpu.pc)
                .map_err(|err| format!("Problem reading from CPU VRAM: {err}"))?;
            cpu.step_cycle();
            Ok(())
        }
        3 => {
            cpu.sp = cpu.sp.wrapping_add(1);
            cpu.step_cycle();
            Ok(())
        }
        4 => {
            cpu.target_addr = 0x0100 | u16::from(cpu.sp);
            let mut status = cpu.bus.read(cpu.target_addr)
                        .map_err(|err| format!("Problem reading from CPU VRAM: {err}"))?;

            // Clear break bit and set unused flag
            status &= 0b11101111;
            status |= 0b00100000;
            
            cpu.p = StatusRegister::from_bits_retain(status);
            cpu.finish_instruction();
            Ok(())
        }
        _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
    }
}