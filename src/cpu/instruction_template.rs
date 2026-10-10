/***
 * Related generic instruction implementations for the CPU
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
pub(super) fn absolute_addr_r<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU, u8) -> ()
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                cpu.high_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            4 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let value = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
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
/// 
pub(super) fn absolute_addr_rmw<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU, u8) -> u8
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                cpu.high_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
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

                cpu.alu_buffer = operation(cpu, cpu.alu_buffer);
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
/// Operation returns the value of the register (STA, STX, ...)
/// 
pub(super) fn absolute_addr_w<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String>
where
    F: Fn(&mut CPU) -> u8
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                cpu.high_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            4 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let register = operation(cpu);
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
pub(super) fn zero_page_addr_r<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU, u8) -> ()
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                let value = cpu.bus.read(u16::from(cpu.low_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
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
/// 
pub(super) fn zero_page_addr_rmw<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU, u8) -> u8
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
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

                cpu.alu_buffer = operation(cpu, cpu.alu_buffer);
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
pub(super) fn zero_page_addr_w<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU) -> u8
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                let register = operation(cpu);
                cpu.bus.write(u16::from(cpu.low_buffer), register)
                    .map_err(|err| format!("Error writing to CPU VRAM: {err}"))?;
                cpu.finish_instruction();
                Ok(())
            }
            _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
        }
    }
}
// ------------------------------------------------------------------
// ZERO-PAGE INDEXED X ADDRESSING

///
/// Performs read operations for zero-page indexed x addressing.
/// 
pub(super) fn zero_page_indexed_x_addr_r<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU, u8) -> ()
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                // Dummy Read
                cpu.bus.read(u16::from(cpu.low_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;

                cpu.target_addr = u16::from(cpu.low_buffer.wrapping_add(cpu.x));
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                let value = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                operation(cpu, value);
                cpu.finish_instruction();
                Ok(())
            }
            _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
        }
    }
}

///
/// Performs read/modify/write operations for zero-page indexed x addressing.
/// 
pub(super) fn zero_page_indexed_x_addr_rmw<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU, u8) -> u8
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                // Dummy Read
                cpu.bus.read(u16::from(cpu.low_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;

                cpu.target_addr = u16::from(cpu.low_buffer.wrapping_add(cpu.x));
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.alu_buffer = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                // Peform the dummy write first
                cpu.bus.write(cpu.target_addr, cpu.alu_buffer)
                    .map_err(|err| format!("Error writing to CPU VRAM: {err}"))?;

                cpu.alu_buffer = operation(cpu, cpu.alu_buffer);
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
/// Performs write operations for zero-page indexed x addressing.
/// 
pub(super) fn zero_page_indexed_x_addr_w<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU) -> u8
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                // Dummy Read
                cpu.bus.read(u16::from(cpu.low_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;

                cpu.target_addr = u16::from(cpu.low_buffer.wrapping_add(cpu.x));
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                let register = operation(cpu);
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
// ZERO-PAGE INDEXED Y ADDRESSING

///
/// Performs read operations for zero-page indexed y addressing.
/// 
pub(super) fn zero_page_indexed_y_addr_r<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU, u8) -> ()
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                // Dummy Read
                cpu.bus.read(u16::from(cpu.low_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;

                cpu.target_addr = u16::from(cpu.low_buffer.wrapping_add(cpu.y));
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                let value = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                operation(cpu, value);
                cpu.finish_instruction();
                Ok(())
            }
            _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
        }
    }
}

///
/// Performs read/modify/write operations for zero-page indexed y addressing.
/// 
pub(super) fn zero_page_indexed_y_addr_rmw<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU, u8) -> u8
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                // Dummy Read
                cpu.bus.read(u16::from(cpu.low_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;

                cpu.target_addr = u16::from(cpu.low_buffer.wrapping_add(cpu.y));
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.alu_buffer = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                // Peform the dummy write first
                cpu.bus.write(cpu.target_addr, cpu.alu_buffer)
                    .map_err(|err| format!("Error writing to CPU VRAM: {err}"))?;

                cpu.alu_buffer = operation(cpu, cpu.alu_buffer);
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
/// Performs write operations for zero-page indexed y addressing.
/// 
pub(super) fn zero_page_indexed_y_addr_w<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU) -> u8
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                // Dummy Read
                cpu.bus.read(u16::from(cpu.low_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;

                cpu.target_addr = u16::from(cpu.low_buffer.wrapping_add(cpu.y));
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                let register = operation(cpu);
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
// ABSOLUTE INDEXED X ADDRESSING

///
/// Performs read operations for absolute indexed X addressing.
/// 
pub(super) fn absolute_indexed_x_addr_r<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU, u8) -> ()
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                // Get high buffer while incrementing lower by index
                cpu.high_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                (cpu.low_buffer, cpu.page_boundary_cross) = cpu.low_buffer.overflowing_add(cpu.x);
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            4 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let value = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                
                // If page boundary cross, overflow and re-read later
                if cpu.page_boundary_cross {
                    cpu.high_buffer = cpu.high_buffer.wrapping_add(1);
                    cpu.step_cycle();
                } else {
                    operation(cpu, value);
                    cpu.finish_instruction();
                }

                Ok(())
            }
            5 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let value = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                operation(cpu, value);
                cpu.finish_instruction();
                Ok(())
            }
            _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
        }
    }
}

///
/// Performs read/modify/write operations for absolute indexed X addressing.
/// 
pub(super) fn absolute_indexed_x_addr_rmw<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU, u8) -> u8
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                cpu.high_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                (cpu.low_buffer, cpu.page_boundary_cross) = cpu.low_buffer.overflowing_add(cpu.x);
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            4 => {
                // Dummy read
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;

                // If page boundary cross, fix high byte and read later
                if cpu.page_boundary_cross {
                    cpu.high_buffer = cpu.high_buffer.wrapping_add(1);
                }

                cpu.step_cycle();
                Ok(())
            }
            5 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.alu_buffer = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                Ok(())
            }
            6 => {
                // Peform the dummy write first
                cpu.bus.write(cpu.target_addr, cpu.alu_buffer)
                    .map_err(|err| format!("Error writing to CPU VRAM: {err}"))?;

                cpu.alu_buffer = operation(cpu, cpu.alu_buffer);
                cpu.step_cycle();
                Ok(())
            }
            7 => {
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
/// Performs write operations for absolute indexed X addressing.
/// Operation returns the value of the register (STA, STX, ...)
/// 
pub(super) fn absolute_indexed_x_addr_w<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String>
where
    F: Fn(&mut CPU) -> u8
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                cpu.high_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                (cpu.low_buffer, cpu.page_boundary_cross) = cpu.low_buffer.overflowing_add(cpu.x);
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            4 => {
                // Dummy read
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;

                // If page boundary cross, fix high byte and read later
                if cpu.page_boundary_cross {
                    cpu.high_buffer = cpu.high_buffer.wrapping_add(1);
                }

                cpu.step_cycle();
                Ok(())
            }
            5 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let register = operation(cpu);
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
// ABSOLUTE INDEXED Y ADDRESSING

///
/// Performs read operations for absolute indexed Y addressing.
/// 
pub(super) fn absolute_indexed_y_addr_r<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU, u8) -> ()
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                // Get high buffer while incrementing lower by index
                cpu.high_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                (cpu.low_buffer, cpu.page_boundary_cross) = cpu.low_buffer.overflowing_add(cpu.y);
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            4 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let value = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                
                // If page boundary cross, overflow and re-read later
                if cpu.page_boundary_cross {
                    cpu.high_buffer = cpu.high_buffer.wrapping_add(1);
                    cpu.step_cycle();
                } else {
                    operation(cpu, value);
                    cpu.finish_instruction();
                }

                Ok(())
            }
            5 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let value = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                operation(cpu, value);
                cpu.finish_instruction();
                Ok(())
            }
            _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
        }
    }
}

///
/// Performs read/modify/write operations for absolute indexed Y addressing.
/// 
pub(super) fn absolute_indexed_y_addr_rmw<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU, u8) -> u8
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                cpu.high_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                (cpu.low_buffer, cpu.page_boundary_cross) = cpu.low_buffer.overflowing_add(cpu.y);
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            4 => {
                // Dummy read
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;

                // If page boundary cross, fix high byte and read later
                if cpu.page_boundary_cross {
                    cpu.high_buffer = cpu.high_buffer.wrapping_add(1);
                }

                cpu.step_cycle();
                Ok(())
            }
            5 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.alu_buffer = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                Ok(())
            }
            6 => {
                // Peform the dummy write first
                cpu.bus.write(cpu.target_addr, cpu.alu_buffer)
                    .map_err(|err| format!("Error writing to CPU VRAM: {err}"))?;

                cpu.alu_buffer = operation(cpu, cpu.alu_buffer);
                cpu.step_cycle();
                Ok(())
            }
            7 => {
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
/// Performs write operations for absolute indexed Y addressing.
/// Operation returns the value of the register (STA, STX, ...)
/// 
pub(super) fn absolute_indexed_y_addr_w<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String>
where
    F: Fn(&mut CPU) -> u8
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                cpu.high_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                (cpu.low_buffer, cpu.page_boundary_cross) = cpu.low_buffer.overflowing_add(cpu.y);
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            4 => {
                // Dummy read
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;

                // If page boundary cross, fix high byte and read later
                if cpu.page_boundary_cross {
                    cpu.high_buffer = cpu.high_buffer.wrapping_add(1);
                }

                cpu.step_cycle();
                Ok(())
            }
            5 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let register = operation(cpu);
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
// INDEXED INDIRECT ADDRESSING

///
/// Performs read operations for indexed indirect addressing.
/// 
pub(super) fn indexed_indirect_addr_r<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU, u8) -> ()
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                // Read pointer into buffer
                cpu.alu_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                // Dummy Read
                cpu.bus.read(u16::from(cpu.alu_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;

                cpu.alu_buffer = cpu.alu_buffer.wrapping_add(cpu.x);
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.low_buffer = cpu.bus.read(u16::from(cpu.alu_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.alu_buffer = cpu.alu_buffer.wrapping_add(1);
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                cpu.high_buffer = cpu.bus.read(u16::from(cpu.alu_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                Ok(())
            }
            6 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let value = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                operation(cpu, value);
                cpu.finish_instruction();
                Ok(())
            }
            _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
        }
    }
}

///
/// Performs read/modify/write operations for indexed indirect addressing.
/// 
pub(super) fn indexed_indirect_addr_rmw<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU, u8) -> u8
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                // Read pointer into buffer
                cpu.alu_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                // Dummy Read
                cpu.bus.read(u16::from(cpu.alu_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;

                cpu.alu_buffer = cpu.alu_buffer.wrapping_add(cpu.x);
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.low_buffer = cpu.bus.read(u16::from(cpu.alu_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.alu_buffer = cpu.alu_buffer.wrapping_add(1);
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                cpu.high_buffer = cpu.bus.read(u16::from(cpu.alu_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                Ok(())
            }
            6 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.alu_buffer = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                Ok(())
            }
            7 => {
                // Peform the dummy write first
                cpu.bus.write(cpu.target_addr, cpu.alu_buffer)
                    .map_err(|err| format!("Error writing to CPU VRAM: {err}"))?;

                cpu.alu_buffer = operation(cpu, cpu.alu_buffer);
                cpu.step_cycle();
                Ok(())
            }
            8 => {
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
/// Performs write operations for indexed indirect addressing.
/// 
pub(super) fn indexed_indirect_addr_w<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU) -> u8
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                // Read pointer into buffer
                cpu.alu_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                // Dummy Read
                cpu.bus.read(u16::from(cpu.alu_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;

                cpu.alu_buffer = cpu.alu_buffer.wrapping_add(cpu.x);
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.low_buffer = cpu.bus.read(u16::from(cpu.alu_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.alu_buffer = cpu.alu_buffer.wrapping_add(1);
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                cpu.high_buffer = cpu.bus.read(u16::from(cpu.alu_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                Ok(())
            }
            6 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let register = operation(cpu);
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
// INDIRECT INDEXED ADDRESSING

///
/// Performs read operations for indirect indexed addressing.
/// 
pub(super) fn indirect_indexed_addr_r<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU, u8) -> ()
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                // Read pointer into buffer
                cpu.alu_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                cpu.low_buffer = cpu.bus.read(u16::from(cpu.alu_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.alu_buffer = cpu.alu_buffer.wrapping_add(1);
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.high_buffer = cpu.bus.read(u16::from(cpu.alu_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                (cpu.low_buffer, cpu.page_boundary_cross) = cpu.low_buffer.overflowing_add(cpu.y);
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let value = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                
                // If page boundary cross, overflow and re-read later
                if cpu.page_boundary_cross {
                    cpu.high_buffer = cpu.high_buffer.wrapping_add(1);
                    cpu.step_cycle();
                } else {
                    operation(cpu, value);
                    cpu.finish_instruction();
                }

                Ok(())
            }
            6 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let value = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                operation(cpu, value);
                cpu.finish_instruction();
                Ok(())
            }
            _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
        }
    }
}

///
/// Performs read/modify/write operations for indirect indexed addressing.
/// 
pub(super) fn indirect_indexed_addr_rmw<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU, u8) -> u8
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                // Read pointer into buffer
                cpu.alu_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                cpu.low_buffer = cpu.bus.read(u16::from(cpu.alu_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.alu_buffer = cpu.alu_buffer.wrapping_add(1);
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.high_buffer = cpu.bus.read(u16::from(cpu.alu_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                (cpu.low_buffer, cpu.page_boundary_cross) = cpu.low_buffer.overflowing_add(cpu.y);
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                // Dummy read
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;

                // If page boundary cross, fix high byte and read later
                if cpu.page_boundary_cross {
                    cpu.high_buffer = cpu.high_buffer.wrapping_add(1);
                }

                cpu.step_cycle();
                Ok(())
            }
            6 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.alu_buffer = cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                Ok(())
            }
            7 => {
                // Peform the dummy write first
                cpu.bus.write(cpu.target_addr, cpu.alu_buffer)
                    .map_err(|err| format!("Error writing to CPU VRAM: {err}"))?;

                cpu.alu_buffer = operation(cpu, cpu.alu_buffer);
                cpu.step_cycle();
                Ok(())
            }
            8 => {
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
/// Performs write operations for indirect indexed addressing.
/// 
pub(super) fn indirect_indexed_addr_w<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String> 
where 
    F: Fn(&mut CPU) -> u8
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                // Read pointer into buffer
                cpu.alu_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.step_cycle();
                cpu.increment_pc();
                Ok(())
            }
            3 => {
                cpu.low_buffer = cpu.bus.read(u16::from(cpu.alu_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.alu_buffer = cpu.alu_buffer.wrapping_add(1);
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.high_buffer = cpu.bus.read(u16::from(cpu.alu_buffer))
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                (cpu.low_buffer, cpu.page_boundary_cross) = cpu.low_buffer.overflowing_add(cpu.y);
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                // Dummy read
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.bus.read(cpu.target_addr)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;

                // If page boundary cross, fix high byte and read later
                if cpu.page_boundary_cross {
                    cpu.high_buffer = cpu.high_buffer.wrapping_add(1);
                }

                cpu.step_cycle();
                Ok(())
            }
            6 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let register = operation(cpu);
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
// IMMEDIATE ADDRESSING

///
/// Performs read operation with immediate addressing
/// 
pub(super) fn immediate_addr<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String>
where 
    F: Fn(&mut CPU, u8) -> ()
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                let value = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                operation(cpu, value);
                cpu.increment_pc();
                cpu.finish_instruction();
                Ok(())
            }
            _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
        }
    }
}

// ------------------------------------------------------------------
// ACCUMULATOR / IMPLIED ADDRESSING

///
/// Performs read operation with accumulator addressing
/// 
pub(super) fn accumulator_addr<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String>
where 
    F: Fn(&mut CPU, u8) -> u8
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.a = operation(cpu, cpu.a);
                cpu.finish_instruction();
                Ok(())
            }
            _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
        }
    }
}

pub(super) fn implied_addr<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String>
where 
    F: Fn(&mut CPU) -> ()
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                operation(cpu);
                cpu.finish_instruction();
                Ok(())
            }
            _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
        }
    }
}

// ------------------------------------------------------------------
// RELATIVE ADDRESSING

///
/// Performs branch operations with relative addressing
/// 
pub(super) fn relative_addr<F>(operation: F) -> impl Fn(&mut CPU) -> Result<(), String>
where 
    F: Fn(&mut CPU) -> bool
{
    move |cpu| {
        match cpu.cycle_step {
            2 => {
                cpu.low_buffer = cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;
                cpu.increment_pc();
                
                // Check if branched
                if operation(cpu) {
                    cpu.step_cycle();
                } else {
                    cpu.finish_instruction();
                }

                Ok(())
            }
            3 => {
                // Dummy read
                cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;

                let offset = cpu.low_buffer as i8 as u16;
                let target_pc = cpu.pc.wrapping_add(offset);
                cpu.page_boundary_cross = (cpu.pc & 0xFF00) != (target_pc & 0xFF00);
                cpu.pc = (cpu.pc & 0xFF00) | (target_pc & 0x00FF);

                if cpu.page_boundary_cross {
                    cpu.high_buffer = (target_pc >> 8) as u8;
                    cpu.step_cycle();
                } else {
                    cpu.finish_instruction();
                }
                Ok(())
            }
            4 => {
                // Dummy read
                cpu.bus.read(cpu.pc)
                    .map_err(|err| format!("Error reading from CPU VRAM: {err}"))?;

                cpu.pc = (cpu.pc & 0x00FF) | (u16::from(cpu.high_buffer) << 8);
                cpu.finish_instruction();
                Ok(())
            }
            _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
        }
    }
}