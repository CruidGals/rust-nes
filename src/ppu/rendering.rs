//! Isolates rendering functionality for the PPU

use crate::ppu::{PPU, PPUMASK, PPUSTATUS};

/// Actual rendering logic
impl PPU {
    /// Given a background unit and a set of sprite units (from the PPU),
    /// determine the pixel that is drawn at the timestamp.
    /// 
    /// NOTE: This function shifts the shift registers. Do NOT shift the
    /// registers after this function is called, as this function will
    /// handle it.
    pub fn determine_pixel(&mut self) -> u8 {
        // Forced blanking: since background rendering and sprite rendering
        // is disabled, we don't actually shift the register. Instead, we
        // force return a full zero pixel
        if !self.ppumask.intersects(PPUMASK::BACKGROUND_RENDERING | PPUMASK::SPRITE_RENDERING) {
            // Quirk, if v register points into palette ram, then the PPU outputs the color
            // at v & 0x1F instead of 0
            if (self.v & 0x3FFF) >= 0x3F00 {
                return (self.v & 0x1F) as u8;
            }

            return 0;
        }

        // BACKGROUND DATA RETRIEVAL
        // 
        // Read from the singular background retrieval unit. If background
        // rendering is turned off, mark it with a zero
        let mut bg_pixel_data = self.background_rendering_unit.read(self.x);

        if !self.ppumask.contains(PPUMASK::BACKGROUND_RENDERING) {
            bg_pixel_data = 0;
        }

        // Background clipping
        if self.dot <= 8 && !self.ppumask.contains(PPUMASK::BACKGROUND_LEFT_SHOW) {
            bg_pixel_data = 0;
        }

        // SPRITE DATA RETRIEVAL
        // 
        // When retrieving the pixel data for sprite, we loop through them
        // sequentially (from sprite 0 to sprite 7) and grab the first sprite
        // that contains an opaque pixel. This handles an "overlapping" priority
        // feature where sprites numbered closer to zero get selected first if
        // they are overlapping. 
        //
        // Once this pixel is found, each sprite unit must still shift its
        // registers. On each read, down counters are decremented which must
        // happen to ensure alignment of the dot with the sprite's entry into
        // positioning.

        // Find the first sprite that contains an opaque pixel
        let mut sprite_pixel_data: u8 = 0;
        let mut sprite_zero_hit = false;

        for unit in &mut self.sprite_rendering_units {
            let data = unit.read();

            // Skip if found, store if its opaque
            if (sprite_pixel_data & 0x03) == 0 && (data & 0x03) != 0 {
                
                // Check if this is Sprite 0
                if unit.is_zero_sprite {
                    sprite_zero_hit = true;
                }

                sprite_pixel_data = data;
            }
        }

        if !self.ppumask.contains(PPUMASK::SPRITE_RENDERING) {
            sprite_pixel_data = 0;
        }

        // Sprite clipping
        if self.dot <= 8 && !self.ppumask.contains(PPUMASK::SPRITE_LEFT_SHOW) {
            sprite_pixel_data = 0;
        }

        // Triggered a sprite zero hit
        if sprite_zero_hit &&
           (bg_pixel_data & 0x03) != 0 &&
           (sprite_pixel_data & 0x03) != 0 &&
           self.dot != 256 {
            self.ppustatus.set(PPUSTATUS::SPRITE_ZERO_HIT, true);
        }

        // With the data, return the value given by priority multiplexer
        priority_multiplexer(bg_pixel_data, sprite_pixel_data)
    }
}

