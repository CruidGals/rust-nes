/***
 * Implementation of the Cartridge struct
 */

pub mod mapper;
pub mod mappers;

use mapper::Mapper;
use mappers::choose_mapper;

use crate::cartridge::{NametableArrangement::{Horizontal, Vertical}};

// TODO: Keep this here, move it somewhere else later
#[derive(Debug)]
enum NametableArrangement {
    Vertical, Horizontal
}

#[derive(Debug)]
pub struct Cartridge {
    nametable_arrangement: NametableArrangement,
    has_persistent_ram: bool,
    ignore_mirroring: bool,
    mapper: Box<dyn Mapper>,
    trainer_data: Option<Vec<u8>>, // Emergency RAM if trainer speciied and PRG_RAM not available
}

impl Cartridge {
    pub fn new(rom: &[u8]) -> Result<Cartridge, String> {
        Self::from_ines(rom)
    }

    /// Loads a iNES compatible ROM into memory
    pub fn from_ines(rom: &[u8]) -> Result<Self, String> {
        // Check for the minimum size of 16 bytes (must at least contain the iNES header)
        if rom.len() < 16 { return Err(format!("ROM is not of minimum size: {} bytes", rom.len())); }

        // Check if magic header is there
        if rom[..4] != *b"NES\x1A" {return Err(format!("ROM contains an invalid header (not $4E $45 $53 $1A)"));}

        // Decode flag 6
        let nametable_arrangement = if rom[6] & 0x01 == 0x01 { Horizontal } else { Vertical };
        let has_persistent_ram = rom[6] & 0x02 == 0x02;
        let has_trainer = rom[6] & 0x04 == 0x04;
        let ignore_mirroring = rom[6] & 0x08 == 0x08;

        // Get mapper
        let mapper = rom[7] & 0xF0 | (rom[6] & 0xF0) >> 4;
        
        // Split parsing between NES and NES 2.0
        if rom[7] & 0x0C == 0x08 { 
            Cartridge::parse_ines_20(rom, nametable_arrangement, has_persistent_ram, has_trainer, ignore_mirroring, mapper) 
        } else if rom[7] & 0x0C == 0x00 && rom[12..=15].iter().all(|&val| val == 0) { 
            Cartridge::parse_ines(rom, nametable_arrangement, has_persistent_ram, has_trainer, ignore_mirroring, mapper) 
        } else {
            Cartridge::parse_archaic_ines(rom, nametable_arrangement, has_persistent_ram, has_trainer, ignore_mirroring, mapper)
        }
    }

    fn parse_archaic_ines(
        rom: &[u8], 
        nametable_arrangement: NametableArrangement, 
        has_persistent_ram: bool, 
        has_trainer: bool, 
        ignore_mirroring: bool, 
        mapper: u8
    ) -> Result<Self, String> {
        let prg_banks = rom[4];
        let chr_banks = rom[5];
        
        // Get the size for each
        let prg_rom_size: usize = (prg_banks as usize) * 16 * 1024; // In 16kb units
        let prg_ram_size: usize = 8 * 1024; // Ignore byte 0
        let chr_rom_size: usize = (chr_banks as usize) * 8 * 1024;
        let chr_ram_size: usize = if chr_banks == 0 { 8 * 1024 } else { 0 }; // Defaults to 8192 if no chr rom specified
        let trainer_size: usize = if has_trainer {512} else {0};

        // Ensure that the size is exact and expected
        if rom.len() != (16 + trainer_size + prg_rom_size + chr_rom_size) {
            return Err(format!("ROM is not of expected size. Check that the size matches what is defined in the header."));
        }

        // Extract ROM
        let prg_rom_offset = 16 + trainer_size;
        let prg_rom = rom[prg_rom_offset..prg_rom_offset + prg_rom_size].to_vec();

        let chr_rom_offset = prg_rom_offset + prg_rom_size;
        let chr_rom = rom[chr_rom_offset..chr_rom_offset + chr_rom_size].to_vec();
        
        // Load in RAM (if present in .sav file) TODO
        let prg_ram = vec![0u8; prg_ram_size];
        let chr_ram = vec![0u8; chr_ram_size];

        // Overwrite PRG_RAM with trainer data (if available)
        // This occupies ram from $7000 - &71FF, so offset this by $6000
        let mut trainer_data = if has_trainer { Some(rom[16..16 + trainer_size].to_vec()) } else { None };
        let prg_ram_trainer_data = if prg_ram_size >= 0x1200 {trainer_data.take()} else {None};

        Ok(Cartridge { 
            nametable_arrangement: nametable_arrangement, 
            has_persistent_ram: has_persistent_ram,
            ignore_mirroring: ignore_mirroring, 
            mapper: choose_mapper(
                u16::from(mapper & 0x0F),
                0,
                prg_rom,
                chr_rom,
                prg_ram,
                chr_ram,
                vec![],
                vec![],
                prg_ram_trainer_data)?,
            trainer_data: trainer_data
        })
    }

