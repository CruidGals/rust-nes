//! Sprite vs background pixel priority
//! 
//! 1. Sprite vs. sprite: output units find the first sprite
//! that contains an opaque pixel (non backdrop). Say sprite 0
//! and sprite 2 contain opaque pixels. Sprite 0 wins.
//! 
//! 2. Sprite vs. background. Based on the priority of sprite,
//! and sprite and background opaqueness vs transparency, one
//! or the other wins. Look at the wiki for correct multiplexer
//! information.

#[derive(Debug, Copy, Clone)]
pub struct SpriteRenderingUnit {
    /// Picks the color in the palette that the sprite should use. 
    /// These registers only start shifting the once x-counter reaches
    /// zero. Otherwise, the unit will keeping outputting 0 (backdrop color)
    /// These registers are loaded in from secondary OAM before the next
    /// scanline. They get filled with zeros after being shifted because
    /// once you exhaust the sprite's pattern data it should only output
    /// backdrop data.
    pattern_low_shift_reg: u8,
    pattern_high_shift_reg: u8,

    /// Attribute data. Bits 0 and 1 represent which palette is chosen.
    /// Bit 2 represents whether this sprite has priority or not.
    attribute_data: u8, // Only 3 bits are used

    /// The x-position of the sprite is set here. Every cycle, this down
    /// counts towards zero. Once it is at zero, the shift registers can
    /// start shifting. This perfectly times the unit to emit sprite pixels
    /// once found the sprite
    down_counter: u8,
}

impl SpriteRenderingUnit {
    pub fn new(pattern_low: u8, pattern_high: u8, attribute_data: u8, x_pos: u8) -> Self {
        // If the data being passed in here is not right, then that represents
        // a real error in code or progrma
        if attribute_data >> 3 > 0 {
            panic!("Attribute data being passed into SpriteRenderingUnit is greater than 3 bytes wide.");
        }

        Self {
            pattern_low_shift_reg: pattern_low,
            pattern_high_shift_reg: pattern_high,

            attribute_data: attribute_data,
            down_counter: x_pos,
        }
    }
}

impl SpriteRenderingUnit {
    /// Read the bit from the unit. Returns backdrop color if down_counter isn't
    /// at zero yet. Handles decrementing of down_counter, and returning of bit
    pub fn read(&mut self) -> u8 {
        // Default to returning backdrop color
        let mut pattern_table_bits: u8 = 0;

        if self.down_counter > 0 {
            self.down_counter -= 1;
        } else {
            pattern_table_bits = ((self.pattern_high_shift_reg >> 6) & 0x02) | (self.pattern_low_shift_reg >> 7);
            self.shift_pattern_registers();
        }

        // Build the output (5 bits)
        (self.attribute_data << 2) | pattern_table_bits
    }

    /// Shifts the pattern registers left. Fills in with zeros from the right.
    /// Does not shift unless down_counter is zero.
    fn shift_pattern_registers(&mut self) -> () {
        if self.down_counter != 0 { () }

        self.pattern_low_shift_reg <<= 1;
        self.pattern_high_shift_reg <<= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_words() {
        let pattern_low = 0b1101_0101;
        let pattern_high = 0b0111_0010;
        let attribute_data = 0b0000_0110;
        let down_counter = 123;

        let mut unit = SpriteRenderingUnit::new(
            pattern_low, pattern_high, attribute_data, down_counter
        );

        assert_eq!(unit.pattern_low_shift_reg, 0b1101_0101);
        assert_eq!(unit.pattern_high_shift_reg, 0b0111_0010);
        assert_eq!(unit.attribute_data, 0b0000_0110);
        assert_eq!(unit.down_counter, 123);

        for i in 0..down_counter {
            let data = unit.read();

            // Ensure its a backdrop color
            assert_eq!(data, 0b0001_1000);
            assert_eq!(unit.down_counter, 122 - i);
        }

        assert_eq!(unit.read(), 0b0001_1001);
        assert_eq!(unit.read(), 0b0001_1011);
        assert_eq!(unit.read(), 0b0001_1010);
        assert_eq!(unit.read(), 0b0001_1011);
        assert_eq!(unit.read(), 0b0001_1000);
        assert_eq!(unit.read(), 0b0001_1001);
        assert_eq!(unit.read(), 0b0001_1010);
        assert_eq!(unit.read(), 0b0001_1001);

        // Ensure that future reads just yield backdrop
        for _ in 0..20 {
            let data = unit.read();

            // Ensure its a backdrop color
            assert_eq!(data, 0b0001_1000);
            assert_eq!(unit.down_counter, 0);
        }
    }
}