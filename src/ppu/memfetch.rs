//! Implements a function to fetch memory from VRAM in order of:
//! 1. Nametable byte
//! 2. Attribute byte
//! 3. Pattern table low byte
//! 4. Pattern table high byte
//! Each of these takes two cycles.

use crate::ppu::{PPU, PPUCTRL, sprite_rendering_unit::SpriteRenderingUnit};

impl PPU {

    /// Performs the 8 cycle memory fetch step to fill the latches in
    /// the background rendering unit
    fn step_background_memory_fetch(&mut self) -> Result<(), String> {
        // Since the flow takes 8 cycles, we can use modulo to effectively
        // split operations

        let operation = (self.dot - 1) % 8;

        // Odd operations (1, 3, 5, 7) acutally do something. Even ones get disregarded
        match operation {
            1 => {
                // Grab nametable byte (taken from NESdev wiki)
                let address = 0x2000 | (self.v & 0x0FFF);
                self.background_rendering_unit.nametable_latch = self.read(address)?;
            }
            3 => {
                // Grab attribute table byte (taken from NESdev wiki)
                let address = 0x23C0 | (self.v & 0x0C00) | ((self.v >> 4) & 0x38) | ((self.v >> 2) & 0x07);
                self.background_rendering_unit.attribute_next_tile_data_latch = self.read(address)?;
            }
            5 => {
                // Grab pattern table data low byte
                let address = self.pattern_table_background_byte_addr(
                    self.background_rendering_unit.nametable_latch, 
                    false);
                
                self.background_rendering_unit.pattern_low_latch = self.read(address)?;
            }
            7 => {
                // Grab pattern table data high byte
                let address = self.pattern_table_background_byte_addr(
                    self.background_rendering_unit.nametable_latch, 
                    true);
                
                self.background_rendering_unit.pattern_high_latch = self.read(address)?;
            }
            _ => {}
        }

        Ok(())
    }

    /// Performs the 8 cycle memory fetch step to fill the latches in
    /// the sprite rendering unit. Separate for each sprite.
    fn step_sprite_memory_fetch(&mut self, sprite_num: usize) -> Result<(), String> {
        let operation = (self.dot - 1) % 8;

        // Offset indexing to secondary OAM
        let sprite_idx_offset = (sprite_num * 4) as u8;

        // More than just odd operations seem to do something
        match operation {
            0 => {
                // Y-coordinate (fine y) is calculated and stored
                // Sprite y-pos are stored as top_scanline - 1. So we can easily find
                // the fine y by subtracting the current_scanline we are on by this one
                let byte = u16::from(self.bus.read_secondary_oam(sprite_idx_offset)?);

                // If its $FF (or over scaline 239) then its out of bounds
                self.sprite_fine_y = if self.sprite_range_check(byte) {
                    self.sprite_empty = true;
                    0
                } else { 
                    self.sprite_empty = false;
                    self.scanline - byte
                };
            },
            1 => {
                // Tile id (nametable byte) of the sprite
                self.sprite_tile_id = self.bus.read_secondary_oam(sprite_idx_offset + 1)?;

                // Dummy nametable fetch
                let address = 0x2000 | (self.v & 0x0FFF);
                self.background_rendering_unit.nametable_latch = self.read(address)?;
            },
            2 => {
                // Grab attribute bits and priority bit
                // Must perform the vertical and/or horizontal flips here
                let attribute_byte = self.bus.read_secondary_oam(sprite_idx_offset + 2)?;

                // Check vertical flip
                if (attribute_byte >> 7) == 0x01 {
                    // Invert the row that we are iterating on
                    self.sprite_fine_y = if self.ppuctrl.contains(PPUCTRL::SPRITE_SIZE) { 15 - self.sprite_fine_y } else { 7 - self.sprite_fine_y };
                }

                // Check horizontal flip
                if ((attribute_byte >> 6) & 0x01) == 0x01 {
                    // Mark this as something to invert later
                    self.horizontal_flip = true;
                } else {
                    self.horizontal_flip = false;
                }

                // Build the 3 bits
                self.sprite_rendering_units[sprite_num].attribute_data = ((attribute_byte >> 3) & 0x04) | (attribute_byte & 0x03);
            },
            3 => {
                // Grab the x-position and store it in the down counter
                self.sprite_rendering_units[sprite_num].down_counter = self.bus.read_secondary_oam(sprite_idx_offset + 3)?;

                // Dummy nametable fetch
                let address = 0x2000 | (self.v & 0x0FFF);
                self.background_rendering_unit.nametable_latch = self.read(address)?;
            },
            5 => {
                // Save pattern table low into shift register
                let address = self.pattern_table_sprite_byte_addr( false);
                let byte = self.read(address)?;

                // If flagged to be out of bounds, set it as zero and break early
                if self.sprite_empty {
                    self.sprite_rendering_units[sprite_num].pattern_low_shift_reg = 0x00;
                } else  {
                    // Flip horizontally
                    self.sprite_rendering_units[sprite_num].pattern_low_shift_reg = if self.horizontal_flip { byte.reverse_bits() } else { byte };
                }
            },
            7 => {
                // Save pattern table low into shift register
                let address = self.pattern_table_sprite_byte_addr( true);
                let byte = self.read(address)?;

                // If flagged to be out of bounds, set it as zero and break early
                if self.sprite_empty {
                    self.sprite_rendering_units[sprite_num].pattern_high_shift_reg = 0x00;
                } else  {
                    // Flip horizontally
                    self.sprite_rendering_units[sprite_num].pattern_high_shift_reg = if self.horizontal_flip { byte.reverse_bits() } else { byte };
                }
            },
            _ => {},
        }

        Ok(())
    }

