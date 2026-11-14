use std::{cell::RefCell, rc::Rc};

use crate::{cartridge::Cartridge, cpu::{cpu::CPU, cpubus::CPUBus}};

mod cpu;
mod cartridge;

pub struct NES {
    cartridge: Rc<RefCell<Cartridge>>,
    cpu: CPU,
    // Add PPU, PPUBus, APU
}

fn main() {
    // TODO: Load the screen (not started yet)
}

/// Power up function for the NES. Creates entirely new dependencies.
/// For now it just returns an NES struct containing the dependencies
/// required to use an NES
fn power_up(rom: &[u8]) -> Result<NES, String> {
    let cartridge = Rc::new(RefCell::new(Cartridge::new(rom)?));
    let cpu_bus = CPUBus::new(Rc::clone(&cartridge));
    let cpu = CPU::new(cpu_bus);

    Ok(NES { cartridge, cpu })
}

fn reset() {
    todo!()
}

fn tick() {
    todo!()
}
