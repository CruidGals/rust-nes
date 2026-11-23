//! Functionality to load in palette of choice

use std::error::Error;

use eframe::egui::Color32;

pub struct Palette {
    palettes: [Option<Vec<Color32>>; 8],
}

/// Read ALL color palettes (if available)
pub fn load_palette_from_file(filepath: &str) -> Result<Palette, Box<dyn Error>> {
    // Read the file
    let palette_file = std::fs::read(filepath)?;

    // Grab the first one
    let normal_palette = load_palette(
        &palette_file, 
        false, 
        false, 
        false)
        .ok_or_else(|| format!("Palette file at {filepath} is invalid."))?;

    // Fill the palette struct
    Ok(Palette {
        palettes: [
            Some(normal_palette),
            load_palette(&palette_file, false, false, true),  // 001 (R)
            load_palette(&palette_file, false, true, false),  // 010 (G)
            load_palette(&palette_file, false, true, true),   // 011 (RG)
            load_palette(&palette_file, true, false, false),  // 100 (B)
            load_palette(&palette_file, true, false, true),   // 101 (RB)
            load_palette(&palette_file, true, true, false),   // 110 (GB)
            load_palette(&palette_file, true, true, true),    // 111 (RGB)
        ],
    })
}

/// Loads a palette given a .pal file. Based on the emphasis given from PPUMASK,
/// different parts of the .pal file are extracted to retrieve a palette:
/// 
/// (BGR, if the bit is on the color is emphasized)
/// 
/// PPUMASK Bits (BGR) | Byte Range 
/// -------------------|-----------
/// 000                | 0x000 - 0x0BF
/// 001                | 0x0C0 – 0x17F
/// 010                | 0x180 – 0x23F
/// 011                | 0x240 – 0x2FF
/// 100                | 0x300 – 0x3BF
/// 101                | 0x3C0 – 0x47F
/// 110                | 0x480 – 0x53F
/// 111                | 0x540 – 0x5FF
/// 
fn load_palette(palette_file: &[u8], blue_emphasis: bool, green_emphasis: bool, red_emphasis: bool) -> Option<Vec<Color32>> {
   let start_range: usize = match (blue_emphasis, green_emphasis, red_emphasis) {
        (false, false, false) => 0x000,
        (false, false, true) =>  0x0C0,
        (false, true, false) =>  0x180,
        (false, true, true) =>   0x240,
        (true, false, false) =>  0x300,
        (true, false, true) =>   0x3C0,
        (true, true, false) =>   0x480,
        (true, true, true) =>    0x540,
    };

    // Ensure the range is within the read range
    if palette_file.len() < start_range + 0xC0 {
        return None;
    }

    // Instantiate the palette
    let palette: Vec<Color32> = palette_file[start_range..start_range + 0xC0]
        .chunks_exact(3)
        .map(|color| Color32::from_rgb(color[0], color[1], color[2]))
        .collect();

    // Palette can ONLY be 64 colors
    assert_eq!(palette.len(), 64);

    Some(palette)
}