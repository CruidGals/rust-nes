pub mod ppubus;

use crate::ppu::ppubus::PPUBus;
use bitflags::bitflags;

/***
 * Business logic for NES PPU
 */

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PPUSTATUS: u8 {
        const SPRITE_OVERFLOW = 0b0010_0000;
        const SPRITE_ZERO_HIT = 0b0100_0000;
        const VBLANK          = 0b1000_0000;
    }
}

 #[derive(Debug)]
pub struct PPU {

    /* PPUSTATUS flags */
    ppustatus: PPUSTATUS,

    /* Wiring */
    data_bus: u8, // USED FOR CPU COMMUNICATION
    bus: PPUBus,
}

impl PPU {
    pub fn new(bus: PPUBus) -> Result<Self, String> {
        Ok(Self { 
            ppustatus: PPUSTATUS::VBLANK | PPUSTATUS::SPRITE_OVERFLOW,
            data_bus: 0,
            bus: bus,
        })
    }
}

impl PPU {
    pub fn read(&self, address: u16) -> Result<u8, String> {
        self.bus.read(address)
            .map_err(|err| format!("Error reading from PPU VRAM at {address:#06X}: {err}"))
    }

    pub fn write(&mut self, address: u16, value: u8) -> Result<(), String> {
        self.bus.write(address, value)
            .map_err(|err| format!("Error writing to PPU VRAM at {address:#06X}: {err}"))
    }

    // CPU-specific memory-mapped IO functions

    /// Given a memory-mapped address from CPU, return the expected contents.
    /// Will be given a raw address from CPU (e.g. $2002 PPUSTATUS)
    pub fn memory_mapped_read(&mut self, address: u16) -> Result<u8, String> {
        match address {
            0x2002 => {
                self.data_bus = self.ppustatus.bits();
                Ok(self.data_bus)
            },
            _ => Err(format!("Address is not a valid memory-mapped address: {:04X}", address)),
        }
    }

    pub fn memory_mapped_write(&self, address: u16, value: u8) -> Result<(), String> {
        todo!()
    }
}