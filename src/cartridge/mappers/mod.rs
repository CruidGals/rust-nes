/***
 * Module declaration for the mappers. Mainly enumerates and chooses the mapper
 * that the ROM needs
 */

/**
 * Mapper declarations
 */
mod nrom;

use super::mapper::{Mapper, check_size, initialize_memory};

pub fn choose_mapper(
    mapper_number: u16, 
    submapper_number: u8,
    prg_rom_size: usize, 
    prg_ram_size: usize, 
    chr_rom_size: usize, 
    chr_ram_size: usize,
    prg_rom: &[u8],
    prg_ram: &[u8],
    chr_rom: &[u8],
    chr_ram: &[u8],
    trainer_data: Option<Vec<u8>>,
) -> Result<Box<dyn Mapper>, String> {
    match (mapper_number, submapper_number) {
        (0, 0) => Ok(Box::new(nrom::NROM::new(prg_rom_size, prg_ram_size, chr_rom_size, prg_rom, prg_ram, chr_rom, trainer_data)?)),
        _ => Err(format!("Invalid mapper number: {mapper_number}"))
    }
}