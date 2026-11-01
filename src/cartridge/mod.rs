/***
 * Implementation of the Cartridge struct
 */

pub mod mapper;
pub mod mappers;

use mapper::Mapper;
use mappers::choose_mapper;

#[derive(Debug)]
pub struct Cartridge {
    mapper: Box<dyn Mapper>
}

impl Cartridge {
    pub fn new(
        mapper_number: u16, 
        prg_rom_size: usize, 
        prg_ram_size: usize, 
        chr_rom_size: usize, 
        chr_ram_size: usize,
        prg_rom: &[u8],
        prg_ram: &[u8],
        chr_rom: &[u8],
        chr_ram: &[u8],
    ) -> Result<Cartridge, String> {
        Ok(Cartridge { mapper: choose_mapper(
            mapper_number, 
            prg_rom_size, 
            prg_ram_size, 
            chr_rom_size, 
            chr_ram_size, 
            prg_rom, 
            prg_ram, 
            chr_rom, 
            chr_ram)? 
        })
    }

    /// Loads a iNES compatible ROM into memory
    pub fn from_ines(rom: &[u8]) -> Result<Self, String> {
        todo!()
    }

    /// Saves the contents of the PRG and CHR ram
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