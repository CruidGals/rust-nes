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
    let result = cpu.a.wrapping_sub(value);
    cpu.set_cmp_flags(cpu.a, result, value);
}

pub fn cpx_op(cpu: &mut CPU, value: u8) -> () {
    let result = cpu.x.wrapping_sub(value);
    cpu.set_cmp_flags(cpu.x, result, value);
}

pub fn cpy_op(cpu: &mut CPU, value: u8) -> () {
    let result = cpu.y.wrapping_sub(value);
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

// ------------------------------------------------------------------
// STACK INSTRUCTIONS

// These instructions don't have a template to go off of,
// so these are the raw implementations

pub fn pha_op(cpu: &mut CPU) -> Result<(), String> {
    match cpu.cycle_step {
        2 => {
            cpu.read(cpu.pc)?;
            cpu.step_cycle();
            Ok(())
        }
        3 => {
            cpu.target_addr = 0x0100 | u16::from(cpu.sp);
            cpu.write(cpu.target_addr, cpu.a)?;
            cpu.decrement_sp();
            cpu.finish_instruction();
            Ok(())
        }
        _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
    }
}

pub fn php_op(cpu: &mut CPU) -> Result<(), String> {
    match cpu.cycle_step {
        2 => {
            cpu.read(cpu.pc)?;
            cpu.step_cycle();
            Ok(())
        }
        3 => {
            cpu.target_addr = 0x0100 | u16::from(cpu.sp);

            // Set before pushing to stack
            cpu.p.set(StatusRegister::BREAK, true);

            cpu.write(cpu.target_addr, cpu.p.bits())?;
            cpu.decrement_sp();
            cpu.finish_instruction();
            Ok(())
        }
        _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
    }
}

pub fn pla_op(cpu: &mut CPU) -> Result<(), String> {
    match cpu.cycle_step {
        2 => {
            cpu.read(cpu.pc)?;
            cpu.step_cycle();
            Ok(())
        }
        3 => {
            cpu.increment_sp();
            cpu.step_cycle();
            Ok(())
        }
        4 => {
            cpu.target_addr = 0x0100 | u16::from(cpu.sp);
            cpu.a = cpu.read(cpu.target_addr)?;
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
            cpu.read(cpu.pc)?;
            cpu.step_cycle();
            Ok(())
        }
        3 => {
            cpu.increment_sp();
            cpu.step_cycle();
            Ok(())
        }
        4 => {
            cpu.target_addr = 0x0100 | u16::from(cpu.sp);
            let mut status = cpu.read(cpu.target_addr)?;

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

// ------------------------------------------------------------------
// JUMP INSTRUCTIONS

// These instructions don't have a template to go off of,
// so these are the raw implementations

pub fn jmp_op_absolute(cpu: &mut CPU) -> Result<(), String> {
    match cpu.cycle_step {
        2 => {
            cpu.low_buffer = cpu.read(cpu.pc)?;
            cpu.increment_pc();
            cpu.step_cycle();
            Ok(())
        }
        3 => {
            cpu.high_buffer = cpu.read(cpu.pc)?;
            cpu.pc = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
            cpu.finish_instruction();
            Ok(())
        }
        _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
    }
}

pub fn jmp_op_indirect(cpu: &mut CPU) -> Result<(), String> {
    match cpu.cycle_step {
        2 => {
            cpu.low_buffer = cpu.read(cpu.pc)?;
            cpu.increment_pc();
            cpu.step_cycle();
            Ok(())
        }
        3 => {
            cpu.high_buffer = cpu.read(cpu.pc)?;
            cpu.increment_pc();
            cpu.step_cycle();
            Ok(())
        }
        4 => {
            cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
            cpu.low_buffer = cpu.read(cpu.target_addr)?;
            cpu.step_cycle();
            Ok(())
        }
        5 => {
            let low_ptr = cpu.target_addr as u8;
            cpu.target_addr = (cpu.target_addr & 0xFF00) | (u16::from(low_ptr.wrapping_add(1)));
            cpu.high_buffer = cpu.read(cpu.target_addr)?;

            // Copy into PC
            cpu.pc = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
            cpu.finish_instruction();
            Ok(())
        }
        _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
    }
}

pub fn jsr_op(cpu: &mut CPU) -> Result<(), String> {
    match cpu.cycle_step {
        2 => {
            cpu.low_buffer = cpu.read(cpu.pc)?;
            cpu.pc = cpu.pc.wrapping_add(1);
            cpu.step_cycle();
            Ok(())
        }
        3 => {
            cpu.target_addr = 0x0100 | u16::from(cpu.sp);

            // Dummy read
            cpu.read(cpu.target_addr)?;
            cpu.step_cycle();
            Ok(())
        }
        4 => {
            let high_byte = (cpu.pc >> 8) as u8;
            cpu.write(cpu.target_addr, high_byte)?;
            cpu.decrement_sp();
            cpu.step_cycle();
            Ok(())
        }
        5 => {
            let low_byte = cpu.pc as u8;
            cpu.target_addr = 0x0100 | u16::from(cpu.sp);
            cpu.write(cpu.target_addr, low_byte)?;
            cpu.decrement_sp();
            cpu.step_cycle();
            Ok(())
        }
        6 => {
            cpu.high_buffer = cpu.read(cpu.pc)?;
            cpu.pc = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
            cpu.finish_instruction();
            Ok(())
        }
        _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
    }
}

pub fn rts_op(cpu: &mut CPU) -> Result<(), String> {
    match cpu.cycle_step {
        2 => {
            // Dummy read
            cpu.read(cpu.pc)?;
            cpu.step_cycle();
            Ok(())
        }
        3 => {
            cpu.increment_sp();
            cpu.step_cycle();
            Ok(())
        }
        4 => {
            cpu.target_addr = 0x0100 | u16::from(cpu.sp);
            let low_byte = cpu.read(cpu.target_addr)?;
            cpu.pc = (cpu.pc & 0xFF00) | u16::from(low_byte);
            cpu.increment_sp();
            cpu.step_cycle();
            Ok(())
        }
        5 => {
            cpu.target_addr = 0x0100 | u16::from(cpu.sp);
            let high_byte = cpu.read(cpu.target_addr)?;
            cpu.pc = (cpu.pc & 0xFF) | (u16::from(high_byte) << 8);
            cpu.step_cycle();
            Ok(())
        }
        6 => {
            cpu.pc = cpu.pc.wrapping_add(1);
            cpu.finish_instruction();
            Ok(())
        }
        _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
    }
}

pub fn rti_op(cpu: &mut CPU) -> Result<(), String> {
    match cpu.cycle_step {
        2 => {
            // Dummy read
            cpu.read(cpu.pc)?;
            cpu.step_cycle();
            Ok(())
        }
        3 => {
            cpu.increment_sp();
            cpu.step_cycle();
            Ok(())
        }
        4 => {
            cpu.target_addr = 0x0100 | u16::from(cpu.sp);
            let mut status = cpu.read(cpu.target_addr)?;

            // Clear break bit and set unused flag
            status &= 0b11101111;
            status |= 0b00100000;

            cpu.p = StatusRegister::from_bits_retain(status);
            cpu.increment_sp();
            cpu.step_cycle();
            Ok(())
        }
        5 => {
            cpu.target_addr = 0x0100 | u16::from(cpu.sp);
            let low_byte = cpu.read(cpu.target_addr)?;
            cpu.pc = (cpu.pc & 0xFF00) | u16::from(low_byte);
            cpu.increment_sp();
            cpu.step_cycle();
            Ok(())
        }
        6 => {
            cpu.target_addr = 0x0100 | u16::from(cpu.sp);
            let high_byte = cpu.read(cpu.target_addr)?;
            cpu.pc = (cpu.pc & 0xFF) | (u16::from(high_byte) << 8);
            cpu.finish_instruction();
            Ok(())
        }
        _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
    }
}

pub fn brk_op(cpu: &mut CPU) -> Result<(), String> {
    match cpu.cycle_step {
        2 => {
            // Dummy read
            cpu.read(cpu.pc)?;
            cpu.pc = cpu.pc.wrapping_add(1);
            cpu.step_cycle();
            Ok(())
        }
        3 => {
            let high_byte = (cpu.pc >> 8) as u8;
            cpu.target_addr = 0x0100 | u16::from(cpu.sp);
            cpu.write(cpu.target_addr, high_byte)?;
            cpu.decrement_sp();
            cpu.step_cycle();
            Ok(())
        }
        4 => {
            let low_byte = cpu.pc as u8;
            cpu.target_addr = 0x0100 | u16::from(cpu.sp);
            cpu.write(cpu.target_addr, low_byte)?;
            cpu.decrement_sp();
            cpu.step_cycle();
            Ok(())
        }
        5 => {
            let status = cpu.p.bits() | 0b00110000; // Set break and unused flag
            cpu.target_addr = 0x0100 | u16::from(cpu.sp);
            cpu.write(cpu.target_addr, status)?;
            cpu.decrement_sp();
            cpu.step_cycle();
            Ok(())
        }
        6 => {
            cpu.p.set(StatusRegister::INTERRUPT, true);
            cpu.low_buffer = cpu.read(0xFFFE)?;
            cpu.pc = (cpu.pc & 0xFF00) | u16::from(cpu.low_buffer);
            cpu.step_cycle();
            Ok(())
        }
        7 => {
            cpu.high_buffer = cpu.read(0xFFFF)?;
            cpu.pc = (cpu.pc & 0xFF) | u16::from(cpu.high_buffer) << 8;
            cpu.finish_instruction();
            Ok(())
        }
        _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
    }
}

// ------------------------------------------------------------------
// EXTRA INSTRUCTIONS

// No-op instruction has no functionality
pub fn nop_op(_cpu: &mut CPU) -> () {}

// ------------------------------------------------------------------
// INSTRUCTIONS TEST

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{cpu::CPU, cpubus::CPUBus};

    #[test]
    fn test_adc_op_overflow() {
        let mut bus = CPUBus::new();
        let mut cpu = CPU::new(&mut bus);

        cpu.a = 0x50;
        adc_op(&mut cpu, 0x50);

        // Assert value
        assert_eq!(cpu.a, 0xA0);

        // Assert the correct flags
        assert!(cpu.p.contains(StatusRegister::OVERFLOW));
        assert!(cpu.p.contains(StatusRegister::NEGATIVE));
        assert!(!cpu.p.contains(StatusRegister::ZERO));
        assert!(!cpu.p.contains(StatusRegister::CARRY));
    }

    #[test]
    fn test_adc_op_unsigned_carry() {
        let mut bus = CPUBus::new();
        let mut cpu = CPU::new(&mut bus);

        cpu.a = 0xFF;
        adc_op(&mut cpu, 0x01);

        // Assert value wrapped to zero
        assert_eq!(cpu.a, 0x00);

        // Assert the correct flags
        assert!(!cpu.p.contains(StatusRegister::OVERFLOW));
        assert!(!cpu.p.contains(StatusRegister::NEGATIVE));
        assert!(cpu.p.contains(StatusRegister::ZERO));
        assert!(cpu.p.contains(StatusRegister::CARRY));
    }

    #[test]
    fn test_adc_op_existing_carry() {
        let mut bus = CPUBus::new();
        let mut cpu = CPU::new(&mut bus);

        cpu.a = 0xFD;
        cpu.p.set(StatusRegister::CARRY, true);
        adc_op(&mut cpu, 0x01);

        // Assert value is correct
        assert_eq!(cpu.a, 0xFF);

        // Assert the correct flags
        assert!(!cpu.p.contains(StatusRegister::OVERFLOW));
        assert!(cpu.p.contains(StatusRegister::NEGATIVE));
        assert!(!cpu.p.contains(StatusRegister::ZERO));
        assert!(!cpu.p.contains(StatusRegister::CARRY));
    }

    #[test]
    fn test_sbc_op_borrow() {
        let mut bus = CPUBus::new();
        let mut cpu = CPU::new(&mut bus);

        // C = 1 means no incoming borrow
        cpu.a = 0x50;
        cpu.p.set(StatusRegister::CARRY, true);
        sbc_op(&mut cpu, 0xF0);

        // 0x50 - 0xF0 = 0x60, but a borrow is generated
        assert_eq!(cpu.a, 0x60);

        // Assert the correct flags
        assert!(!cpu.p.contains(StatusRegister::OVERFLOW));
        assert!(!cpu.p.contains(StatusRegister::NEGATIVE));
        assert!(!cpu.p.contains(StatusRegister::ZERO));
        assert!(!cpu.p.contains(StatusRegister::CARRY));
    }

    #[test]
    fn test_bit_op_nv_from_operand() {
        let mut bus = CPUBus::new();
        let mut cpu = CPU::new(&mut bus);

        // A & value == 0, but bits 7 and 6 of the operand are set
        cpu.a = 0x00;
        bit_op(&mut cpu, 0xC0);

        // Assert accumulator is unchanged
        assert_eq!(cpu.a, 0x00);

        // N and V are copied from operand bits 7 and 6, not from A & value
        assert!(cpu.p.contains(StatusRegister::NEGATIVE));
        assert!(cpu.p.contains(StatusRegister::OVERFLOW));
        assert!(cpu.p.contains(StatusRegister::ZERO));
    }

    #[test]
    fn test_bit_op_zero_from_and() {
        let mut bus = CPUBus::new();
        let mut cpu = CPU::new(&mut bus);

        // Same operand bits 7/6, but A & value != 0
        cpu.a = 0xC0;
        bit_op(&mut cpu, 0xC0);

        // Assert accumulator is unchanged
        assert_eq!(cpu.a, 0xC0);

        // N and V still come from the operand; Z only reflects A & value
        assert!(cpu.p.contains(StatusRegister::NEGATIVE));
        assert!(cpu.p.contains(StatusRegister::OVERFLOW));
        assert!(!cpu.p.contains(StatusRegister::ZERO));
    }

    #[test]
    fn test_cmp_op_register_greater() {
        let mut bus = CPUBus::new();
        let mut cpu = CPU::new(&mut bus);

        cpu.a = 0x50;
        cmp_op(&mut cpu, 0x20);

        // Assert accumulator is unchanged
        assert_eq!(cpu.a, 0x50);

        // Register > Memory: C = 1, Z = 0
        assert!(cpu.p.contains(StatusRegister::CARRY));
        assert!(!cpu.p.contains(StatusRegister::ZERO));
        assert!(!cpu.p.contains(StatusRegister::NEGATIVE));
    }

    #[test]
    fn test_cmp_op_register_equal() {
        let mut bus = CPUBus::new();
        let mut cpu = CPU::new(&mut bus);

        cpu.a = 0x50;
        cmp_op(&mut cpu, 0x50);

        // Assert accumulator is unchanged
        assert_eq!(cpu.a, 0x50);

        // Register == Memory: C = 1, Z = 1
        assert!(cpu.p.contains(StatusRegister::CARRY));
        assert!(cpu.p.contains(StatusRegister::ZERO));
        assert!(!cpu.p.contains(StatusRegister::NEGATIVE));
    }

    #[test]
    fn test_cmp_op_register_less() {
        let mut bus = CPUBus::new();
        let mut cpu = CPU::new(&mut bus);

        cpu.a = 0x20;
        cmp_op(&mut cpu, 0x50);

        // Assert accumulator is unchanged
        assert_eq!(cpu.a, 0x20);

        // Register < Memory: C = 0, Z = 0
        assert!(!cpu.p.contains(StatusRegister::CARRY));
        assert!(!cpu.p.contains(StatusRegister::ZERO));
        assert!(cpu.p.contains(StatusRegister::NEGATIVE));
    }

    #[test]
    fn test_rol_op_carry_through() {
        let mut bus = CPUBus::new();
        let mut cpu = CPU::new(&mut bus);

        // Bit 7 shifts into C; previous C shifts into bit 0
        cpu.p.set(StatusRegister::CARRY, true);
        let result = rol_op(&mut cpu, 0x80);

        // Assert value
        assert_eq!(result, 0x01);

        // Assert the correct flags
        assert!(cpu.p.contains(StatusRegister::CARRY));
        assert!(!cpu.p.contains(StatusRegister::NEGATIVE));
        assert!(!cpu.p.contains(StatusRegister::ZERO));
    }

    #[test]
    fn test_ror_op_carry_through() {
        let mut bus = CPUBus::new();
        let mut cpu = CPU::new(&mut bus);

        // Bit 0 shifts into C; previous C shifts into bit 7
        cpu.p.set(StatusRegister::CARRY, true);
        let result = ror_op(&mut cpu, 0x01);

        // Assert value
        assert_eq!(result, 0x80);

        // Assert the correct flags
        assert!(cpu.p.contains(StatusRegister::CARRY));
        assert!(cpu.p.contains(StatusRegister::NEGATIVE));
        assert!(!cpu.p.contains(StatusRegister::ZERO));
    }

    #[test]
    fn test_jmp_op_indirect_page_wrap() {
        let mut bus = CPUBus::new();
        let mut cpu = CPU::new(&mut bus);

        // JMP ($02FF): low byte at $02FF, high byte wraps to $0200
        cpu.pc = 0x0000;
        cpu.cycle_step = 2;
        cpu.write(0x0000, 0xFF).unwrap();
        cpu.write(0x0001, 0x02).unwrap();
        cpu.write(0x02FF, 0x34).unwrap();
        cpu.write(0x0200, 0x12).unwrap();
        cpu.write(0x0300, 0xAB).unwrap();

        jmp_op_indirect(&mut cpu).unwrap();
        jmp_op_indirect(&mut cpu).unwrap();
        jmp_op_indirect(&mut cpu).unwrap();
        jmp_op_indirect(&mut cpu).unwrap();

        // PC must come from $0200, not $0300
        assert_eq!(cpu.pc, 0x1234);
        assert_eq!(cpu.cycle_step, 0);
    }

    #[test]
    fn test_jsr_op_pushes_third_byte() {
        let mut bus = CPUBus::new();
        let mut cpu = CPU::new(&mut bus);

        // JSR $1234 at $01FF: operands at $0200/$0201; third byte is $0201
        cpu.pc = 0x0200;
        cpu.sp = 0xFD;
        cpu.cycle_step = 2;
        cpu.write(0x0200, 0x34).unwrap();
        cpu.write(0x0201, 0x12).unwrap();

        jsr_op(&mut cpu).unwrap();
        jsr_op(&mut cpu).unwrap();
        jsr_op(&mut cpu).unwrap();
        jsr_op(&mut cpu).unwrap();
        jsr_op(&mut cpu).unwrap();

        // Pushed PC - 1 (address of the third byte), SP decremented twice
        assert_eq!(cpu.read(0x01FD).unwrap(), 0x02);
        assert_eq!(cpu.read(0x01FC).unwrap(), 0x01);
        assert_eq!(cpu.sp, 0xFB);
        assert_eq!(cpu.pc, 0x1234);
        assert_eq!(cpu.cycle_step, 0);
    }

    #[test]
    fn test_rts_op_pulls_and_increments_pc() {
        let mut bus = CPUBus::new();
        let mut cpu = CPU::new(&mut bus);

        // Stack holds the JSR-style return address $0201
        cpu.pc = 0x0000;
        cpu.sp = 0xFB;
        cpu.cycle_step = 2;
        cpu.write(0x01FC, 0x01).unwrap();
        cpu.write(0x01FD, 0x02).unwrap();

        rts_op(&mut cpu).unwrap();
        rts_op(&mut cpu).unwrap();
        rts_op(&mut cpu).unwrap();
        rts_op(&mut cpu).unwrap();
        rts_op(&mut cpu).unwrap();

        // Reconstructs $0201, then adds +1; SP incremented twice
        assert_eq!(cpu.pc, 0x0202);
        assert_eq!(cpu.sp, 0xFD);
        assert_eq!(cpu.cycle_step, 0);
    }

    #[test]
    fn test_php_op_sets_break_and_unused() {
        let mut bus = CPUBus::new();
        let mut cpu = CPU::new(&mut bus);

        cpu.pc = 0x0000;
        cpu.sp = 0xFD;
        cpu.cycle_step = 2;
        cpu.p.set(StatusRegister::CARRY, true);

        php_op(&mut cpu).unwrap();
        php_op(&mut cpu).unwrap();

        let pushed = cpu.read(0x01FD).unwrap();

        // Bit 4 (B) and bit 5 (unused) are forced set; other flags are preserved
        assert_eq!(pushed & 0b00110000, 0b00110000);
        assert_eq!(pushed & 0b00000001, 0b00000001);
        assert_eq!(cpu.sp, 0xFC);
        assert_eq!(cpu.cycle_step, 0);
    }

    #[test]
    fn test_plp_op_clears_break_sets_unused() {
        let mut bus = CPUBus::new();
        let mut cpu = CPU::new(&mut bus);

        // Stack byte has B set and unused clear
        cpu.pc = 0x0000;
        cpu.sp = 0xFC;
        cpu.cycle_step = 2;
        cpu.write(0x01FD, 0b11011111).unwrap();

        plp_op(&mut cpu).unwrap();
        plp_op(&mut cpu).unwrap();
        plp_op(&mut cpu).unwrap();

        // B is cleared, unused is forced set, remaining flags come from the stack
        assert!(!cpu.p.contains(StatusRegister::BREAK));
        assert_eq!(cpu.p.bits() & 0b00100000, 0b00100000);
        assert!(cpu.p.contains(StatusRegister::NEGATIVE));
        assert!(cpu.p.contains(StatusRegister::OVERFLOW));
        assert!(cpu.p.contains(StatusRegister::CARRY));
        assert_eq!(cpu.sp, 0xFD);
        assert_eq!(cpu.cycle_step, 0);
    }

    #[test]
    fn test_brk_op_stack_and_vector() {
        let mut bus = CPUBus::new();
        let mut cpu = CPU::new(&mut bus);

        // BRK at $0200: after the opcode fetch, PC is $0201 and BRK pushes PC + 2
        cpu.pc = 0x0201;
        cpu.sp = 0xFD;
        cpu.cycle_step = 2;
        cpu.p.set(StatusRegister::CARRY, true);
        cpu.write(0xFFFE, 0x00).unwrap();
        cpu.write(0xFFFF, 0x03).unwrap();

        brk_op(&mut cpu).unwrap();
        brk_op(&mut cpu).unwrap();
        brk_op(&mut cpu).unwrap();
        brk_op(&mut cpu).unwrap();
        brk_op(&mut cpu).unwrap();
        brk_op(&mut cpu).unwrap();

        // Pushed PC + 2 ($0202), then P with B and unused set
        assert_eq!(cpu.read(0x01FD).unwrap(), 0x02);
        assert_eq!(cpu.read(0x01FC).unwrap(), 0x02);
        assert_eq!(cpu.read(0x01FB).unwrap() & 0b00110000, 0b00110000);
        assert_eq!(cpu.read(0x01FB).unwrap() & 0b00000001, 0b00000001);

        // I is set on the live register; PC comes from $FFFE/$FFFF
        assert!(cpu.p.contains(StatusRegister::INTERRUPT));
        assert_eq!(cpu.pc, 0x0300);
        assert_eq!(cpu.sp, 0xFA);
        assert_eq!(cpu.cycle_step, 0);
    }
}