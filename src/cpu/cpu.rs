/***
 * Business logic for the 6502 CPU
 */

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

#[derive(Debug, Clone)]
pub struct CPU {
    /* Registers */
    a: u8, /* Accumulator */
    x: u8, /* X-Indexed register */
    y: u8, /* Y-Indexed register */
    pc: u16, /* Program Counter */
    sp: u8, /* Stack Pointer */
    p: StatusRegister, /* Status register */
}

impl CPU {
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0xFFFC,
            sp: 0xFD,
            p: StatusRegister::empty()
        }
    }
}