    fn parse_ines(
        rom: &[u8], 
        nametable_arrangement: NametableArrangement, 
        has_persistent_ram: bool, 
        has_trainer: bool, 
        ignore_mirroring: bool, 
        mapper: u8
    ) -> Result<Self, String> {
        let prg_banks = rom[4];
        let chr_banks = rom[5];
        
        // Get the size for each
        let prg_rom_size: usize = (prg_banks as usize) * 16 * 1024; // In 16kb units
        let prg_ram_size: usize = if rom[8] == 0x00 { 8 * 1024 } else { (rom[8] as usize) * 8 * 1024 }; // Defaults to 8192 if byte is 0
        let chr_rom_size: usize = (chr_banks as usize) * 8 * 1024;
        let chr_ram_size: usize = if chr_banks == 0 { 8 * 1024 } else { 0 }; // Defaults to 8192 if no chr rom specified
        let trainer_size: usize = if has_trainer {512} else {0};

        // Ensure that the size is exact and expected
        if rom.len() != (16 + trainer_size + prg_rom_size + chr_rom_size) {
            return Err(format!("ROM is not of expected size. Check that the size matches what is defined in the header."));
        }

        // Extract ROM
        let prg_rom_offset = 16 + trainer_size;
        let prg_rom = rom[prg_rom_offset..prg_rom_offset + prg_rom_size].to_vec();

        let chr_rom_offset = prg_rom_offset + prg_rom_size;
        let chr_rom = rom[chr_rom_offset..chr_rom_offset + chr_rom_size].to_vec();
        
        // Load in RAM (if present in .sav file) TODO
        let prg_ram = vec![0u8; prg_ram_size];
        let chr_ram = vec![0u8; chr_ram_size];

        // Overwrite PRG_RAM with trainer data (if available)
        // This occupies ram from $7000 - &71FF, so offset this by $6000
        let mut trainer_data = if has_trainer { Some(rom[16..16 + trainer_size].to_vec()) } else { None };
        let prg_ram_trainer_data = if prg_ram_size >= 0x1200 {trainer_data.take()} else {None};

        Ok(Cartridge { 
            nametable_arrangement: nametable_arrangement, 
            has_persistent_ram: has_persistent_ram,
            ignore_mirroring: ignore_mirroring, 
            mapper: choose_mapper(
                u16::from(mapper),
                0,
                prg_rom,
                chr_rom,
                prg_ram,
                chr_ram,
                vec![],
                vec![],
                prg_ram_trainer_data)?,
            trainer_data: trainer_data
        })
    }

    fn parse_ines_20(
        rom: &[u8],
        nametable_arrangement: NametableArrangement,
        has_persistent_ram: bool,
        has_trainer: bool,
        ignore_mirroring: bool,
        mapper: u8
    ) -> Result<Self, String> {
        // Get size for all banks
        let prg_rom_size: usize = if rom[9] & 0x0F == 0x0F { 
            Cartridge::exponent_size(rom[4])? 
        } else {
            let size = (u16::from(rom[9] & 0x0F) << 8) | u16::from(rom[4]);
            usize::from(size) * 16 * 1024 
        };

        let chr_rom_size: usize = if rom[9] & 0xF0 == 0xF0 { 
            Cartridge::exponent_size(rom[5])? 
        } else {
            let size = (u16::from(rom[9] & 0xF0) << 4) | u16::from(rom[5]);
            usize::from(size) * 8 * 1024 
        };

        let prg_ram_size: usize = if rom[10] & 0x0F == 0 { 0 } else { 64 << usize::from(rom[10] & 0x0F) };
        let prg_nvram_size: usize = if rom[10] >> 4 == 0 { 0 } else { 64 << usize::from(rom[10] >> 4) };
        let chr_ram_size: usize = if rom[11] & 0x0F == 0 { 0 } else { 64 << usize::from(rom[11] & 0x0F) };
        let chr_nvram_size: usize = if rom[11] >> 4 == 0 { 0 } else { 64 << usize::from(rom[11] >> 4) };
        let trainer_size: usize = if has_trainer {512} else {0};

        // Ensure that the size is exact and expected
        if rom.len() != (16 + trainer_size + prg_rom_size + chr_rom_size) {
            return Err(format!("ROM is not of expected size. Check that the size matches what is defined in the header."));
        }

        // Extract ROM
        let prg_rom_offset = 16 + trainer_size;
        let prg_rom = rom[prg_rom_offset..prg_rom_offset + prg_rom_size].to_vec();

        let chr_rom_offset = prg_rom_offset + prg_rom_size;
        let chr_rom = rom[chr_rom_offset..chr_rom_offset + chr_rom_size].to_vec();

        // TODO: load ram with associated .sav file of the rom if present
        let prg_ram = vec![0u8; prg_ram_size];
        let prg_nvram = vec![0u8; prg_nvram_size];
        let chr_ram = vec![0u8; chr_ram_size];
        let chr_nvram = vec![0u8; chr_nvram_size];

        // Overwrite PRG_RAM with trainer data (if available)
        // This occupies ram from $7000 - &71FF, so offset this by $6000
        let mut trainer_data = if has_trainer { Some(rom[16..16 + trainer_size].to_vec()) } else { None };
        let prg_ram_trainer_data = if prg_ram_size >= 0x1200 {trainer_data.take()} else {None};

        Ok(Cartridge { 
            nametable_arrangement: nametable_arrangement, 
            has_persistent_ram: has_persistent_ram,
            ignore_mirroring: ignore_mirroring, 
            mapper: choose_mapper(
                u16::from(mapper),
                0,
                prg_rom,
                chr_rom,
                prg_ram,
                chr_ram,
                prg_nvram,
                chr_nvram,
                prg_ram_trainer_data)?,
            trainer_data: None
        })
    }

