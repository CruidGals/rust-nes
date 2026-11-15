use std::fs::{File, OpenOptions};
use std::io::Write;

#[derive(Debug)]
struct CPUState {
    opcodes: Vec<u8>,
    pc: u16,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    sp: u8,
    cycle_count: i32,
}

impl CPUState {
    pub fn new() -> Self {
        CPUState { opcodes: vec![], pc: 0, a: 0, x: 0, y: 0, p: 0, sp: 0, cycle_count: 0 }
    }
}

#[derive(Debug)]
pub struct Logger {
    cpu_state: CPUState,
    log_file: File
}


impl Logger {
    pub fn new() -> Result<Self, String> {
        let log_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("logs/debug.log")
                .map_err(|err| err.to_string())?;
        
        Ok(Self { 
            cpu_state: CPUState::new(), 
            log_file 
        })
    }

    pub fn log(&mut self) -> Result<(), String> {
        let byte_string = self.cpu_state.opcodes.iter()
                .map(|byte| format!("{:02X}", byte))
                .collect::<Vec<String>>()
                .join(" ");
        let message = format!("{:04X}\t{}\t\tA:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{}", 
                self.cpu_state.pc, 
                byte_string,
                self.cpu_state.a,
                self.cpu_state.x,
                self.cpu_state.y,
                self.cpu_state.p,
                self.cpu_state.sp,
                self.cpu_state.cycle_count);

        writeln!(self.log_file, "{message}").map_err(|err| err.to_string())?;
        self.log_file.flush().map_err(|err| err.to_string())?;
        Ok(())
    }

    pub fn reset_cpu_state(&mut self, pc: u16, a: u8, x: u8, y: u8, p: u8, sp: u8, cycle_count: i32) {
        self.cpu_state.opcodes = vec![];
        self.cpu_state.pc = pc;
        self.cpu_state.a = a;
        self.cpu_state.x = x;
        self.cpu_state.y = y;
        self.cpu_state.p = p;
        self.cpu_state.sp = sp;
        self.cpu_state.cycle_count = cycle_count;
    }

    pub fn add_opcode(&mut self, value: u8) {
        self.cpu_state.opcodes.push(value);
    }
}