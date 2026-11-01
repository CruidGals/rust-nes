
/***
 * Mapper trait declaration
 */

pub trait Mapper {
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