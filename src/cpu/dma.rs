/***
 * Simulation of DMA circuitry
 */

use crate::cpu::cpubus::CPUBus;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DMAProcess {
    OAMDMA(u8), // Holds the starting high byte address and fetched value
    DMCDMA,
}

#[derive(Debug)]
pub struct DMA {
    pub enabled: bool,
    curr_process: Option<DMAProcess>,
    cycle_count: i32,

    // Register
    pub held_value: u8
}

impl DMA {
    pub fn new() -> Self {
        Self { enabled: false, curr_process: None, cycle_count: 0, held_value: 0 }
    }

    /// Step whatever DMA process it has scheduled
    pub fn step(&mut self, cpu_bus: &mut CPUBus) -> Result<(), String> {
        // Cannot run this function if not enabled
        if !self.enabled {
            return Err(format!("Called DMA step when the DMA has not been scheduled."));
        }

        let Some(dma_process) = self.curr_process else {
            return Err(format!("Called DMA step when the DMA does not have a scheduled process."));
        };

        match dma_process {
            DMAProcess::OAMDMA(high_byte) => self.step_oamdma(cpu_bus, high_byte),
            DMAProcess::DMCDMA => Err(format!("DMCDMA not implemented."))
        }
    }
}

impl DMA {
    /// OAMDMA; purposely lightweight to handle edge case of multiple writes.
    pub fn begin_oamdma(&mut self, cpu_cycle_count: i32, starting_high_byte: u8) -> () {
        // If odd cycle, we do 2 synchronization steps
        // If even cycle, we do 1 synchronization step
        // Thus, we can start actual DMA operations at cycle 0, where
        // even cycles start at cycle -1 and odd cycles start at cycle -2
        self.cycle_count = if cpu_cycle_count % 2 == 0 { -1 } else { -2 };
        self.enabled = true;
        self.curr_process = Some(DMAProcess::OAMDMA(starting_high_byte));
    }

    pub fn step_oamdma(&mut self, cpu_bus: &mut CPUBus, high_byte: u8) -> Result<(), String> {

        // Synchronization steps (wait until cycle count reaches 0)
        if self.cycle_count < 0 {
            self.cycle_count += 1;
            return Ok(());
        }

        // Perform GETS on even cycle counts, and PUTS on odd cycle counts
        if self.cycle_count % 2 == 0 {
            // self.cycle_count is guaranteed to be between 0 and 511
            let low_byte = (self.cycle_count / 2) as u16;
            let address = u16::from(high_byte) << 8 | low_byte;
            self.held_value = cpu_bus.read(address)?;
        } else {
            cpu_bus.write(0x2004, self.held_value)?;
        }

        self.cycle_count += 1;

        // Reached the end of OAMDMA. We do this here to ensure the DMA
        // circuit releases the CPU right after the last PUT operation
        if self.cycle_count == 512 {
            self.enabled = false;
            self.curr_process = None;
            self.cycle_count = 0;
        }

        Ok(())
    }
}