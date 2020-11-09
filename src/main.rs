mod bus;
mod cpu;
mod memory;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::cpu::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: simp <filename>");
    }
    let mut file = File::open(&args[1])?;
    let mut binary = Vec::new();
    file.read_to_end(&mut binary)?;

    let mut cpu = Cpu::new(binary);

    loop {
        let inst = match cpu.fetch() {
            Ok(inst) => inst,
            Err(_) => break,
        };

        cpu.pc += 4;

        match cpu.execute(inst) {
            Ok(_) => {},
            Err(_) => break,
        }

        if cpu.pc == 0 {
            break;
        }
    }
    cpu.dump_registers();

    Ok(())
}
