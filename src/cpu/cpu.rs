/***
 * Business logic for the 6502 CPU
 */
use super::cpubus::CPUBus;
use bitflags::bitflags;

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
pub struct CPU<'bus> {
    /* Registers */
    pub a: u8, /* Accumulator */
    pub x: u8, /* X-Indexed register */
    pub y: u8, /* Y-Indexed register */
    pub pc: u16, /* Program Counter */
    pub sp: u8, /* Stack Pointer */
    pub p: StatusRegister, /* Status register */

    /* Instruction specific */
    pub curr_opcode: u8,  /* opcode that is being done */
    pub cycle_step: u8, /* cycle that opcode is on */
    pub low_buffer: u8, /* low read buffer when grabbing value */
    pub high_buffer: u8, /* high read buffer when grabbing value */
    pub page_boundary_cross: bool, /* checks if crossed page boundary */
    pub target_addr: u16, /* store addr for later use during write instructions */
    pub alu_buffer: u8, /* buffer used to perform ops with ALU */

    /* Wiring */
    pub bus: &'bus mut CPUBus,
}

impl<'bus> CPU<'bus> {
    pub fn new(cpu_bus: &'bus mut CPUBus) -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0xFFFC,
            sp: 0xFD,
            p: StatusRegister::empty(),

            curr_opcode: 0,
            cycle_step: 0,
            low_buffer: 0,
            high_buffer: 0,
            page_boundary_cross: false,
            target_addr: 0,
            alu_buffer: 0,

            bus: cpu_bus
        }
    }

    ///
    /// Steps through a cycle in the instruction. Increments cycle_step
    /// 
    pub fn step_cycle(&mut self) -> () {
        self.cycle_step += 1;
    }

    ///
    /// Concludes the current instruction being ran. Resets the
    /// current opcode being ran and the cycle step counter. Resets
    /// all necessary buffers.
    /// 
    pub fn finish_instruction(&mut self) -> () {
        self.curr_opcode = 0;
        self.cycle_step = 0;
        self.low_buffer = 0;
        self.high_buffer = 0;
        self.page_boundary_cross = false;
        self.target_addr = 0;
        self.alu_buffer = 0;
    }

}