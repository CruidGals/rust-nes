pub mod ppubus;
pub mod background_rendering_unit;
pub mod sprite_rendering_unit;

use crate::ppu::{background_rendering_unit::BackgroundRenderingUnit, ppubus::PPUBus, sprite_rendering_unit::SpriteRenderingUnit};
use bitflags::bitflags;

/***
 * Business logic for NES PPU
 */

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PPUCTRL: u8 {
        const VRAM_ADDR_INC           = 0b0000_0100; // 0: add 1 ; 1: add 32
        const SPRITE_PATTERN_ADDR     = 0b0000_1000; // 0: $0000 ; 1: $1000
        const BACKGROUND_PATTERN_ADDR = 0b0001_0000; // 0: $0000 ; 1: $1000
        const SPRITE_SIZE             = 0b0010_0000; // 0: 8x8   ; 1: 8x16
        const MASTER_SLAVE_SELECT     = 0b0100_0000; // 0: read backdrop ; 1: output color
        const VBLANK_NMI              = 0b1000_0000; // 0: off   ; 1: on
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PPUMASK: u8 {
        const GREYSCALE            = 0b0000_0001; // 1: grayscale on
        const BACKGROUND_LEFT_SHOW = 0b0000_0010; // 1: show background on left 8 pixels of screen
        const SPIRTE_LEFT_SHOW     = 0b0000_0100; // 1: show sprites on left 8 pixels of screen
        const BACKGROUND_RENDERING = 0b0000_1000; // 1: enable background rendering
        const SPRITE_RENDERING     = 0b0001_0000; // 1: enable sprite rendering
        const EMPHASIZE_RED        = 0b0010_0000; // 1: emphasize red coloring
        const EMPHASIZE_GREEN      = 0b0100_0000; // 1: emphasize green coloring
        const EMPHASIZE_BLUE       = 0b1000_0000; // 1: emphasize blue coloring
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PPUSTATUS: u8 {
        const SPRITE_OVERFLOW = 0b0010_0000;
        const SPRITE_ZERO_HIT = 0b0100_0000;
        const VBLANK          = 0b1000_0000;
    }
}

#[derive(Debug)]
pub struct PPU {

    /* PPU Internal registers */
    v: u16, // Only use 15 bits
    t: u16, // Only use 15 bits
    x: u8, // Only use 3 bits
    w: bool,

    /* PPU Register flags */
    ppuctrl: PPUCTRL,
    ppumask: PPUMASK,
    ppustatus: PPUSTATUS,
    oamaddr: u8,
    read_buffer: u8, // Simulate the read buffer

    /* Rendering attributes */
    scanline: i32,
    dot: i32,
    cycle_count: i32,
    background_rendering_unit: BackgroundRenderingUnit,
    sprite_rendering_units: [SpriteRenderingUnit; 8],

    /* Wiring */
    data_bus: u8, // USED FOR CPU COMMUNICATION
    bus: PPUBus,
}

impl PPU {
    pub fn new(bus: PPUBus) -> Result<Self, String> {
        Ok(Self {
            /* PPU Internal flags */
            v: 0,
            t: 0,
            x: 0,
            w: false,

            /* PPU Register flags */
            ppuctrl: PPUCTRL::empty(),
            ppumask: PPUMASK::empty(),
            ppustatus: PPUSTATUS::VBLANK | PPUSTATUS::SPRITE_OVERFLOW,
            oamaddr: 0,
            read_buffer: 0,

            /* Rendering attributes */
            scanline: 0,
            dot: 0,
            cycle_count: 21, // Starts at 21 PPU cycles (7 cpu cycles * 3)
            background_rendering_unit: BackgroundRenderingUnit::new(),

            // Disregard the current units, will be replaced once sprite data is filled.
            sprite_rendering_units: [SpriteRenderingUnit::new(0, 0, 0, 0); 8],

            /* Wiring */
            data_bus: 0,
            bus: bus,
        })
    }
}

impl PPU {
    pub fn read(&self, address: u16) -> Result<u8, String> {
        self.bus.read(address)
            .map_err(|err| format!("Error reading from PPU VRAM at {address:#06X}: {err}"))
    }

    pub fn write(&mut self, address: u16, value: u8) -> Result<(), String> {
        self.bus.write(address, value)
            .map_err(|err| format!("Error writing to PPU VRAM at {address:#06X}: {err}"))
    }

    // CPU-specific memory-mapped IO functions
    // CPU will NEVER read in the OAMDMA address $4014. Instead, the CPU
    // will handle its own read and writes

    /// Given a memory-mapped address from CPU, return the expected contents.
    /// Will be given a raw address from CPU (e.g. $2002 PPUSTATUS)
    pub fn memory_mapped_read(&mut self, address: u16) -> Result<u8, String> {
        match address {
            // PPUSTATUS
            0x2002 => {
                self.data_bus = self.ppustatus.bits();
                self.w = false; // Always set this to false
                Ok(self.data_bus)
            },

            // OAMDATA
            0x2004 => {
                self.data_bus = self.bus.read_oam(self.oamaddr);
                Ok(self.data_bus)
            }
        
            // PPUDATA
            // TODO: Simulate coarse X or Y increment trigger if called
            // during rendering
            0x2007 => {
                // Simulate 1 call delay
                self.data_bus = self.read_buffer;
                self.read_buffer = self.bus.read(self.v)?;
                self.increment_vram_addr();
                Ok(self.data_bus)
            }

            // All other MMIO read calls just returns whatever was left last
            // in the data bus
            0x2000..=0x2007 => Ok(self.data_bus),
            _ => Err(format!("Address is not a valid memory-mapped address: {:04X}", address)),
        }
    }

    pub fn memory_mapped_write(&mut self, address: u16, value: u8) -> Result<(), String> {
        match address {
            // PPUCTRL
            0x2000 => {
                // Set nametable bytes
                self.t = (self.t & 0x73FF) | u16::from(value & 0x02) << 10;

                self.ppuctrl = PPUCTRL::from_bits(value).expect("Value written to PPUCTRL is invalid.");
                self.data_bus = value;
                Ok(())
            }

            // PPUMASK
            0x2001 => {
                self.ppumask = PPUMASK::from_bits(value).expect("Value written to PPUMASK is invalid.");
                self.data_bus = value;
                Ok(())
            }

            // OAMADDR
            0x2003 => {
                self.oamaddr = value;
                self.data_bus = value;
                Ok(())
            }

            // OAMDATA
            0x2004 => {
                self.bus.write_oam(self.oamaddr, value);
                self.increment_oam_addr();
                self.data_bus = value;
                Ok(())
            }

            // PPUSCROLL
            0x2005 => {
                if self.w {
                    // 2nd write, mimic:
                    // t: FGH..AB CDE..... <- d: ABCDEFGH
                    let fgh = u16::from(value & 0x7) << 12;
                    let abcde  = u16::from(value >> 3) << 5;

                    // Assemble to mask (make all untouched bits equal to 1 with 0xC1F)
                    self.t = fgh | abcde | (self.t & 0x0C1F);
                    self.w = false;
                } else {
                    // 1st write, mimic:
                    // t: ....... ...ABCDE <- d: ABCDE...
                    // x:              FGH <- d: .....FGH
                    let abcde = u16::from(value >> 3);
                    let fgh    = value & 0x7;

                    // Set the registers (mask of 0x7FE0 sets all untouched bits equal to 1)
                    self.t = abcde | (self.t & 0x7FE0);
                    self.x = fgh;
                    self.w = true;
                }

                self.data_bus = value;
                Ok(())
            }

            // PPUADDR
            0x2006 => {
                if self.w {
                    // 2nd write, mimic:
                    // t: ....... ABCDEFGH <- d: ABCDEFGH
                    self.t = (self.t & 0x7F00) | u16::from(value);
                    self.w = false;

                    // TODO: implement a flag where after 1-1.5 cycles, we write t into v
                    // and put this code in the tick() function for PPU
                    self.v = self.t;
                } else {
                    // 1st write, mimic:
                    // t: .CDEFGH ........ <- d: ..CDEFGH
                    // <unused>     <- d: AB......
                    // t: Z...... ........ <- 0 (bit Z is cleared)
                    self.t = (self.t & 0x00FF) | (u16::from(value << 2) << 6);
                    self.w = true;
                }

                self.data_bus = value;
                Ok(())
            }

            // PPUDATA
            // TODO: Simulate coarse X or Y increment trigger if called
            // during rendering
            0x2007 => {
                self.bus.write(self.v, value)?;
                self.increment_vram_addr();
                Ok(())
            }

            // All other MMIO read calls fills the data bus with with whatever
            // value was passed in
            0x2000..=0x2007 => {
                self.data_bus = value;
                Ok(())
            },
            _ => Err(format!("Address is not a valid memory-mapped address: {:04X}", address)),
        }
    }
}

/**
 * Helper functions for PPU methods
 */
impl PPU {

    /// Increments VRAM address by a perdetermined amount based on
    /// the state of bit 2 in $2000
    /// TODO: Capture side effect of incrementing v during rendering,
    /// where it increments coarse X and Y as well
    fn increment_vram_addr(&mut self) -> () {
        self.v += if self.ppuctrl.contains(PPUCTRL::VRAM_ADDR_INC) { 32 } else { 1 };

        // Ensure that it stays within 15 bits (wrapping add)
        self.v = self.v & 0x7FFF;
    }

    /// Increments OAM address using wrapping add
    fn increment_oam_addr(&mut self) -> () {
        self.oamaddr = self.oamaddr.wrapping_add(1);
    }
}

// Testing matrix:
// - writes and reads of every $2000-$2007 register
// - a $2002 read between the two writes of $2005 and of $2006

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::test_cartridge;

    fn new_ppu() -> PPU {
        PPU::new(PPUBus::new(test_cartridge())).unwrap()
    }

    fn write(ppu: &mut PPU, address: u16, value: u8) {
        ppu.memory_mapped_write(address, value).unwrap();
    }

    fn read(ppu: &mut PPU, address: u16) -> u8 {
        ppu.memory_mapped_read(address).unwrap()
    }

    /// Byte at `addr`, as a later `$2007` read would return it.
    fn read_vram(ppu: &mut PPU, addr: u16) -> u8 {
        read(ppu, 0x2002);
        write(ppu, 0x2006, (addr >> 8) as u8);
        write(ppu, 0x2006, addr as u8);
        let _stale = read(ppu, 0x2007);
        read(ppu, 0x2007)
    }

    #[test]
    fn test_write_only_registers_read_back_the_last_bus_byte() {
        let mut ppu = new_ppu();

        write(&mut ppu, 0x2000, 0xA4);
        assert_eq!(read(&mut ppu, 0x2000), 0xA4);

        write(&mut ppu, 0x2001, 0x18);
        assert_eq!(read(&mut ppu, 0x2001), 0x18);
        assert_eq!(read(&mut ppu, 0x2005), 0x18);
        assert_eq!(read(&mut ppu, 0x2006), 0x18);

        // $2003 selects OAM. A read of $2003 itself is still the bus byte.
        write(&mut ppu, 0x2003, 0x10);
        assert_eq!(read(&mut ppu, 0x2005), 0x10);
        assert_eq!(read(&mut ppu, 0x2004), 0x00);

        // Writing $2002 does not clear the write latch: the following $2006
        // is still the low byte, so the data lands at $2040.
        write(&mut ppu, 0x2006, 0x20);
        write(&mut ppu, 0x2002, 0x7F);
        assert_eq!(read(&mut ppu, 0x2005), 0x7F);
        write(&mut ppu, 0x2006, 0x40);
        write(&mut ppu, 0x2007, 0x5A);
        assert_eq!(read_vram(&mut ppu, 0x2040), 0x5A);
        assert_eq!(read_vram(&mut ppu, 0x2000), 0x00);
    }

    #[test]
    fn test_ppustatus_read_returns_powerup_flags() {
        let mut ppu = new_ppu();

        assert_eq!(
            read(&mut ppu, 0x2002),
            (PPUSTATUS::VBLANK | PPUSTATUS::SPRITE_OVERFLOW).bits()
        );
        assert_eq!(read(&mut ppu, 0x2000), 0xA0);
    }

    #[test]
    fn test_ppuscroll_second_write_changes_where_ppudata_lands() {
        let mut ppu = new_ppu();

        // High address, vertical scroll, then a horizontal scroll so the latch is
        // armed again, then the low address. $C2 moves the byte from $2010 to $2310.
        write(&mut ppu, 0x2006, 0x20);
        write(&mut ppu, 0x2005, 0xC2);
        write(&mut ppu, 0x2005, 0x00);
        write(&mut ppu, 0x2006, 0x10);
        write(&mut ppu, 0x2007, 0x5A);

        assert_eq!(read_vram(&mut ppu, 0x2310), 0x5A);
        assert_eq!(read_vram(&mut ppu, 0x2010), 0x00);
    }

    #[test]
    fn test_ppustatus_read_restarts_ppuscroll_writes() {
        let mut ppu = new_ppu();

        // The status read clears the latch, so $C2 is a horizontal scroll write.
        // The byte lands at $2010 instead of $2310.
        write(&mut ppu, 0x2006, 0x20);
        read(&mut ppu, 0x2002);
        write(&mut ppu, 0x2005, 0xC2);
        write(&mut ppu, 0x2006, 0x10);
        write(&mut ppu, 0x2007, 0x5A);

        assert_eq!(read_vram(&mut ppu, 0x2010), 0x5A);
        assert_eq!(read_vram(&mut ppu, 0x2310), 0x00);
    }

    #[test]
    fn test_ppuaddr_second_write_selects_the_stored_byte() {
        let mut ppu = new_ppu();

        write(&mut ppu, 0x2006, 0x2C);
        write(&mut ppu, 0x2006, 0x34);
        write(&mut ppu, 0x2007, 0xAB);

        assert_eq!(read_vram(&mut ppu, 0x2C34), 0xAB);
        assert_eq!(read_vram(&mut ppu, 0x2C00), 0x00);
    }

    #[test]
    fn test_ppustatus_read_restarts_ppuaddr_writes() {
        let mut ppu = new_ppu();

        // Status makes $21 another high byte. The low byte is the following $40,
        // so the data is at $2140. Without the status read it would be at $2021.
        write(&mut ppu, 0x2006, 0x20);
        read(&mut ppu, 0x2002);
        write(&mut ppu, 0x2006, 0x21);
        write(&mut ppu, 0x2006, 0x40);
        write(&mut ppu, 0x2007, 0xAB);

        assert_eq!(read_vram(&mut ppu, 0x2140), 0xAB);
        assert_eq!(read_vram(&mut ppu, 0x2021), 0x00);
    }

    #[test]
    fn test_ppuaddr_after_scroll_still_addresses_the_requested_byte() {
        let mut ppu = new_ppu();

        // Second scroll write sets the high address bit. The following address
        // writes still store at $2000.
        write(&mut ppu, 0x2005, 0x00);
        write(&mut ppu, 0x2005, 0x07);
        write(&mut ppu, 0x2006, 0x20);
        write(&mut ppu, 0x2006, 0x00);
        write(&mut ppu, 0x2007, 0x5A);

        assert_eq!(read_vram(&mut ppu, 0x2000), 0x5A);
    }

    #[test]
    fn test_oamdata_write_is_read_back_at_that_slot_only() {
        let mut ppu = new_ppu();

        write(&mut ppu, 0x2003, 0x10);
        write(&mut ppu, 0x2004, 0x99);
        assert_eq!(read(&mut ppu, 0x2004), 0x00);
        assert_eq!(read(&mut ppu, 0x2004), 0x00);

        write(&mut ppu, 0x2003, 0x10);
        assert_eq!(read(&mut ppu, 0x2004), 0x99);
        assert_eq!(read(&mut ppu, 0x2004), 0x99);
    }

    #[test]
    fn test_oamdata_write_at_ff_is_read_back_from_ff() {
        let mut ppu = new_ppu();

        write(&mut ppu, 0x2003, 0xFF);
        write(&mut ppu, 0x2004, 0x5A);
        assert_eq!(read(&mut ppu, 0x2004), 0x00);

        write(&mut ppu, 0x2003, 0xFF);
        assert_eq!(read(&mut ppu, 0x2004), 0x5A);
        assert_eq!(read(&mut ppu, 0x2004), 0x5A);
    }

    #[test]
    fn test_ppudata_read_returns_the_previous_byte() {
        let mut ppu = new_ppu();

        write(&mut ppu, 0x2006, 0x2C);
        write(&mut ppu, 0x2006, 0x34);
        write(&mut ppu, 0x2007, 0x11);
        write(&mut ppu, 0x2007, 0x22);

        write(&mut ppu, 0x2006, 0x2C);
        write(&mut ppu, 0x2006, 0x34);
        assert_eq!(read(&mut ppu, 0x2007), 0x00);
        assert_eq!(read(&mut ppu, 0x2007), 0x11);
        assert_eq!(read(&mut ppu, 0x2007), 0x22);
    }

    #[test]
    fn test_ppudata_second_byte_lands_32_ahead_when_ctrl_bit_set() {
        let mut ppu = new_ppu();

        write(&mut ppu, 0x2000, 0x04);
        write(&mut ppu, 0x2006, 0x20);
        write(&mut ppu, 0x2006, 0x00);
        write(&mut ppu, 0x2007, 0x77);
        write(&mut ppu, 0x2007, 0x88);

        assert_eq!(read_vram(&mut ppu, 0x2000), 0x77);
        assert_eq!(read_vram(&mut ppu, 0x2020), 0x88);
        assert_eq!(read_vram(&mut ppu, 0x2001), 0x00);
    }

    #[test]
    fn test_mmio_rejects_address_outside_registers() {
        let mut ppu = new_ppu();

        assert!(ppu.memory_mapped_read(0x4014).is_err());
        assert!(ppu.memory_mapped_write(0x4014, 0x00).is_err());
    }
}