    /// Helper function to grab the address for a byte in the pattern table.
    /// Must specify the nametable byte and whether or not you want to grab
    /// the high byte or not.
    fn pattern_table_background_byte_addr(&self, nametable: u8, high: bool) -> u16 {
        return if self.ppuctrl.contains(PPUCTRL::BACKGROUND_PATTERN_ADDR) { 1 << 12 } else { 0 } | // Which pattern table
               u16::from(nametable) << 4 | // Nametable
               if high { 1 << 3 } else { 0 } |  // Bit plane
               self.v >> 12; // Fine y
    }

    fn pattern_table_sprite_byte_addr(&self, high: bool) -> u16 {
        if self.ppuctrl.contains(PPUCTRL::SPRITE_SIZE) {
            // 8x16 sprite
            return u16::from(self.sprite_tile_id & 0x01) << 12 | // Pattern table
                   u16::from(self.sprite_tile_id & 0xFE) << 4 |  // Top tile id
                   (self.sprite_fine_y & 0x08) << 1 |            // bit 3 of fine_y picks top or bot tile
                   if high { 1 << 3 } else { 0 } |               // Bit plane
                   (self.sprite_fine_y & 0x07);
        }

        return if self.ppuctrl.contains(PPUCTRL::SPRITE_PATTERN_ADDR) { 1 << 12 } else { 0 } | // Pattern table
               u16::from(self.sprite_tile_id) << 4 |  // Top tile id
               if high { 1 << 3 } else { 0 } |               // Bit plane
               (self.sprite_fine_y & 0x07);
    }
    
    /// Returns a boolean saying whether the sprite is either empty
    /// or out of bounds
    fn sprite_range_check(&self, y_index: u16) -> bool {
        return self.scanline == 261 ||               // Cannot check on pre-render scanline
               y_index > 239 ||                      // Empty
               self.scanline < y_index ||            // Y coordinate is below scanline
               if self.ppuctrl.contains(PPUCTRL::SPRITE_SIZE) { self.scanline - y_index >= 16 } else { self.scanline - y_index >= 8 } // Imposible bounds
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{cpu::test_cartridge, ppu::ppubus::PPUBus};

    fn new_ppu() -> PPU {
        PPU::new(PPUBus::new(test_cartridge())).unwrap()
    }

    #[test]
    fn pattern_table_background_byte_addr_fetchs_correctly() {
        let mut ppu = new_ppu();
        let nametable: u8 = 0b1001_1011;
        let fine_y: u16 = 0b110;
        
        // Try $0xxx page
        ppu.ppuctrl.remove(PPUCTRL::BACKGROUND_PATTERN_ADDR);
        ppu.v = (ppu.v | 0x0FFF) | (fine_y << 12);

        assert_eq!(ppu.pattern_table_background_byte_addr(nametable, true), 0b0000_1001_1011_1110);
        assert_eq!(ppu.pattern_table_background_byte_addr(nametable, false), 0b0000_1001_1011_0110);

        // Try $1xxx page
        ppu.ppuctrl.insert(PPUCTRL::BACKGROUND_PATTERN_ADDR);
        assert_eq!(ppu.pattern_table_background_byte_addr(nametable, true), 0b0001_1001_1011_1110);
        assert_eq!(ppu.pattern_table_background_byte_addr(nametable, false), 0b0001_1001_1011_0110);
    }

    #[test]
    fn pattern_table_sprite_byte_addr_fetchs_correctly() {
        let mut ppu = new_ppu();
        ppu.sprite_tile_id = 0b1001_1011;
        ppu.sprite_fine_y = 0b110;
        
        // Try $0xxx page
        ppu.ppuctrl.remove(PPUCTRL::SPRITE_PATTERN_ADDR);
        assert_eq!(ppu.pattern_table_sprite_byte_addr(true), 0b0000_1001_1011_1110);
        assert_eq!(ppu.pattern_table_sprite_byte_addr(false), 0b0000_1001_1011_0110);

        // Try $1xxx page
        ppu.ppuctrl.insert(PPUCTRL::SPRITE_PATTERN_ADDR);
        assert_eq!(ppu.pattern_table_sprite_byte_addr(true), 0b0001_1001_1011_1110);
        assert_eq!(ppu.pattern_table_sprite_byte_addr(false), 0b0001_1001_1011_0110);

        // Try with 8x16 sprite now
        ppu.ppuctrl.insert(PPUCTRL::SPRITE_SIZE);

        // Try $0xxx page
        ppu.ppuctrl.remove(PPUCTRL::SPRITE_PATTERN_ADDR);
        assert_eq!(ppu.pattern_table_sprite_byte_addr(true), 0b0001_1001_1010_1110);
        assert_eq!(ppu.pattern_table_sprite_byte_addr(false), 0b0001_1001_1010_0110);
 
        // Try $1xxx page (8x16 should ignore this)
        ppu.ppuctrl.insert(PPUCTRL::SPRITE_PATTERN_ADDR);
        assert_eq!(ppu.pattern_table_sprite_byte_addr(true), 0b0001_1001_1010_1110);
        assert_eq!(ppu.pattern_table_sprite_byte_addr(false), 0b0001_1001_1010_0110);

    }
}