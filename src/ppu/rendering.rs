pub struct RenderingInternals {
    enabled: bool,

    // Pattern table latches and registers. These shift registers
    // shift left every dot. Low shift register gets filled with zeros
    // from the right, high shift register gets filled with ones.
    // After every 8 dots, (or an inc hori(v)), the latch contents
    // get placed in the low byte of the registers.
    pattern_low_shift_reg: u16,
    pub pattern_low_latch: u8,
    pattern_high_shift_reg: u16,
    pub pattern_high_latch: u8,

    // Attribute table latches and registers. These shift registers
    // shift left every dot, and whatever bit is in the latches gets
    // pushed into each register from the right. After every 8 dots,
    // (or an inc hori(v)), the latches get filled with whatever is
    // in the next tile data latch. The bits are chosen via multiplexer
    // given bit 1 of coarse x and bit 1 of coarse y.
    //
    // Here is a table of how these bytes are chosen:
    //
    // coarse y bit 1 | coarse x bit 1 |   quadrant   | attribute_bot_latch | attribute_top_latch
    // ---------------|----------------|--------------|---------------------|--------------------
    //        0       |        0       | top left     |   Attribute bit 0   |   Attribute bit 1
    //        0       |        1       | top right    |   Attribute bit 2   |   Attribute bit 3
    //        1       |        0       | bottom left  |   Attribute bit 4   |   Attribute bit 5
    //        0       |        0       | bottom right |   Attribute bit 6   |   Attribute bit 7
    attribute_bot_shift_reg: u8,
    attribute_bot_latch: u8, // Misleading, only 1 bit wide
    attribute_top_shift_reg: u8,
    attribute_top_latch: u8,  // Misleading, only 1 bit wide
    pub attribute_next_tile_data_latch: u8,

    // Nametable latch.
    // TODO: put some sort of whimsical knowledge here
    pub nametable_latch: u8,
}

impl RenderingInternals {
    pub fn new() -> Self {
        Self {
            enabled: false,

            pattern_low_shift_reg: 0,
            pattern_low_latch: 0,
            pattern_high_shift_reg: 0,
            pattern_high_latch: 0,
            
            attribute_bot_shift_reg: 0,
            attribute_bot_latch: 0,
            attribute_top_shift_reg: 0,
            attribute_top_latch: 0,
            attribute_next_tile_data_latch: 0,

            nametable_latch: 0,
        }
    }

    // Shift register specific

    /// Shifts both pattern registers. Low pattern register gets filled 
    /// with zeros, high pattern register gets filled with ones.
    pub fn shift_pattern_registers(&mut self) -> () {
        // Shift the registers
        self.pattern_low_shift_reg <<= 1;
        self.pattern_high_shift_reg = (self.pattern_high_shift_reg << 1) | 0x0001;
    }

    /// Shifts both attribute registers. Both attribute registers get
    /// filled with whatever their latches carry.
    pub fn shift_attribute_registers(&mut self) -> () {
        // Shift the registers
        self.attribute_bot_shift_reg = (self.attribute_bot_shift_reg << 1) | self.attribute_bot_latch;
        self.attribute_top_shift_reg = (self.attribute_top_shift_reg << 1) | self.attribute_top_latch;
    }

    // Load latch values into register
    pub fn load_pattern_latch_into_register(&mut self) -> () {
        self.pattern_low_shift_reg = (self.pattern_low_shift_reg & 0xFF00) | u16::from(self.pattern_low_latch);
        self.pattern_high_shift_reg = (self.pattern_high_shift_reg & 0xFF00) | u16::from(self.pattern_high_latch);
    }

