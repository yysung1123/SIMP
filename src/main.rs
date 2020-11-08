use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub const MEMORY_SIZE: u64 = 1024 * 1024 * 128;

struct Cpu {
    regs: [u32; 32],
    pc: u64,
    memory: Vec<u8>,
}

impl Cpu {
    fn new(binary: Vec<u8>) -> Self {
        let regs = [0; 32];

        Self {
            regs: regs,
            pc: 0,
            memory: binary,
        }
    }

    pub fn dump_registers(&self) {
        let mut output = String::from("");
        let abi = [
            "zero", "at", "v0", "v1", "a0", "a1", "a2", "a3", "t0", "t1",
            "t2", "t3", "t4", "t5", "t6", "t7", "s0", "s1", "s2", "s3",
            "s4", "s5", "s6", "s7", "t8", "t9", "k0", "k1", "gp", "sp",
            "fp", "ra",
        ];
        for i in (0..32).step_by(4) {
            output = format!(
                "{}\n{}",
                output,
                format!(
                    "x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x}",
                    i,
                    abi[i],
                    self.regs[i],
                    i + 1,
                    abi[i + 1],
                    self.regs[i + 1],
                    i + 2,
                    abi[i + 2],
                    self.regs[i + 2],
                    i + 3,
                    abi[i + 3],
                    self.regs[i + 3],
                )
            );
        }
        println!("{}", output);
    }

    fn fetch(&self) -> u32 {
        let index = self.pc as usize;
        return (self.memory[index] as u32)
            | ((self.memory[index + 1] as u32) << 8)
            | ((self.memory[index + 2] as u32) << 16)
            | ((self.memory[index + 3] as u32) << 24);
    }

    fn execute(&mut self, inst: u32) {
        let opcode = (inst & 0xfc000000) >> 26;
        let rs = ((inst & 0x03e00000) >> 21) as usize;
        let rt = ((inst & 0x001f0000) >> 16) as usize;
        let rd = ((inst & 0x0000f800) >> 11) as usize;

        self.regs[0] = 0;

        match opcode {
            0x00 => {
                // addu
                let funct = inst & 0x0000003f;
                match funct {
                    0x21 => {
                        self.regs[rd] = self.regs[rs].wrapping_add(self.regs[rt]);
                    }
                    _ => {
                        dbg!(format!("not implemented yet: opcode {:#x} funct {:#x}", opcode, funct));
                    }
                }
            }
            0x09 => {
                // addiu
                let imm = ((inst & 0x0000ffff) as i16) as u32;
                self.regs[rt] = self.regs[rs].wrapping_add(imm);
            }
            _ => {
                dbg!(format!("not implemented yet: opcode {:#x}", opcode));
            }
        }

        println!("{}", format!(
            "opcode={:#x}, rs={}, rt={}, rd={}",
            opcode, rs, rt, rd
        ));
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: simp <filename>");
    }
    let mut file = File::open(&args[1])?;
    let mut binary = Vec::new();
    file.read_to_end(&mut binary)?;

    let mut cpu = Cpu::new(binary);

    while cpu.pc < cpu.memory.len() as u64 {
        let inst = cpu.fetch();

        cpu.pc += 4;

        cpu.execute(inst);
    }
    cpu.dump_registers();

    Ok(())
}
