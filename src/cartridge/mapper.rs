
/***
 * Mapper trait declaration
 */

/// Helper function for mappers to ensure ROM sizes don't go past their limits
pub fn check_size(name: &str, size: usize, max_size: usize) -> Result<(), String> {
    if size > max_size {
        Err(format!("{name} size is outside of maximum for this mapper: {size} > {max_size}"))
    } else {
        Ok(())
    }
}

pub fn initialize_memory(name: &str, size: usize, memory: &[u8]) -> Result<Vec<u8>, String> {
    check_size(format!("{name} initialized memory").as_str(), memory.len(), size)?;

    // Make the vector
    let mut vec = vec![0u8; size];
    vec[..memory.len()].copy_from_slice(memory);
    Ok(vec)
}

pub trait Mapper: std::fmt::Debug {
    /// Wrapper function to capture CPU reads to PRG_ROM and PRG_RAM. The raw CPU 
    /// read address must be preserved in the addr argument (e.g a 0x8000 should 
    /// be passed in as 0x8000 rather than 0x0000). Structs that implement this 
    /// function will handle this offset accordingly.
    fn cpu_read(&self, addr: u16) -> Result<u8, String>;

    /// Wrapper function to capture CPU writes to the cartridge. Functionality is
    /// often used in bank switching; mapper controls the behavior of this.
    fn cpu_write(&mut self, addr: u16, value: u8) -> Result<(), String>;

    /// Wrapper function to capture PPU reads to CHR_ROM and CHR_RAM. The raw PPU 
    /// read address must be preserved in the addr argument (e.g a 0x3000 should 
    /// be passed in as 0x3000 rather than 0x1000). Structs that implement this 
    /// function will handle this offset accordingly.
    fn ppu_read(&self, addr: u16) -> Result<u8, String>;

    /// Wrapper function to capture PPUP writes to the cartridge. Functionality is
    /// often used in bank switching; mapper controls the behavior of this.
    fn ppu_write(&mut self, addr: u16, value: u8) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_size() {
        assert!(check_size("PRG_ROM", 16 * 1024, 32 * 1024).is_ok());
        assert!(check_size("PRG_ROM", 32 * 1024, 32 * 1024).is_ok());
        assert!(check_size("PRG_ROM", 32 * 1024 + 1, 32 * 1024).is_err());
    }

    #[test]
    fn test_initialize_memory() {
        let data = [0xAA, 0xBB, 0xCC];

        let exact = initialize_memory("PRG_ROM", 3, &data).unwrap();
        assert_eq!(exact, vec![0xAA, 0xBB, 0xCC]);

        let padded = initialize_memory("PRG_ROM", 5, &data).unwrap();
        assert_eq!(padded, vec![0xAA, 0xBB, 0xCC, 0x00, 0x00]);

        assert!(initialize_memory("PRG_ROM", 2, &data).is_err());
    }
}