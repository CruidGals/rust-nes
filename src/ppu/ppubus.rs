/***
 * Bus speciically to connect PPU components
 */

use std::{cell::RefCell, rc::Rc};
use crate::cartridge::{Cartridge, NametableArrangement};

#[derive(Debug)]
pub struct PPUBus {
    vram: [u8; 2048], // 2KB to store the current nametable
    palette_ram: [u8; 256], // 256 bytes internal ram to store palette
    oam: [u8; 256], // 256 bytes (64 sprites * 4 bytes)
    secondary_oam: [u8; 32], // 32 bytes (8 sprites * 4 bytes)
    cartridge: Rc<RefCell<Cartridge>>
}

impl PPUBus {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> Self {
        Self { 
            vram: [0; 2048],
            palette_ram: [0; 256],
            oam: [0; 256],
            secondary_oam: [0xFF; 32],
            cartridge: cartridge
        }
    }

    ///
    /// Read from PPU vram or cartridge
    /// 
    pub fn read(&self, address: u16) -> Result<u8, String> {
        match address {
            // Catridge read to pattern table
            0x0000..=0x1FFF => self.cartridge.borrow().ppu_read(address),

            // Nametable + attribute table read (mirrored)
            0x2000..=0x3EFF => Ok(self.vram[self.nametable_mirroring(address) as usize]),

            // Internal PPU ram reads
            0x3F00..=0x3FFF => {
                // Perform aliasing of background and sprite backdrop color
                let mut offset = address % 0x20;
                offset = match offset {
                    0x10 => 0x00,
                    0x14 => 0x04,
                    0x18 => 0x08,
                    0x1C => 0x0C,
                    _ => offset,
                };

                Ok(self.palette_ram[offset as usize])
            },
            _ => Err(format!("Address {:#X} is outside of VRAM range", address)),
        }
    }

    ///
    /// Write to PPU palette ram or vram
    /// 
    pub fn write(&mut self, address: u16, value: u8) -> Result<(), String> {
        match address {
            // Catridge writes (probably no-op but its dealt with at mapper level)
            0x0000..=0x1FFF => self.cartridge.borrow_mut().ppu_write(address, value),
            
            // Nametable + attribute table read (mirrored)
            0x2000..=0x3EFF => {
                self.vram[self.nametable_mirroring(address) as usize] = value;
                Ok(())
            },
            
            // Internal PPU ram reads
            0x3F00..=0x3FFF => {
                // Perform aliasing of background and sprite backdrop color
                let mut offset = address % 0x20;
                offset = match offset {
                    0x10 => 0x00,
                    0x14 => 0x04,
                    0x18 => 0x08,
                    0x1C => 0x0C,
                    _ => offset,
                };

                self.palette_ram[offset as usize] = value;
                Ok(())
            },
            _ => Err(format!("Address {:#X} is outside of VRAM range", address)),
        }
    }

    /// Transforms the address to fit the bounds of VRAM
    /// based on the mirroring selected by the cartridge.
    /// Address is given in raw form (0x2000->0x3EFF)
    fn nametable_mirroring(&self, address: u16) -> u16 {
        let offset= address & 0x0FFF;

        match self.cartridge.borrow().nametable_arrangement {
            NametableArrangement::Horizontal => offset % 0x800,
            NametableArrangement::Vertical => {
                match offset {
                    0x000..=0x7FF => offset % 0x400,
                    _ => (offset % 0x400) + 0x400,
                }
            }
        }
    }

    /// Read from OAM using any address in u8 range
    pub fn read_oam(&self, address: u8) -> u8 { self.oam[address as usize] }

    /// Write to OAM using any address in u8 range
    pub fn write_oam(&mut self, address: u8, value: u8) -> () { self.oam[address as usize] = value; }

    /// Read from secondary OAM
    pub fn read_secondary_oam(&self, address: u8) -> Result<u8, String> { 
        match address {
            0..0x20 => Ok(self.secondary_oam[address as usize]),
            _ => Err(format!("Address {:#X} is outside of secondary OAM range", address)),
        }
    }

    /// Write to secondary OAM
    pub fn write_secondary_oam(&mut self, address: u8, value: u8) -> Result<(), String> { 
        match address {
            0..0x20 => {
                self.secondary_oam[address as usize] = value;
                Ok(())
            },
            _ => Err(format!("Address {:#X} is outside of secondary OAM range", address)),
        }
    }
}