/***
 * Bus speciically to connect PPU components
 */

use std::{cell::RefCell, rc::Rc};
use crate::cartridge::Cartridge;

#[derive(Debug)]
pub struct PPUBus {
    vram: [u8; 256], // 256 bytes internal ram to store palette
    cartridge: Rc<RefCell<Cartridge>>
}

impl PPUBus {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> Self {
        Self { 
            vram: [0; 256],
            cartridge: cartridge
        }
    }

    ///
    /// Read from PPU vram or cartridge
    /// 
    pub fn read(&self, address: u16) -> Result<u8, String> {
        match address {
            // Catridge reads
            0x0000..=0x3EFF => self.cartridge.borrow().ppu_read(address),

            // Internal PPU ram reads
            0x3F00..=0x3FFF => Ok(self.vram[(address % 0x20) as usize]),
            _ => Err(format!("Address {:#X} is outside of VRAM range", address)),
        }
    }

    ///
    /// Write to PPU vram or cartridge
    /// 
    pub fn write(&mut self, address: u16, value: u8) -> Result<(), String> {
        match address {
            // Catridge writes (probably no-op but its dealt with at mapper level)
            0x0000..=0x3EFF => self.cartridge.borrow_mut().ppu_write(address, value),
            
            0x3F00..=0x3FFF => {
                self.vram[(address % 0x20) as usize] = value;
                Ok(())
            }
            _ => Err(format!("Address {:#X} is outside of VRAM range", address)),
        }
    }
}