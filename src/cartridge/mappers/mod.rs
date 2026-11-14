/***
 * Module declaration for the mappers. Mainly enumerates and chooses the mapper
 * that the ROM needs
 */

/**
 * Mapper declarations
 */
mod nrom;

use super::mapper::{Mapper, check_size};

pub fn choose_mapper(
    mapper_number: u16, 
    submapper_number: u8,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    prg_ram: Vec<u8>,
    chr_ram: Vec<u8>,
    prg_nvram: Vec<u8>,
    chr_nvram: Vec<u8>,
    trainer_data: Option<Vec<u8>>,
) -> Result<Box<dyn Mapper>, String> {
    match (mapper_number, submapper_number) {
        (0, 0) => Ok(Box::new(nrom::NROM::new(prg_rom, chr_rom, prg_ram, trainer_data)?)),
        _ => Err(format!("Invalid mapper number: {mapper_number}"))
    }
}