/// Multiplexer used to determine if the background or sprite
/// pixel has priority to be rendered first. Takes in arguments
/// in the form of these bits:
/// 
/// background_data
/// ----
/// AAPP
/// ||||
/// ||++- Pixel value from tile pattern data
/// ++--- Palette number from attributes
/// 
/// sprite_data
/// -----
/// pAAPP
/// |||||
/// |||++- Pixel value from tile pattern data
/// |++--- Palette number from attributes
/// +----- Sprite priority flag
/// 
/// Returns a u8 in the form: (taken from NESDev WIKI)
/// 
/// 4bit0
/// -----
/// SAAPP
/// |||||
/// |||++- Pixel value from tile pattern data
/// |++--- Palette number from attributes
/// +----- Background/Sprite select
/// 
fn priority_multiplexer(bg_data: u8, sprite_data: u8) -> u8 {
    let bg_opaque = (bg_data & 0x03) > 0;
    let sprite_opaque = (sprite_data & 0x03) > 0;
    let sprite_priority = (sprite_data & 0x10) != 0;

    let bg_result = bg_data & 0x0F;
    let sprite_result = 0x10 | (sprite_data & 0x0F);

    match (bg_opaque, sprite_opaque, sprite_priority) {
        // Background and sprite transparent
        (false, false, _) => { 0 },

        // Sprite opaque
        (false, true, _) => { sprite_result },

        // Background opaque
        (true, false, _) => { bg_result },

        // Both opaque, sprite priority
        (true, true, false) => { sprite_result },

        // Both opaque, bg priority
        (true, true, true) => { bg_result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::test_cartridge;
    use crate::ppu::ppubus::PPUBus;
    use crate::ppu::sprite_rendering_unit::SpriteRenderingUnit;

    fn new_ppu() -> PPU {
        PPU::new(PPUBus::new(test_cartridge())).unwrap()
    }

    /// Puts a single opaque background pixel (pattern 1, palette 0) on fine X 0.
    fn load_background_pixel(ppu: &mut PPU) {
        ppu.background_rendering_unit.pattern_low_latch = 0x80;
        ppu.background_rendering_unit.pattern_high_latch = 0x00;
        ppu.background_rendering_unit.load_pattern_latch_into_register();
        for _ in 0..8 {
            ppu.background_rendering_unit.shift_pattern_registers();
        }
        ppu.x = 0;
    }

    fn enable_rendering(ppu: &mut PPU, background: bool, sprites: bool) {
        ppu.ppumask = PPUMASK::empty();
        if background {
            ppu.ppumask.insert(PPUMASK::BACKGROUND_RENDERING | PPUMASK::BACKGROUND_LEFT_SHOW);
        }
        if sprites {
            ppu.ppumask.insert(PPUMASK::SPRITE_RENDERING | PPUMASK::SPRITE_LEFT_SHOW);
        }
    }

    #[test]
    fn priority_multiplexer_selects_the_documented_pixel() {
        // Both pattern values are 0, so both are transparent.
        assert_eq!(priority_multiplexer(0x00, 0x00), 0x00);
        assert_eq!(priority_multiplexer(0x04, 0x10), 0x00);

        // Only the sprite has a pattern. Bit 4 of the result marks a sprite.
        assert_eq!(priority_multiplexer(0x00, 0x01), 0x11);
        assert_eq!(priority_multiplexer(0x04, 0x01), 0x11);

        // Only the background has a pattern. The high bits of the input are ignored.
        assert_eq!(priority_multiplexer(0x07, 0x00), 0x07);
        assert_eq!(priority_multiplexer(0xF7, 0x10), 0x07);

        // Both opaque. Priority bit clear: the sprite is in front.
        assert_eq!(priority_multiplexer(0x07, 0x03), 0x13);
        assert_eq!(priority_multiplexer(0xF7, 0xE3), 0x13);

        // Both opaque. Priority bit set: the background is in front.
        assert_eq!(priority_multiplexer(0x07, 0x13), 0x07);
    }

    #[test]
    fn determine_pixel_returns_the_visible_pixel() {
        let mut ppu = new_ppu();
        load_background_pixel(&mut ppu);
        ppu.dot = 9;

        assert_eq!(ppu.determine_pixel(), 0x00);
        enable_rendering(&mut ppu, true, false);
        assert_eq!(ppu.determine_pixel(), 0x01);

        ppu.v = 0x3F15;
        ppu.ppumask = PPUMASK::empty();
        assert_eq!(ppu.determine_pixel(), 0x15);

        load_background_pixel(&mut ppu);
        ppu.ppumask = PPUMASK::BACKGROUND_RENDERING;
        ppu.dot = 8;
        assert_eq!(ppu.determine_pixel(), 0x00);

        load_background_pixel(&mut ppu);
        enable_rendering(&mut ppu, true, true);
        ppu.dot = 9;
        ppu.sprite_rendering_units[0] = SpriteRenderingUnit::new(0x80, 0x00, 0x00, 0, true);
        assert_eq!(ppu.determine_pixel(), 0x11);
        assert_eq!(ppu.memory_mapped_read(0x2002).unwrap(), 0xE0);

        let mut ppu = new_ppu();
        load_background_pixel(&mut ppu);
        enable_rendering(&mut ppu, true, true);
        ppu.dot = 9;
        ppu.sprite_rendering_units[0] = SpriteRenderingUnit::new(0x80, 0x00, 0b100, 0, true);
        assert_eq!(ppu.determine_pixel(), 0x01);
        assert_eq!(ppu.memory_mapped_read(0x2002).unwrap(), 0xE0);

        let mut ppu = new_ppu();
        ppu.ppumask = PPUMASK::SPRITE_RENDERING;
        ppu.dot = 1;
        ppu.sprite_rendering_units[0] = SpriteRenderingUnit::new(0x80, 0x00, 0x00, 0, true);
        assert_eq!(ppu.determine_pixel(), 0x00);
        assert_eq!(ppu.memory_mapped_read(0x2002).unwrap(), 0xA0);

        let mut ppu = new_ppu();
        load_background_pixel(&mut ppu);
        enable_rendering(&mut ppu, true, true);
        ppu.dot = 256;
        ppu.sprite_rendering_units[0] = SpriteRenderingUnit::new(0x80, 0x00, 0x00, 0, true);
        assert_eq!(ppu.determine_pixel(), 0x11);
        assert_eq!(ppu.memory_mapped_read(0x2002).unwrap(), 0xA0);
    }
}