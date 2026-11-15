use crate::ppu::ppubus::PPUBus;

pub mod ppubus;

/***
 * Business logic for NES PPU
 */

 #[derive(Debug)]
pub struct PPU {
    /* Wiring */
    bus: PPUBus,
}

impl PPU {
    pub fn new(bus: PPUBus) -> Result<Self, String> {
        Ok(Self { 
            bus: bus,
        })
    }
}