    pub fn load_attribute_bit_latch(&mut self, coarse_x_bit: u8, coarse_y_bit: u8) -> Result<(), String> {
        // Mimic the multiplexer
        match (coarse_y_bit, coarse_x_bit) {
            (0, 0) => {
                self.attribute_bot_latch = self.attribute_next_tile_data_latch & 0x01;
                self.attribute_top_latch = (self.attribute_next_tile_data_latch >> 1) & 0x01;
                Ok(())
            },
            (0, 1) => {
                self.attribute_bot_latch = (self.attribute_next_tile_data_latch >> 2) & 0x01;
                self.attribute_top_latch = (self.attribute_next_tile_data_latch >> 3) & 0x01;
                Ok(())
            },
            (1, 0) => {
                self.attribute_bot_latch = (self.attribute_next_tile_data_latch >> 4) & 0x01;
                self.attribute_top_latch = (self.attribute_next_tile_data_latch >> 5) & 0x01;
                Ok(())
            },
            (1, 1) => {
                self.attribute_bot_latch = (self.attribute_next_tile_data_latch >> 6) & 0x01;
                self.attribute_top_latch = (self.attribute_next_tile_data_latch >> 7) & 0x01;
                Ok(())
            }
            _ => Err(format!("Coarse bits are not given in bit form: coarse_x_bit: {} coarse_y_bit: {}", coarse_x_bit, coarse_y_bit))
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    fn internal_fixture() -> RenderingInternals {
        let mut internals = RenderingInternals::new();

        internals.pattern_low_shift_reg  = 0b1001_1011_0111_1111;
        internals.pattern_high_shift_reg = 0b1011_1000_0010_0000;

        internals.attribute_bot_shift_reg = 0b1111_0000;
        internals.attribute_top_shift_reg = 0b0000_0000;
        internals.attribute_bot_latch = 0b0000_0001;
        internals.attribute_top_latch = 0b0000_0001;
        
        internals
    }

    #[test]
    fn shift_registers_have_expected_value() {
        let mut internals = internal_fixture();

        // Assert one shift
        internals.shift_pattern_registers();
        internals.shift_attribute_registers();
        assert_eq!(internals.pattern_low_shift_reg,  0b0011_0110_1111_1110);
        assert_eq!(internals.pattern_high_shift_reg, 0b0111_0000_0100_0001);
        assert_eq!(internals.attribute_bot_shift_reg, 0b1110_0001);
        assert_eq!(internals.attribute_top_shift_reg, 0b0000_0001);

        // Assert the shift of the rest of the bytes
        for _ in 0..7 {
            internals.shift_pattern_registers();
        }

        // Only shift half the register
        for _ in 0..3 {
            internals.shift_attribute_registers();
        }

        assert_eq!(internals.pattern_low_shift_reg,  0b0111_1111_0000_0000);
        assert_eq!(internals.pattern_high_shift_reg, 0b0010_0000_1111_1111);
        assert_eq!(internals.attribute_bot_shift_reg, 0b0000_1111);
        assert_eq!(internals.attribute_top_shift_reg, 0b0000_1111);
    }

    #[test]
    fn pattern_registers_are_correctly_loaded_with_latch_value() {
        let mut internals = internal_fixture();

        // Shift out the registers and attributes
        for _ in 0..8 {
            internals.shift_pattern_registers();
        }

        assert_eq!(internals.pattern_low_shift_reg,  0b0111_1111_0000_0000);
        assert_eq!(internals.pattern_high_shift_reg, 0b0010_0000_1111_1111);

        // See if latch is filled properly
        internals.pattern_low_latch  = 0b1010_0101;
        internals.pattern_high_latch = 0b0101_1010;

        // See if pattern is loaded correctly
        internals.load_pattern_latch_into_register();
        assert_eq!(internals.pattern_low_shift_reg,  0b0111_1111_1010_0101);
        assert_eq!(internals.pattern_high_shift_reg, 0b0010_0000_0101_1010);
    }

    #[test]
    fn shift_registers_are_correctly_loaded_with_latch_value() {
        let mut internals = internal_fixture();

        // Shift out the registers and attributes
        for _ in 0..8 {
            internals.shift_attribute_registers();
        }

        assert_eq!(internals.attribute_bot_shift_reg, 0xFF);
        assert_eq!(internals.attribute_top_shift_reg, 0xFF);

        // See if latch is filled properly
        internals.attribute_next_tile_data_latch = 0b1110_0100;

        // Try every combination of attribute latch loading
        internals.load_attribute_bit_latch(0, 0).unwrap();
        assert_eq!(internals.attribute_bot_latch, 0);
        assert_eq!(internals.attribute_top_latch, 0);

        internals.load_attribute_bit_latch(0, 1).unwrap();
        assert_eq!(internals.attribute_bot_latch, 0);
        assert_eq!(internals.attribute_top_latch, 1);

        internals.load_attribute_bit_latch(1, 0).unwrap();
        assert_eq!(internals.attribute_bot_latch, 1);
        assert_eq!(internals.attribute_top_latch, 0);

        internals.load_attribute_bit_latch(1, 1).unwrap();
        assert_eq!(internals.attribute_bot_latch, 1);
        assert_eq!(internals.attribute_top_latch, 1);
    }
}