use std::{cell::RefCell, error::Error, rc::Rc};

use crate::{cartridge::Cartridge, cpu::{CPU, cpubus::CPUBus}, ppu::{PPU, ppubus::PPUBus}, screen::run_gui};

mod cpu;
mod ppu;
mod cartridge;
mod util;
mod screen;

pub struct NES {
    cartridge: Rc<RefCell<Cartridge>>,
    cpu: CPU,
    ppu: Rc<RefCell<PPU>>,
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // TODO: Load the screen (not started yet)

    run_gui()?;

    // Try using test rom
    let rom = std::fs::read("roms/nestest.nes")?;
    let mut nes = power_up(&rom)?;
    
    // Setup the nestest
    nes.cpu.pc = 0xC000;
    while nes.cpu.instructions_ran < 8991 {
        nes.cpu.tick()?;
    }

    Ok(())
}

/// Power up function for the NES. Creates entirely new dependencies.
/// For now it just returns an NES struct containing the dependencies
/// required to use an NES
fn power_up(rom: &[u8]) -> Result<NES, String> {
    let cartridge = Rc::new(RefCell::new(Cartridge::new(rom)?));
    let ppu_bus = PPUBus::new(Rc::clone(&cartridge));
    let ppu = Rc::new(RefCell::new(PPU::new(ppu_bus)?));
    let cpu_bus = CPUBus::new(Rc::clone(&cartridge), Rc::clone(&ppu));
    let cpu = CPU::new(cpu_bus, true)?;

    Ok(NES { cartridge, cpu, ppu })
}

fn reset() {
    todo!()
}

fn tick() {
    todo!()
}