    /// Helper function to calculate exponent size for PRG_ROM and CHR_ROM in NES2.0
    fn exponent_size(byte: u8) -> Result<usize, String> {
        let exponent = u32::from(byte >> 2);
        let multiplier: usize = usize::from(byte & 0x03) * 2 + 1;
        let base = 1usize.checked_shl(exponent).ok_or("ROM size exponent does not fit system usize.")?;
        base.checked_mul(multiplier).ok_or("ROM size does not fit usize".to_string())
    }

    /// Saves the contents of the PRG and CHR ram
    pub fn save_ram(&self) -> Result<(), String> {
        todo!()
    }
}

// Split implementation to include the RW wrapper functions
impl Cartridge {
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

#[cfg(test)]
mod tests {
    use super::*;

    const INES_MAGIC: [u8; 4] = *b"NES\x1A";
    const PRG_BANK_SIZE: usize = 16 * 1024;
    const CHR_BANK_SIZE: usize = 8 * 1024;
    const TRAINER_SIZE: usize = 512;
    const HORIZONTAL_ARRANGEMENT: u8 = 0x01;
    const BATTERY_FLAG: u8 = 0x02;
    const TRAINER_FLAG: u8 = 0x04;
    const ALTERNATIVE_NAMETABLE: u8 = 0x08;

    /// Builds an iNES image. `prg_banks` is in 16 KB units and `chr_banks` is in 8 KB units.
    /// `trainer_first` sets the trainer flag and places that byte at the start of the trainer.
    fn ines_image(
        signature: [u8; 4],
        prg_banks: u8,
        chr_banks: u8,
        mapper: u8,
        trainer_first: Option<u8>,
        prg_first: u8,
        chr_first: u8,
    ) -> Vec<u8> {
        let mut flags6 = (mapper & 0x0F) << 4;
        if trainer_first.is_some() {
            flags6 |= TRAINER_FLAG;
        }
        let flags7 = mapper & 0xF0;

        let mut rom = vec![0u8; 16];
        rom[..4].copy_from_slice(&signature);
        rom[4] = prg_banks;
        rom[5] = chr_banks;
        rom[6] = flags6;
        rom[7] = flags7;

        if let Some(first) = trainer_first {
            let mut trainer = vec![0u8; TRAINER_SIZE];
            trainer[0] = first;
            rom.extend(trainer);
        }

        let mut prg = vec![0u8; prg_banks as usize * PRG_BANK_SIZE];
        if !prg.is_empty() {
            prg[0] = prg_first;
        }
        rom.extend(prg);

        let mut chr = vec![0u8; chr_banks as usize * CHR_BANK_SIZE];
        if !chr.is_empty() {
            chr[0] = chr_first;
        }
        rom.extend(chr);

        rom
    }

    /*
     * Unit tests for from_ines
     */

    #[test]
    fn test_from_ines_rejects_input_shorter_than_header() {
        let rom = vec![0u8; 15];
        assert!(Cartridge::from_ines(&rom).is_err());
    }

    #[test]
    fn test_from_ines_reads_prg_chr_and_flag6() {
        let rom = ines_image(INES_MAGIC, 1, 1, 0, None, 0x22, 0x33);
        let cart = Cartridge::from_ines(&rom).unwrap();

        assert_eq!(cart.cpu_read(0x8000).unwrap(), 0x22);
        assert_eq!(cart.ppu_read(0x0000).unwrap(), 0x33);
        assert!(matches!(cart.nametable_arrangement, NametableArrangement::Vertical));
        assert!(!cart.has_persistent_ram);
        assert!(!cart.ignore_mirroring);

        let mut flagged = rom.clone();
        flagged[6] |= HORIZONTAL_ARRANGEMENT | BATTERY_FLAG | ALTERNATIVE_NAMETABLE;
        let cart = Cartridge::from_ines(&flagged).unwrap();

        assert_eq!(cart.cpu_read(0x8000).unwrap(), 0x22);
        assert_eq!(cart.ppu_read(0x0000).unwrap(), 0x33);
        assert!(matches!(cart.nametable_arrangement, NametableArrangement::Horizontal));
        assert!(cart.has_persistent_ram);
        assert!(cart.ignore_mirroring);
    }

