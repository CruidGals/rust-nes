/***
 * Related instructions for the CPU
 */

use std::fmt::format;

use super::cpu::CPU;

// ==================================================================
// BASE INSTRUCTIONS FOR ADDRESSING MODES
//
// Each addressing mode contain relatively similar implementations
// when comparing their cycle-by-cycle breakdown. To simplify matters,
// generic functions will be made to encapsulate functionality of
// later instructions
//===================================================================


// ------------------------------------------------------------------
// ABSOLUTE ADDRESSING

///
/// Performs read operations for absolute addressing.
/// 
fn absolute_addr_r<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU, u8) -> ()
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Problem reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.pc += 1;
                Ok(())
            }
            3 => {
                cpu.high_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Problem reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.pc += 1;
                Ok(())
            }
            4 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let value = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Problem reading from CPU VRAM: {err}"))?;
                operation(cpu, value);
                cpu.finish_instruction();
                Ok(())
            }
            _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
        }
    }
}

///
/// Performs read/modify/write operations for absolute addressing.
/// Functions using this must operate on cpu.alu_buffer
/// 
fn absolute_addr_rmw<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU) -> ()
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Problem reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.pc += 1;
                Ok(())
            }
            3 => {
                cpu.high_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Problem reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.pc += 1;
                Ok(())
            }
            4 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.alu_buffer = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                // Peform the dummy write first
                cpu.bus.write(cpu.target_addr, cpu.alu_buffer)
                    .map_err(|err| format!("Error writing to CPU VRAM: {err}"))?;

                operation(cpu);
                cpu.step_cycle();
                Ok(())
            }
            6 => {
                cpu.bus.write(cpu.target_addr, cpu.alu_buffer)
                    .map_err(|err| format!("Error writing to CPU VRAM: {err}"))?;
                cpu.finish_instruction();
                Ok(())
            }
            _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
        }
    }
}

///
/// Performs write operations for absolute addressing.
/// 
fn absolute_addr_w() -> impl Fn(&mut CPU, u8) -> Result<(), String> 
{
    move |cpu, register| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Problem reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.pc += 1;
                Ok(())
            }
            3 => {
                cpu.high_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Problem reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.pc += 1;
                Ok(())
            }
            4 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.bus.write(cpu.target_addr, register)
                    .map_err(|err| format!("Error writing to CPU VRAM: {err}"))?;
                cpu.finish_instruction();
                Ok(())
            }
            _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
        }
    }
}

// ------------------------------------------------------------------
// ZERO-PAGE ADDRESSING

///
/// Performs read operations for zero-page addressing.
/// 
fn zero_page_addr_r<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU, u8) -> ()
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Problem reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.pc += 1;
                Ok(())
            }
            3 => {
                let value = cpu.bus.read(u16::from(cpu.low_buffer))
                    .map_err(|err| format!("Problem reading from CPU VRAM: {err}"))?;
                operation(cpu, value);
                cpu.finish_instruction();
                Ok(())
            }
            _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
        }
    }
}

///
/// Performs read/modify/write operations for zero-page addressing.
/// Functions using this must operate on cpu.alu_buffer
/// 
fn zero_page_addr_rmw<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU) -> ()
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Problem reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.pc += 1;
                Ok(())
            }
            3 => {
                cpu.target_addr = u16::from(cpu.low_buffer);
                cpu.alu_buffer = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                // Peform the dummy write first
                cpu.bus.write(cpu.target_addr, cpu.alu_buffer)
                    .map_err(|err| format!("Error writing to CPU VRAM: {err}"))?;

                operation(cpu);
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                cpu.bus.write(cpu.target_addr, cpu.alu_buffer)
                    .map_err(|err| format!("Error writing to CPU VRAM: {err}"))?;
                cpu.finish_instruction();
                Ok(())
            }
            _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
        }
    }
}

///
/// Performs write operations for zero-page addressing.
/// 
fn zero_page_addr_w() -> impl Fn(&mut CPU, u8) -> Result<(), String> 
{
    move |cpu, register| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Problem reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.pc += 1;
                Ok(())
            }
            3 => {
                cpu.bus.write(u16::from(cpu.low_buffer), register)
                    .map_err(|err| format!("Error writing to CPU VRAM: {err}"))?;
                cpu.finish_instruction();
                Ok(())
            }
            _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
        }
    }
}