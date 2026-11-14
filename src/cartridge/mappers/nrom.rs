/***
 * Implementation of NROM mapper (mapper 000)
 */

use super::Mapper;
use super::check_size;
use super::initialize_memory;

/// Size of the PRG_ROM for NROM. This is either 16K or 32K,
/// for simplicity sake we set it to the max that it can be.
const PRG_ROM_MAX_SIZE: usize = 32 * 1024;

/// Size of the PRG_RAM for NROM. Boards are 2K, 4K, or 8K.
/// iNES treats a PRG-RAM size of 0 as 8K, so the max is 8K.
const PRG_RAM_MAX_SIZE: usize = 8 * 1024;

/// Size of the CHR_ROM for NROM, which is 8K.
const CHR_ROM_MAX_SIZE: usize = 8 * 1024;

#[derive(Debug)]
pub struct NROM {
    prg_rom: Vec<u8>,
    prg_ram: Vec<u8>,
    chr_rom: Vec<u8>,
}

impl NROM {
    pub fn new(prg_rom_size: usize, prg_ram_size: usize, chr_rom_size: usize, prg_rom: &[u8], prg_ram: &[u8], chr_rom: &[u8]) -> Result<NROM, String> {
        // Perform size checks
        check_size("PRG_ROM", prg_rom_size, PRG_ROM_MAX_SIZE)?;
        check_size("PRG_RAM", prg_ram_size, PRG_RAM_MAX_SIZE)?;
        check_size("CHR_ROM", chr_rom_size, CHR_ROM_MAX_SIZE)?;

        Ok(NROM { 
            prg_rom: initialize_memory("PRG_ROM", prg_rom_size, prg_rom)?, 
            prg_ram: initialize_memory("PRG_RAM", prg_ram_size, prg_ram)?, 
            chr_rom: initialize_memory("CHR_ROM", chr_rom_size, chr_rom)? 
        })
    }
}

impl Mapper for NROM {
    fn cpu_read(&self, addr: u16) -> Result<u8, String> {
        // Must handle memory mirroring
        match addr {
            0x6000..=0x7FFF => {
                let offset = (addr - 0x6000) as usize % self.prg_ram.len();
                Ok(self.prg_ram[offset])
            },
            0x8000..=0xFFFF => {
                let offset = (addr - 0x8000) as usize % self.prg_rom.len();
                Ok(self.prg_rom[offset])
            },
            _ => Err(format!("Memory address is out of bounds for CPU: {addr}"))
        }
    }

    fn cpu_write(&mut self, addr: u16, value: u8) -> Result<(), String> {
        // Write to PRG_RAM. Anything else is a no-op.
        match addr {
            0x6000..=0x7FFF => {
                let offset = (addr - 0x6000) as usize % self.prg_ram.len();
                self.prg_ram[offset] = value;
                Ok(())
            },
            _ => Ok(())
        }
    }

    fn ppu_read(&self, addr: u16) -> Result<u8, String> {
        match addr {
            0x0000..=0x1FFF => Ok(self.chr_rom[addr as usize]),
            _ => Err(format!("Memory address is out of bounds for PPU: {addr}"))
        }
    }

    fn ppu_write(&mut self, _addr: u16, _value: u8) -> Result<(), String> {
        // Function is inherently a no-op because there is no bank switching or CHR RAM
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Mapper;

    fn garbage_banks(prg_rom_size: usize, prg_ram_size: usize, chr_rom_size: usize) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        (
            vec![0xAA; prg_rom_size],
            vec![0xBB; prg_ram_size],
            vec![0xCC; chr_rom_size],
        )
    }

    fn new_nrom(prg_rom_size: usize, prg_ram_size: usize, chr_rom_size: usize) -> Result<NROM, String> {
        let (prg_rom, prg_ram, chr_rom) = garbage_banks(prg_rom_size, prg_ram_size, chr_rom_size);
        NROM::new(prg_rom_size, prg_ram_size, chr_rom_size, &prg_rom, &prg_ram, &chr_rom)
    }

    #[test]
    fn test_nrom_new_success() {
        let prg_rom_size = 16 * 1024;
        let prg_ram_size = 2 * 1024;
        let chr_rom_size = 8 * 1024;
        let (prg_rom, prg_ram, chr_rom) = garbage_banks(prg_rom_size, prg_ram_size, chr_rom_size);

        let nrom = NROM::new(prg_rom_size, prg_ram_size, chr_rom_size, &prg_rom, &prg_ram, &chr_rom).unwrap();

        assert_eq!(nrom.prg_rom, prg_rom);
        assert_eq!(nrom.prg_ram, prg_ram);
        assert_eq!(nrom.chr_rom, chr_rom);
    }

    #[test]
    fn test_nrom_cpu_read_below_prg_ram() {
        let nrom = new_nrom(16 * 1024, 2 * 1024, 8 * 1024).unwrap();
        assert!(nrom.cpu_read(0x5FFF).is_err());
    }

    #[test]
    fn test_nrom_ppu_read_above_chr() {
        let nrom = new_nrom(16 * 1024, 2 * 1024, 8 * 1024).unwrap();
        assert!(nrom.ppu_read(0x2000).is_err());
    }

    #[test]
    fn test_nrom_cpu_write() {
        let mut nrom = new_nrom(16 * 1024, 2 * 1024, 8 * 1024).unwrap();

        nrom.cpu_write(0x6000, 0x42).unwrap();
        assert_eq!(nrom.cpu_read(0x6000).unwrap(), 0x42);

        nrom.cpu_write(0x8000, 0x99).unwrap();
        assert_eq!(nrom.cpu_read(0x8000).unwrap(), 0xAA);
    }
}