/***
 * Related generic instruction implementations for the CPU
 */

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
                cpu.low_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                cpu.high_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let value = cpu.read(cpu.target_addr)?;
                operation(cpu, value);
                cpu.finish_instruction()?;
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
                cpu.low_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                cpu.high_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.alu_buffer = cpu.read(cpu.target_addr)?;
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                // Peform the dummy write first
                cpu.write(cpu.target_addr, cpu.alu_buffer)?;

                cpu.alu_buffer = operation(cpu, cpu.alu_buffer);
                cpu.step_cycle();
                Ok(())
            }
            6 => {
                cpu.write(cpu.target_addr, cpu.alu_buffer)?;
                cpu.finish_instruction()?;
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
                cpu.low_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                cpu.high_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let register = operation(cpu);
                cpu.write(cpu.target_addr, register)?;
                cpu.finish_instruction()?;
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
                cpu.low_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                let value = cpu.read(u16::from(cpu.low_buffer))?;
                operation(cpu, value);
                cpu.finish_instruction()?;
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
                cpu.low_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                cpu.target_addr = u16::from(cpu.low_buffer);
                cpu.alu_buffer = cpu.read(cpu.target_addr)?;
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                // Peform the dummy write first
                cpu.write(cpu.target_addr, cpu.alu_buffer)?;

                cpu.alu_buffer = operation(cpu, cpu.alu_buffer);
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                cpu.write(cpu.target_addr, cpu.alu_buffer)?;
                cpu.finish_instruction()?;
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
                cpu.low_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                let register = operation(cpu);
                cpu.write(u16::from(cpu.low_buffer), register)?;
                cpu.finish_instruction()?;
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
                cpu.low_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                // Dummy Read
                cpu.read(u16::from(cpu.low_buffer))?;

                cpu.target_addr = u16::from(cpu.low_buffer.wrapping_add(cpu.x));
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                let value = cpu.read(cpu.target_addr)?;
                operation(cpu, value);
                cpu.finish_instruction()?;
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
                cpu.low_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                // Dummy Read
                cpu.read(u16::from(cpu.low_buffer))?;

                cpu.target_addr = u16::from(cpu.low_buffer.wrapping_add(cpu.x));
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.alu_buffer = cpu.read(cpu.target_addr)?;
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                // Peform the dummy write first
                cpu.write(cpu.target_addr, cpu.alu_buffer)?;

                cpu.alu_buffer = operation(cpu, cpu.alu_buffer);
                cpu.step_cycle();
                Ok(())
            }
            6 => {
                cpu.write(cpu.target_addr, cpu.alu_buffer)?;
                cpu.finish_instruction()?;
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
                cpu.low_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                // Dummy Read
                cpu.read(u16::from(cpu.low_buffer))?;

                cpu.target_addr = u16::from(cpu.low_buffer.wrapping_add(cpu.x));
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                let register = operation(cpu);
                cpu.write(cpu.target_addr, register)?;
                cpu.finish_instruction()?;
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
                cpu.low_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                // Dummy Read
                cpu.read(u16::from(cpu.low_buffer))?;

                cpu.target_addr = u16::from(cpu.low_buffer.wrapping_add(cpu.y));
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                let value = cpu.read(cpu.target_addr)?;
                operation(cpu, value);
                cpu.finish_instruction()?;
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
                cpu.low_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                // Dummy Read
                cpu.read(u16::from(cpu.low_buffer))?;

                cpu.target_addr = u16::from(cpu.low_buffer.wrapping_add(cpu.y));
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.alu_buffer = cpu.read(cpu.target_addr)?;
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                // Peform the dummy write first
                cpu.write(cpu.target_addr, cpu.alu_buffer)?;

                cpu.alu_buffer = operation(cpu, cpu.alu_buffer);
                cpu.step_cycle();
                Ok(())
            }
            6 => {
                cpu.write(cpu.target_addr, cpu.alu_buffer)?;
                cpu.finish_instruction()?;
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
                cpu.low_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                // Dummy Read
                cpu.read(u16::from(cpu.low_buffer))?;

                cpu.target_addr = u16::from(cpu.low_buffer.wrapping_add(cpu.y));
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                let register = operation(cpu);
                cpu.write(cpu.target_addr, register)?;
                cpu.finish_instruction()?;
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
                cpu.low_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                // Get high buffer while incrementing lower by index
                cpu.high_buffer = cpu.fetch()?;
                (cpu.low_buffer, cpu.page_boundary_cross) = cpu.low_buffer.overflowing_add(cpu.x);
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let value = cpu.read(cpu.target_addr)?;
                
                // If page boundary cross, overflow and re-read later
                if cpu.page_boundary_cross {
                    cpu.high_buffer = cpu.high_buffer.wrapping_add(1);
                    cpu.step_cycle();
                } else {
                    operation(cpu, value);
                    cpu.finish_instruction()?;
                }

                Ok(())
            }
            5 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let value = cpu.read(cpu.target_addr)?;
                operation(cpu, value);
                cpu.finish_instruction()?;
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
                cpu.low_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                cpu.high_buffer = cpu.fetch()?;
                (cpu.low_buffer, cpu.page_boundary_cross) = cpu.low_buffer.overflowing_add(cpu.x);
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                // Dummy read
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.read(cpu.target_addr)?;

                // If page boundary cross, fix high byte and read later
                if cpu.page_boundary_cross {
                    cpu.high_buffer = cpu.high_buffer.wrapping_add(1);
                }

                cpu.step_cycle();
                Ok(())
            }
            5 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.alu_buffer = cpu.read(cpu.target_addr)?;
                cpu.step_cycle();
                Ok(())
            }
            6 => {
                // Peform the dummy write first
                cpu.write(cpu.target_addr, cpu.alu_buffer)?;

                cpu.alu_buffer = operation(cpu, cpu.alu_buffer);
                cpu.step_cycle();
                Ok(())
            }
            7 => {
                cpu.write(cpu.target_addr, cpu.alu_buffer)?;
                cpu.finish_instruction()?;
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
                cpu.low_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                cpu.high_buffer = cpu.fetch()?;
                (cpu.low_buffer, cpu.page_boundary_cross) = cpu.low_buffer.overflowing_add(cpu.x);
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                // Dummy read
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.read(cpu.target_addr)?;

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
                cpu.write(cpu.target_addr, register)?;
                cpu.finish_instruction()?;
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
                cpu.low_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                // Get high buffer while incrementing lower by index
                cpu.high_buffer = cpu.fetch()?;
                (cpu.low_buffer, cpu.page_boundary_cross) = cpu.low_buffer.overflowing_add(cpu.y);
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let value = cpu.read(cpu.target_addr)?;
                
                // If page boundary cross, overflow and re-read later
                if cpu.page_boundary_cross {
                    cpu.high_buffer = cpu.high_buffer.wrapping_add(1);
                    cpu.step_cycle();
                } else {
                    operation(cpu, value);
                    cpu.finish_instruction()?;
                }

                Ok(())
            }
            5 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let value = cpu.read(cpu.target_addr)?;
                operation(cpu, value);
                cpu.finish_instruction()?;
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
                cpu.low_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                cpu.high_buffer = cpu.fetch()?;
                (cpu.low_buffer, cpu.page_boundary_cross) = cpu.low_buffer.overflowing_add(cpu.y);
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                // Dummy read
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.read(cpu.target_addr)?;

                // If page boundary cross, fix high byte and read later
                if cpu.page_boundary_cross {
                    cpu.high_buffer = cpu.high_buffer.wrapping_add(1);
                }

                cpu.step_cycle();
                Ok(())
            }
            5 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.alu_buffer = cpu.read(cpu.target_addr)?;
                cpu.step_cycle();
                Ok(())
            }
            6 => {
                // Peform the dummy write first
                cpu.write(cpu.target_addr, cpu.alu_buffer)?;

                cpu.alu_buffer = operation(cpu, cpu.alu_buffer);
                cpu.step_cycle();
                Ok(())
            }
            7 => {
                cpu.write(cpu.target_addr, cpu.alu_buffer)?;
                cpu.finish_instruction()?;
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
                cpu.low_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                cpu.high_buffer = cpu.fetch()?;
                (cpu.low_buffer, cpu.page_boundary_cross) = cpu.low_buffer.overflowing_add(cpu.y);
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                // Dummy read
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.read(cpu.target_addr)?;

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
                cpu.write(cpu.target_addr, register)?;
                cpu.finish_instruction()?;
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
                cpu.alu_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                // Dummy Read
                cpu.read(u16::from(cpu.alu_buffer))?;

                cpu.alu_buffer = cpu.alu_buffer.wrapping_add(cpu.x);
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.low_buffer = cpu.read(u16::from(cpu.alu_buffer))?;
                cpu.alu_buffer = cpu.alu_buffer.wrapping_add(1);
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                cpu.high_buffer = cpu.read(u16::from(cpu.alu_buffer))?;
                cpu.step_cycle();
                Ok(())
            }
            6 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let value = cpu.read(cpu.target_addr)?;
                operation(cpu, value);
                cpu.finish_instruction()?;
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
                cpu.alu_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                // Dummy Read
                cpu.read(u16::from(cpu.alu_buffer))?;

                cpu.alu_buffer = cpu.alu_buffer.wrapping_add(cpu.x);
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.low_buffer = cpu.read(u16::from(cpu.alu_buffer))?;
                cpu.alu_buffer = cpu.alu_buffer.wrapping_add(1);
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                cpu.high_buffer = cpu.read(u16::from(cpu.alu_buffer))?;
                cpu.step_cycle();
                Ok(())
            }
            6 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.alu_buffer = cpu.read(cpu.target_addr)?;
                cpu.step_cycle();
                Ok(())
            }
            7 => {
                // Peform the dummy write first
                cpu.write(cpu.target_addr, cpu.alu_buffer)?;

                cpu.alu_buffer = operation(cpu, cpu.alu_buffer);
                cpu.step_cycle();
                Ok(())
            }
            8 => {
                cpu.write(cpu.target_addr, cpu.alu_buffer)?;
                cpu.finish_instruction()?;
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
                cpu.alu_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                // Dummy Read
                cpu.read(u16::from(cpu.alu_buffer))?;

                cpu.alu_buffer = cpu.alu_buffer.wrapping_add(cpu.x);
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.low_buffer = cpu.read(u16::from(cpu.alu_buffer))?;
                cpu.alu_buffer = cpu.alu_buffer.wrapping_add(1);
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                cpu.high_buffer = cpu.read(u16::from(cpu.alu_buffer))?;
                cpu.step_cycle();
                Ok(())
            }
            6 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let register = operation(cpu);
                cpu.write(cpu.target_addr, register)?;
                cpu.finish_instruction()?;
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
                cpu.alu_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                cpu.low_buffer = cpu.read(u16::from(cpu.alu_buffer))?;
                cpu.alu_buffer = cpu.alu_buffer.wrapping_add(1);
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.high_buffer = cpu.read(u16::from(cpu.alu_buffer))?;
                (cpu.low_buffer, cpu.page_boundary_cross) = cpu.low_buffer.overflowing_add(cpu.y);
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let value = cpu.read(cpu.target_addr)?;
                
                // If page boundary cross, overflow and re-read later
                if cpu.page_boundary_cross {
                    cpu.high_buffer = cpu.high_buffer.wrapping_add(1);
                    cpu.step_cycle();
                } else {
                    operation(cpu, value);
                    cpu.finish_instruction()?;
                }

                Ok(())
            }
            6 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                let value = cpu.read(cpu.target_addr)?;
                operation(cpu, value);
                cpu.finish_instruction()?;
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
                cpu.alu_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                cpu.low_buffer = cpu.read(u16::from(cpu.alu_buffer))?;
                cpu.alu_buffer = cpu.alu_buffer.wrapping_add(1);
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.high_buffer = cpu.read(u16::from(cpu.alu_buffer))?;
                (cpu.low_buffer, cpu.page_boundary_cross) = cpu.low_buffer.overflowing_add(cpu.y);
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                // Dummy read
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.read(cpu.target_addr)?;

                // If page boundary cross, fix high byte and read later
                if cpu.page_boundary_cross {
                    cpu.high_buffer = cpu.high_buffer.wrapping_add(1);
                }

                cpu.step_cycle();
                Ok(())
            }
            6 => {
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.alu_buffer = cpu.read(cpu.target_addr)?;
                cpu.step_cycle();
                Ok(())
            }
            7 => {
                // Peform the dummy write first
                cpu.write(cpu.target_addr, cpu.alu_buffer)?;

                cpu.alu_buffer = operation(cpu, cpu.alu_buffer);
                cpu.step_cycle();
                Ok(())
            }
            8 => {
                cpu.write(cpu.target_addr, cpu.alu_buffer)?;
                cpu.finish_instruction()?;
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
                cpu.alu_buffer = cpu.fetch()?;
                cpu.step_cycle();
                Ok(())
            }
            3 => {
                cpu.low_buffer = cpu.read(u16::from(cpu.alu_buffer))?;
                cpu.alu_buffer = cpu.alu_buffer.wrapping_add(1);
                cpu.step_cycle();
                Ok(())
            }
            4 => {
                cpu.high_buffer = cpu.read(u16::from(cpu.alu_buffer))?;
                (cpu.low_buffer, cpu.page_boundary_cross) = cpu.low_buffer.overflowing_add(cpu.y);
                cpu.step_cycle();
                Ok(())
            }
            5 => {
                // Dummy read
                cpu.target_addr = (u16::from(cpu.high_buffer) << 8) | u16::from(cpu.low_buffer);
                cpu.read(cpu.target_addr)?;

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
                cpu.write(cpu.target_addr, register)?;
                cpu.finish_instruction()?;
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
                let value = cpu.fetch()?;
                operation(cpu, value);
                cpu.finish_instruction()?;
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
                cpu.read(cpu.pc)?;
                cpu.a = operation(cpu, cpu.a);
                cpu.finish_instruction()?;
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
                cpu.read(cpu.pc)?;
                operation(cpu);
                cpu.finish_instruction()?;
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
                cpu.low_buffer = cpu.fetch()?;
                
                // Check if branched
                if operation(cpu) {
                    cpu.step_cycle();
                } else {
                    cpu.finish_instruction()?;
                }

                Ok(())
            }
            3 => {
                // Dummy read
                cpu.read(cpu.pc)?;

                let offset = cpu.low_buffer as i8 as u16;
                let target_pc = cpu.pc.wrapping_add(offset);
                cpu.page_boundary_cross = (cpu.pc & 0xFF00) != (target_pc & 0xFF00);
                cpu.pc = (cpu.pc & 0xFF00) | (target_pc & 0x00FF);

                if cpu.page_boundary_cross {
                    cpu.high_buffer = (target_pc >> 8) as u8;
                    cpu.step_cycle();
                } else {
                    cpu.finish_instruction()?;
                }
                Ok(())
            }
            4 => {
                // Dummy read
                cpu.read(cpu.pc)?;

                cpu.pc = (cpu.pc & 0x00FF) | (u16::from(cpu.high_buffer) << 8);
                cpu.finish_instruction()?;
                Ok(())
            }
            _ => Err(format!("cpu.cycle_step outside of its expected range: {}", cpu.cycle_step))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{cpu::{test_cartridge, CPU}, cpubus::CPUBus};
    use std::cell::Cell;

    #[test]
    fn test_absolute_indexed_x_addr_r_no_page_cross() {
        let bus = CPUBus::new(test_cartridge());
        let mut cpu = CPU::new(bus, false).unwrap();

        // Base $0100 + X = $05 → $0105, no page cross
        cpu.pc = 0x0000;
        cpu.x = 0x05;
        cpu.cycle_step = 2;
        cpu.write(0x0000, 0x00).unwrap();
        cpu.write(0x0001, 0x01).unwrap();
        cpu.write(0x0105, 0x42).unwrap();

        let seen = Cell::new(None);
        let template = absolute_indexed_x_addr_r(|_, value| {
            seen.set(Some(value));
        });

        // Cycle 2: read low byte
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 3);

        // Cycle 3: read high byte and add X
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 4);
        assert!(!cpu.page_boundary_cross);

        // Cycle 4: final read, then finish
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 1);
        assert_eq!(seen.get(), Some(0x42));
    }

    #[test]
    fn test_absolute_indexed_x_addr_r_page_cross() {
        let bus = CPUBus::new(test_cartridge());
        let mut cpu = CPU::new(bus, false).unwrap();

        // Base $01FE + X = $05 → dummy $0103, real $0203
        cpu.pc = 0x0000;
        cpu.x = 0x05;
        cpu.cycle_step = 2;
        cpu.write(0x0000, 0xFE).unwrap();
        cpu.write(0x0001, 0x01).unwrap();
        cpu.write(0x0103, 0x11).unwrap();
        cpu.write(0x0203, 0x22).unwrap();

        let seen = Cell::new(None);
        let template = absolute_indexed_x_addr_r(|_, value| {
            seen.set(Some(value));
        });

        // Cycle 2: read low byte
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 3);

        // Cycle 3: read high byte and add X (overflows)
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 4);
        assert!(cpu.page_boundary_cross);

        // Cycle 4: uncorrected dummy read, then fix high byte
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 5);
        assert_eq!(cpu.target_addr, 0x0103);
        assert_eq!(cpu.high_buffer, 0x02);
        assert_eq!(seen.get(), None);

        // Cycle 5: corrected final read, then finish
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 1);
        assert_eq!(seen.get(), Some(0x22));
    }

    #[test]
    fn test_absolute_indexed_x_addr_w_no_page_cross() {
        let bus = CPUBus::new(test_cartridge());
        let mut cpu = CPU::new(bus, false).unwrap();

        // Base $0100 + X = $05 → still 5 cycles, dummy read on cycle 4
        cpu.pc = 0x0000;
        cpu.x = 0x05;
        cpu.cycle_step = 2;
        cpu.write(0x0000, 0x00).unwrap();
        cpu.write(0x0001, 0x01).unwrap();

        let template = absolute_indexed_x_addr_w(|_| 0xAA);

        // Cycle 2: read low byte
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 3);

        // Cycle 3: read high byte and add X
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 4);
        assert!(!cpu.page_boundary_cross);

        // Cycle 4: dummy read, no early finish
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 5);
        assert_eq!(cpu.target_addr, 0x0105);
        assert_eq!(cpu.read(0x0105).unwrap(), 0x00);

        // Cycle 5: real write, then finish
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 1);
        assert_eq!(cpu.read(0x0105).unwrap(), 0xAA);
    }

    #[test]
    fn test_absolute_indexed_x_addr_w_page_cross() {
        let bus = CPUBus::new(test_cartridge());
        let mut cpu = CPU::new(bus, false).unwrap();

        // Base $01FE + X = $05 → dummy $0103, write $0203
        cpu.pc = 0x0000;
        cpu.x = 0x05;
        cpu.cycle_step = 2;
        cpu.write(0x0000, 0xFE).unwrap();
        cpu.write(0x0001, 0x01).unwrap();

        let template = absolute_indexed_x_addr_w(|_| 0xAA);

        // Cycle 2: read low byte
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 3);

        // Cycle 3: read high byte and add X (overflows)
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 4);
        assert!(cpu.page_boundary_cross);

        // Cycle 4: uncorrected dummy read, then fix high byte
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 5);
        assert_eq!(cpu.target_addr, 0x0103);
        assert_eq!(cpu.high_buffer, 0x02);
        assert_eq!(cpu.read(0x0103).unwrap(), 0x00);
        assert_eq!(cpu.read(0x0203).unwrap(), 0x00);

        // Cycle 5: write to the corrected address, then finish
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 1);
        assert_eq!(cpu.read(0x0103).unwrap(), 0x00);
        assert_eq!(cpu.read(0x0203).unwrap(), 0xAA);
    }

    #[test]
    fn test_zero_page_addr_rmw_dummy_write() {
        let bus = CPUBus::new(test_cartridge());
        let mut cpu = CPU::new(bus, false).unwrap();

        cpu.pc = 0x0000;
        cpu.cycle_step = 2;
        cpu.write(0x0000, 0x20).unwrap();
        cpu.write(0x0020, 0x10).unwrap();

        let template = zero_page_addr_rmw(|_, value| value.wrapping_add(1));

        // Cycle 2: read zero-page address
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 3);

        // Cycle 3: read operand into alu_buffer
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 4);
        assert_eq!(cpu.target_addr, 0x0020);
        assert_eq!(cpu.alu_buffer, 0x10);

        // Cycle 4: dummy write of the unmodified buffer, then mutate
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 5);
        assert_eq!(cpu.read(0x0020).unwrap(), 0x10);
        assert_eq!(cpu.alu_buffer, 0x11);

        // Cycle 5: write the mutated result, then finish
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 1);
        assert_eq!(cpu.read(0x0020).unwrap(), 0x11);
    }

    #[test]
    fn test_relative_addr_not_taken() {
        let bus = CPUBus::new(test_cartridge());
        let mut cpu = CPU::new(bus, false).unwrap();

        cpu.pc = 0x0100;
        cpu.cycle_step = 2;
        cpu.write(0x0100, 0x05).unwrap();

        let template = relative_addr(|_| false);

        // Cycle 2: read offset, branch not taken, finish
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 1);
        assert_eq!(cpu.pc, 0x0101);
    }

    #[test]
    fn test_relative_addr_taken_same_page() {
        let bus = CPUBus::new(test_cartridge());
        let mut cpu = CPU::new(bus, false).unwrap();

        cpu.pc = 0x0100;
        cpu.cycle_step = 2;
        cpu.write(0x0100, 0x05).unwrap();

        let template = relative_addr(|_| true);

        // Cycle 2: read offset, branch taken
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 3);
        assert_eq!(cpu.pc, 0x0101);

        // Cycle 3: apply offset on the same page, then finish
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 1);
        assert_eq!(cpu.pc, 0x0106);
    }

    #[test]
    fn test_relative_addr_taken_page_cross() {
        let bus = CPUBus::new(test_cartridge());
        let mut cpu = CPU::new(bus, false).unwrap();

        // Offset at $01FE; after the increment, PC is $01FF + $05 → $0204
        cpu.pc = 0x01FE;
        cpu.cycle_step = 2;
        cpu.write(0x01FE, 0x05).unwrap();

        let template = relative_addr(|_| true);

        // Cycle 2: read offset, branch taken
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 3);
        assert_eq!(cpu.pc, 0x01FF);

        // Cycle 3: dummy read, fix only the low byte
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 4);
        assert!(cpu.page_boundary_cross);
        assert_eq!(cpu.pc, 0x0104);
        assert_eq!(cpu.high_buffer, 0x02);

        // Cycle 4: dummy read, then write the corrected high byte
        template(&mut cpu).unwrap();
        assert_eq!(cpu.cycle_step, 1);
        assert_eq!(cpu.pc, 0x0204);
    }
}