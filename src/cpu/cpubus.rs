/***
 * Bus speciically to connect CPU components
 */

pub struct CPUBus {
    // For now, just include reference to CPU VRAM
    vram: [u8; 2048]
}

impl CPUBus {
    pub fn new() -> Self {
        Self { vram: [0; 2048] }
    }
}