    #[test]
    fn test_from_ines_skips_trainer_before_prg() {
        let rom = ines_image(INES_MAGIC, 1, 1, 0, Some(0x11), 0x22, 0x33);
        let cart = Cartridge::from_ines(&rom).unwrap();

        assert_eq!(cart.cpu_read(0x7000).unwrap(), 0x11);
        assert_eq!(cart.cpu_read(0x6000).unwrap(), 0x00);
        assert_eq!(cart.cpu_read(0x8000).unwrap(), 0x22);
        assert_eq!(cart.ppu_read(0x0000).unwrap(), 0x33);
    }

    #[test]
    fn test_from_ines_prg_ram_size() {
        for prg_ram_byte in [0, 1] {
            let mut rom = ines_image(INES_MAGIC, 1, 1, 0, None, 0x22, 0x33);
            rom[8] = prg_ram_byte;
            let mut cart = Cartridge::from_ines(&rom).unwrap();

            cart.cpu_write(0x6000, 0x42).unwrap();
            cart.cpu_write(0x7000, 0x99).unwrap();
            assert_eq!(cart.cpu_read(0x6000).unwrap(), 0x42);
            assert_eq!(cart.cpu_read(0x7000).unwrap(), 0x99);
        }

        let mut rom = ines_image(INES_MAGIC, 1, 1, 0, None, 0x22, 0x33);
        rom[8] = 2;
        let err = Cartridge::from_ines(&rom).unwrap_err();
        assert!(err.contains("16384"), "{err}");
    }

    #[test]
    fn test_from_ines_rejects_image_one_byte_short() {
        let mut rom = ines_image(INES_MAGIC, 1, 1, 0, None, 0x22, 0x33);
        rom.pop();
        assert!(Cartridge::from_ines(&rom).is_err());
    }

    #[test]
    fn test_from_ines_accepts_zero_chr_banks() {
        let rom = ines_image(INES_MAGIC, 1, 0, 0, None, 0x22, 0x00);
        let cart = Cartridge::from_ines(&rom).unwrap();

        assert_eq!(cart.cpu_read(0x8000).unwrap(), 0x22);
    }

    #[test]
    fn test_from_ines_rejects_trailing_byte() {
        let mut rom = ines_image(INES_MAGIC, 1, 1, 0, None, 0x22, 0x33);
        rom.push(0xFF);
        assert!(Cartridge::from_ines(&rom).is_err());
    }

    #[test]
    fn test_from_ines_rejects_bad_signature() {
        let rom = ines_image(*b"NEX\x1A", 1, 1, 0, None, 0x22, 0x33);
        assert!(Cartridge::from_ines(&rom).is_err());
    }

    #[test]
    fn test_from_ines_combines_mapper_nibbles() {
        // 0x21: low nibble in byte 6, high nibble in byte 7. Only mapper 0 is implemented.
        let rom = ines_image(INES_MAGIC, 1, 1, 0x21, None, 0x22, 0x33);
        let err = Cartridge::from_ines(&rom).unwrap_err();
        assert!(err.contains("33"), "{err}");
    }

    #[test]
    fn test_from_ines_archaic_header_masks_mapper_and_ignores_prg_ram_byte() {
        // "DiskDude!" at bytes 7-15: 'D' would make mapper 64, 'i' would ask for 840 KB of PRG RAM.
        let mut rom = ines_image(INES_MAGIC, 1, 1, 0, None, 0x22, 0x33);
        rom[7..16].copy_from_slice(b"DiskDude!");

        let mut cart = Cartridge::from_ines(&rom).unwrap();
        assert_eq!(cart.cpu_read(0x8000).unwrap(), 0x22);
        assert_eq!(cart.ppu_read(0x0000).unwrap(), 0x33);
        cart.cpu_write(0x6000, 0x42).unwrap();
        cart.cpu_write(0x7000, 0x99).unwrap();
        assert_eq!(cart.cpu_read(0x6000).unwrap(), 0x42);
        assert_eq!(cart.cpu_read(0x7000).unwrap(), 0x99);

        // Low nibble from byte 6 is kept. 0x10 | junk high nibble must be mapper 1, not 65.
        rom[6] |= 0x10;
        let err = Cartridge::from_ines(&rom).unwrap_err();
        assert_eq!(err, "Invalid mapper number: 1");
    }
}