/***
 * Implementation of the Cartridge struct
 */

pub mod mapper;
pub mod mappers;

use mapper::Mapper;

pub struct Cartridge {
    mapper: Box<dyn Mapper>
}

impl Cartridge {
    ///
    /// Loads a iNES compatible ROM into memory
    /// 
    pub fn from_ines(rom: &[u8]) -> Result<Self, String> {
        todo!()
    }

    ///
    /// Saves the contents of the PRG and CHR ram
    /// 
    pub fn save_ram(&self) -> Result<(), String> {
        todo!()
    }

    pub fn cpu_read(&self, addr: u16) -> Result<u8, String> {
        self.mapper.cpu_read(addr) // TODO
    }

    pub fn cpu_write(&mut self, addr: u16, value: u8) -> Result<(), String> {
        self.mapper.cpu_write(addr, value)
    }

    pub fn ppu_read(&self, addr: u16) -> Result<u8, String> {
        self.mapper.ppu_read(addr) // TODO
    }

    pub fn ppu_write(&mut self, addr: u16, value: u8) -> Result<(), String> {
        self.mapper.ppu_write(addr, value)
    }
}