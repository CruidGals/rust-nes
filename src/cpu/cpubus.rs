/***
 * Bus speciically to connect CPU components
 */

#[derive(Debug)]
pub struct CPUBus {
    // For now, just include reference to CPU VRAM
    vram: [u8; 2048], // 2KB of internal vram
    interrupt_vecs: [u8; 6] // vectors for interrupts
}

impl CPUBus {
    pub fn new() -> Self {
        Self { 
            vram: [0; 2048],
            interrupt_vecs: [0; 6]
        }
    }

    ///
    /// Read from CPU vram or through memory mapped I/O
    /// 
    pub fn read(&self, address: u16) -> Result<u8, String> {
        match address {
            // 2KB Internal VRAM is mirrored 4 times
            0x0000..=0x1FFF => Ok(self.vram[(address & 0x7FF) as usize]),

            // Interrupt vectors mapping -- 1:1
            0xFFFA..=0xFFFF => Ok(self.interrupt_vecs[(0xFFFF - address) as usize]),
            
            // Implement memory mapped PPU stuff later..
            _ => Err(format!("Address {:#X} is outside of VRAM range", address)),
        }
    }

    ///
    /// Write to CPU vram or through memory mapped I/O
    /// 
    pub fn write(&mut self, address: u16, value: u8) -> Result<(), String> {
        match address {
            // 2KB Internal VRAM is mirrored 4 times
            0x0000..=0x1FFF => {
                self.vram[(address & 0x7FF) as usize] = value;
                Ok(())
            },

            // Interrupt vectors mapping -- 1:1
            0xFFFA..=0xFFFF => {
                self.interrupt_vecs[(0xFFFF - address) as usize] = value;
                Ok(())
            }
            
            // Implement memory mapped PPU stuff later..
            _ => Err(format!("Address {:#X} is outside of VRAM range", address)),
        }
    }
}