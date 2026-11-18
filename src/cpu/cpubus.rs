/***
 * Bus speciically to connect CPU components
 */

use std::cell::RefCell;
use std::rc::Rc;
use crate::ppu::PPU;

use super::super::cartridge::Cartridge;

#[derive(Debug)]
pub struct CPUBus {
    // For now, just include reference to CPU VRAM
    vram: [u8; 2048], // 2KB of internal vram
    cartridge: Rc<RefCell<Cartridge>>,
    ppu: Rc<RefCell<PPU>>,
}

impl CPUBus {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>, ppu: Rc<RefCell<PPU>>) -> Self {
        Self { 
            vram: [0; 2048],
            cartridge: cartridge,
            ppu: ppu
        }
    }

    ///
    /// Read from CPU vram or through memory mapped I/O
    /// 
    pub fn read(&self, address: u16) -> Result<u8, String> {
        match address {
            // 2KB Internal VRAM is mirrored 4 times
            0x0000..=0x1FFF => Ok(self.vram[(address & 0x7FF) as usize]),

            // Memory mapped reads
            0x2000..=0x3FFF => Ok(self.ppu.borrow_mut().memory_mapped_read(0x2000 | (address & 0x07))?),

            // Catridge reads (hold interrupt vectors too)
            0x6000..=0xFFFF => self.cartridge.borrow().cpu_read(address),
            
            _ => Err(format!("Address {:#X} is outside of VRAM range", address)),
        }
    }

    ///
    /// Write to CPU vram or through memory mapped I/O
    /// 
    pub fn write(&mut self, address: u16, value: u8) -> Result<(), String> {
        match address {
            // 2KB Internal VRAM is mirrored 4 times
            0x0000..=0x1FFF => {
                self.vram[(address & 0x7FF) as usize] = value;
                Ok(())
            },

            // Memory mapped writes
            // TODO: Make sure implement ignore writes to certain registers until reach
            // prerender scanline of next frame
            0x2000..=0x3FFF => Ok(self.ppu.borrow_mut().memory_mapped_write(0x2000 | (address & 0x07), value)?),

            // Writing to either ram or bank switching
            0x4000..=0xFFFF => self.cartridge.borrow_mut().cpu_write(address, value),

            _ => Err(format!("Address {:#X} is outside of VRAM range", address)),
        }
    }
}