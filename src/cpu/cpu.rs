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
    curr_opcode: Option<fn(&mut CPU) -> Result<(), String>>,  /* opcode that is being done (in function form) */
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

            curr_opcode: None,
            cycle_step: 0,
            low_buffer: 0,
            high_buffer: 0,
            page_boundary_cross: false,
            target_addr: 0,
            alu_buffer: 0,

            bus: cpu_bus
        }
    }

    pub fn tick(&mut self) -> () {
        // TODO stub: Polling PPU's NMI wire

        
    }

    ///
    /// Creates the instruction given opcode.
    /// Fills in the curr_opcode field, nothing else
    /// 
    fn choose_opcode(&mut self, opcode: u8) -> () {
        match opcode {
            0x00 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x01 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x02 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x03 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x04 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x05 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x06 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x07 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x08 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x09 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x0A => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x0B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x0C => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x0D => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x0E => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x0F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x10 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x11 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x12 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x13 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x14 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x15 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x16 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x17 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x18 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x19 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x1A => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x1B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x1C => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x1D => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x1E => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x1F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x20 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x21 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x22 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x23 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x24 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x25 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x26 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x27 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x28 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x29 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x2A => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x2B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x2C => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x2D => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x2E => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x2F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x30 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x31 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x32 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x33 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x34 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x35 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x36 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x37 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x38 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x39 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x3A => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x3B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x3C => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x3D => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x3E => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x3F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x40 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x41 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x42 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x43 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x44 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x45 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x46 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x47 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x48 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x49 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x4A => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x4B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x4C => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x4D => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x4E => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x4F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x50 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x51 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x52 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x53 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x54 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x55 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x56 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x57 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x58 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x59 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x5A => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x5B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x5C => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x5D => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x5E => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x5F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x60 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x61 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x62 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x63 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x64 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x65 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x66 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x67 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x68 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x69 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x6A => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x6B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x6C => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x6D => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x6E => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x6F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x70 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x71 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x72 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x73 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x74 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x75 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x76 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x77 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x78 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x79 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x7A => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x7B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x7C => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x7D => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x7E => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x7F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x80 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x81 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x82 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x83 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x84 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x85 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x86 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x87 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x88 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x89 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x8A => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x8B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x8C => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x8D => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x8E => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x8F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x90 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x91 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x92 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x93 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x94 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x95 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x96 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x97 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x98 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x99 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x9A => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x9B => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x9C => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x9D => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x9E => todo!("Unimplemented opcode: {:#04X}", opcode),
            0x9F => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xA0 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xA1 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xA2 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xA3 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xA4 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xA5 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xA6 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xA7 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xA8 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xA9 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xAA => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xAB => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xAC => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xAD => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xAE => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xAF => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xB0 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xB1 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xB2 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xB3 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xB4 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xB5 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xB6 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xB7 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xB8 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xB9 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xBA => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xBB => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xBC => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xBD => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xBE => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xBF => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xC0 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xC1 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xC2 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xC3 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xC4 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xC5 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xC6 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xC7 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xC8 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xC9 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xCA => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xCB => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xCC => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xCD => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xCE => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xCF => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xD0 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xD1 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xD2 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xD3 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xD4 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xD5 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xD6 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xD7 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xD8 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xD9 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xDA => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xDB => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xDC => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xDD => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xDE => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xDF => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xE0 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xE1 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xE2 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xE3 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xE4 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xE5 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xE6 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xE7 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xE8 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xE9 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xEA => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xEB => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xEC => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xED => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xEE => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xEF => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xF0 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xF1 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xF2 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xF3 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xF4 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xF5 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xF6 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xF7 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xF8 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xF9 => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xFA => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xFB => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xFC => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xFD => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xFE => todo!("Unimplemented opcode: {:#04X}", opcode),
            0xFF => todo!("Unimplemented opcode: {:#04X}", opcode),
        }
    }
}

// Instruction specific functions
impl<'bus> CPU<'bus> {
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
        self.curr_opcode = None;
        self.cycle_step = 0;
        self.low_buffer = 0;
        self.high_buffer = 0;
        self.page_boundary_cross = false;
        self.target_addr = 0;
        self.alu_buffer = 0;
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
impl<'bus> CPU<'bus> {
    pub fn increment_pc(&mut self) -> () {
        self.pc = self.pc.wrapping_add(1);
    }

    pub fn increment_sp(&mut self) -> () {
        self.sp = self.sp.wrapping_add(1);
    }

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
}