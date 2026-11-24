//! Isolates rendering functionality for the PPU

use crate::ppu::PPU;

/// Rendering helper functions
impl PPU {
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
}