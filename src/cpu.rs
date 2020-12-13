use crate::bus::*;

pub const BOOT_EXCEPTION_VECTOR: u32 = 0xbfc0_0000;
pub const KUSEG_BASE: u32 = 0x0000_0000;
pub const KUSEG_SIZE: u32 = 0x8000_0000;
pub const KSEG0_BASE: u32 = 0x8000_0000;
pub const KSEG0_SIZE: u32 = 0x2000_0000;
pub const KSEG1_BASE: u32 = 0xa000_0000;
pub const KSEG1_SIZE: u32 = 0x2000_0000;
pub const KSEG2_BASE: u32 = 0xc000_0000;

pub struct Cpu {
    pub regs: [u32; 32],
    pub pc: u32,
    pub bus: Bus,
    pc_branch_delay: Option<u32>,
    pub hi: u32,
    pub lo: u32,
}

impl Cpu {
    pub fn new(binary: Vec<u8>) -> Self {
        let regs = [0; 32];

        Self {
            regs: regs,
            pc: BOOT_EXCEPTION_VECTOR,
            bus: Bus::new(binary),
            pc_branch_delay: None,
            hi: 0u32,
            lo: 0u32,
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

    pub fn load(&mut self, addr: u32, size: u32) -> Result<u32, ()> {
        let physical_addr = self.mmu(addr);
        self.bus.load(physical_addr, size)
    }

    pub fn store(&mut self, addr: u32, size: u32, value: u32) -> Result<(), ()> {
        let physical_addr = self.mmu(addr);
        self.bus.store(physical_addr, size, value)
    }

    pub fn mmu(&mut self, addr: u32) -> u32 {
        if KUSEG_BASE <= addr && addr < KUSEG_BASE + KUSEG_SIZE {
            dbg!(format!("not implemented yet: page table"));
        } else if KSEG0_BASE <= addr && addr < KSEG0_BASE + KSEG0_SIZE {
            return addr - KSEG0_BASE;
        } else if KSEG1_BASE <= addr && addr < KSEG1_BASE + KSEG1_SIZE {
            return addr - KSEG1_BASE;
        } else if KSEG2_BASE <= addr {
            dbg!(format!("not implemented yet: page table"));
        }
        return 0;
    }

    pub fn fetch(&mut self) -> Result<u32, ()> {
        match self.load(self.pc, 32) {
            Ok(inst) => Ok(inst),
            Err(_e) => Err(()),
        }
    }

    pub fn execute(&mut self, inst: u32) -> Result<(), ()> {
        let opcode = (inst & 0xfc000000) >> 26;
        let rs = ((inst & 0x03e00000) >> 21) as usize;
        let rt = ((inst & 0x001f0000) >> 16) as usize;
        let rd = ((inst & 0x0000f800) >> 11) as usize;
        let mut is_branch = false;

        self.regs[0] = 0;

        match opcode {
            0x00 => {
                let funct = inst & 0x0000003f;
                match funct {
                    0x00 => {
                        // noop
                        // sll
                        let shamt = (inst & 0x000007c0) >> 6;
                        self.regs[rd] = self.regs[rt].wrapping_shl(shamt);
                    }
                    0x02 => {
                        //srl
                        let shamt = (inst & 0x000007c0) >> 6;
                        self.regs[rd] = self.regs[rt].wrapping_shr(shamt);
                    }
                    0x03 => {
                        // sra
                        let shamt = (inst & 0x000007c0) >> 6;
                        self.regs[rd] = (self.regs[rt] as i32).wrapping_shr(shamt) as u32;
                    }
                    0x04 => {
                        // sllv
                        self.regs[rd] = self.regs[rt].wrapping_shl(self.regs[rs]);
                    }
                    0x06 => {
                        // srlv
                        self.regs[rd] = self.regs[rt].wrapping_shr(self.regs[rs]);
                    }
                    0x07 => {
                        // srav
                        self.regs[rd] = (self.regs[rt] as i32).wrapping_shr(self.regs[rs]) as u32;
                    }
                    0x08 => {
                        // jr
                        is_branch = true;
                        self.pc_branch_delay = Some(self.regs[rs]);
                    }
                    0x09 => {
                        // jalr
                        is_branch = true;
                        self.regs[rd] = self.pc.wrapping_add(4);
                        self.pc_branch_delay = Some(self.regs[rs]);
                    }
                    0x0d => {
                        // break
                        // TODO: excpetion handling
                    }
                    0x10 => {
                        // mfhi
                        self.regs[rd] = self.hi;
                    }
                    0x11 => {
                        // mthi
                        self.hi = self.regs[rs];
                    }
                    0x12 => {
                        // mflo
                        self.regs[rd] = self.lo;
                    }
                    0x13 => {
                        // mtlo
                        self.lo = self.regs[rs];
                    }
                    0x18 => {
                        // mult
                        let product = ((self.regs[rs] as i32) as i64).wrapping_mul((self.regs[rt] as i32) as i64) as u64;
                        self.hi = (product >> 32) as u32;
                        self.lo = product as u32;
                    }
                    0x19 => {
                        // multu
                        let product = (self.regs[rs] as u64).wrapping_mul(self.regs[rt] as u64);
                        self.hi = (product >> 32) as u32;
                        self.lo = product as u32;
                    }
                    0x1a => {
                        // div
                        // If the divisor in GPRrt is zero, the arithmetic result value is undefined.
                        if self.regs[rt] != 0 {
                            self.lo = (self.regs[rs] as i32).wrapping_div(self.regs[rt] as i32) as u32;
                            self.hi = (self.regs[rs] as i32).wrapping_rem(self.regs[rt] as i32) as u32;
                        }
                    }
                    0x1b => {
                        // divu
                        // If the divisor in GPRrt is zero, the arithmetic result value is undefined.
                        if self.regs[rt] != 0 {
                            self.lo = self.regs[rs].wrapping_div(self.regs[rt]);
                            self.hi = self.regs[rs].wrapping_rem(self.regs[rt]);
                        }
                    }
                    0x21 => {
                        // addu
                        self.regs[rd] = self.regs[rs].wrapping_add(self.regs[rt]);
                    }
                    0x23 => {
                        // subu
                        self.regs[rd] = self.regs[rs].wrapping_sub(self.regs[rt]);
                    }
                    0x24 => {
                        // and
                        self.regs[rd] = self.regs[rs] & self.regs[rt];
                    }
                    0x25 => {
                        // or
                        self.regs[rd] = self.regs[rs] | self.regs[rt];
                    }
                    0x26 => {
                        // xor
                        self.regs[rd] = self.regs[rs] ^ self.regs[rt];
                    }
                    0x27 => {
                        // nor
                        self.regs[rd] = !(self.regs[rs] | self.regs[rt]);
                    }
                    0x2a => {
                        // slt
                        if (self.regs[rs] as i32) < (self.regs[rt] as i32) {
                            self.regs[rd] = 1u32;
                        } else {
                            self.regs[rd] = 0u32;
                        }
                    }
                    0x2b => {
                        // sltu
                        if self.regs[rs] < self.regs[rt] {
                            self.regs[rd] = 1u32;
                        } else {
                            self.regs[rd] = 0u32;
                        }
                    }
                    _ => {
                        dbg!(format!("not implemented yet: opcode {:#x} funct {:#x}", opcode, funct));
                        return Err(());
                    }
                }
            }
            0x01 => {
                let funct = rt;
                match funct {
                    0x00 => {
                        // bltz
                        is_branch = true;
                        let offset = ((inst & 0x0000ffff) as i16) as u32;
                        if (self.regs[rs] as i32) < 0 {
                            self.pc_branch_delay = Some(self.pc.wrapping_add(offset << 2));
                        }
                    }
                    0x01 => {
                        // bgez
                        is_branch = true;
                        let offset = ((inst & 0x0000ffff) as i16) as u32;
                        if (self.regs[rs] as i32) >= 0 {
                            self.pc_branch_delay = Some(self.pc.wrapping_add(offset << 2));
                        }
                    }
                    0x10 => {
                        // bltzal
                        is_branch = true;
                        let offset = ((inst & 0x0000ffff) as i16) as u32;
                        if (self.regs[rs] as i32) < 0 {
                            self.regs[31] = self.pc.wrapping_add(4);
                            self.pc_branch_delay = Some(self.pc.wrapping_add(offset << 2));
                        }
                    }
                    0x11 => {
                        // bgezal
                        is_branch = true;
                        let offset = ((inst & 0x0000ffff) as i16) as u32;
                        if (self.regs[rs] as i32) >= 0 {
                            self.regs[31] = self.pc.wrapping_add(4);
                            self.pc_branch_delay = Some(self.pc.wrapping_add(offset << 2));
                        }
                    }
                    _ => {
                        dbg!(format!("not implemented yet: opcode {:#x} funct {:#x}", opcode, funct));
                        return Err(());
                    }
                }
            }
            0x02 => {
                // j
                is_branch = true;
                let target = inst & 0x03ffffff;
                self.pc_branch_delay = Some((self.pc & 0xf0000000) | (target << 2));
            }
            0x03 => {
                // jal
                is_branch = true;
                self.regs[31] = self.pc.wrapping_add(4);
                let target = inst & 0x03ffffff;
                self.pc_branch_delay = Some((self.pc & 0xf0000000) | (target << 2));
            }
            0x04 => {
                // beq
                is_branch = true;
                let offset = ((inst & 0x0000ffff) as i16) as u32;
                if self.regs[rs] == self.regs[rt] {
                    self.pc_branch_delay = Some(self.pc.wrapping_add(offset << 2));
                }
            }
            0x05 => {
                // bne
                is_branch = true;
                let offset = ((inst & 0x0000ffff) as i16) as u32;
                if self.regs[rs] != self.regs[rt] {
                    self.pc_branch_delay = Some(self.pc.wrapping_add(offset << 2));
                }
            }
            0x06 => {
                // blez
                is_branch = true;
                let offset = ((inst & 0x0000ffff) as i16) as u32;
                if (self.regs[rs] as i32) <= 0 {
                    self.pc_branch_delay = Some(self.pc.wrapping_add(offset << 2));
                }
            }
            0x07 => {
                // bgtz
                is_branch = true;
                let offset = ((inst & 0x0000ffff) as i16) as u32;
                if (self.regs[rs] as i32) > 0 {
                    self.pc_branch_delay = Some(self.pc.wrapping_add(offset << 2));
                }
            }
            0x09 => {
                // addiu
                let imm = ((inst & 0x0000ffff) as i16) as u32;
                self.regs[rt] = self.regs[rs].wrapping_add(imm);
            }
            0x0a => {
                // slti
                let imm = ((inst & 0x0000ffff) as i16) as i32;
                if (self.regs[rs] as i32) < imm {
                    self.regs[rt] = 1u32;
                } else {
                    self.regs[rt] = 0u32;
                }
            }
            0x0b => {
                // sltiu
                let imm = inst & 0x0000ffff;
                if self.regs[rs] < imm {
                    self.regs[rt] = 1u32;
                } else {
                    self.regs[rt] = 0u32;
                }
            }
            0x0c => {
                // andi
                let imm = inst & 0x0000ffff;
                self.regs[rt] = self.regs[rs] & imm;
            }
            0x0d => {
                // ori
                let imm = inst & 0x0000ffff;
                self.regs[rt] = self.regs[rs] | imm;
            }
            0x0e => {
                // xori
                let imm = inst & 0x0000ffff;
                self.regs[rt] = self.regs[rs] ^ imm;
            }
            0x0f => {
                // lui
                let imm = inst & 0x0000ffff;
                self.regs[rt] = imm << 16;
            }
            0x1c => {
                let funct = inst & 0x0000003f;
                match funct {
                    0x0 => {
                        // madd
                        let product = ((self.regs[rs] as i32) as i64).wrapping_mul((self.regs[rt] as i32) as i64) as u64;
                        let acc = product.wrapping_add((self.hi as u64).wrapping_shl(32) + self.lo as u64);
                        self.hi = (acc >> 32) as u32;
                        self.lo = acc as u32;
                    }
                    0x1 => {
                        // maddu
                        let product = (self.regs[rs] as u64).wrapping_mul(self.regs[rt] as u64);
                        let acc = product.wrapping_add((self.hi as u64).wrapping_shl(32) + self.lo as u64);
                        self.hi = (acc >> 32) as u32;
                        self.lo = acc as u32;
                    }
                    0x2 => {
                        // mul
                        self.regs[rd] = ((self.regs[rs] as i64).wrapping_mul(self.regs[rt] as i64)) as u32;
                    }
                    0x20 => {
                        // clz
                        self.regs[rt] = self.regs[rs].leading_zeros();
                    }
                    0x21 => {
                        // clo
                        self.regs[rt] = self.regs[rs].leading_ones();
                    }
                    _ => {
                        dbg!(format!("not implemented yet: opcode {:#x} funct {:#x}", opcode, funct));
                        return Err(());
                    }
                }

            }
            0x20 => {
                // lb
                let imm = ((inst & 0x0000ffff) as i16) as u32;
                self.regs[rt] = self.load(self.regs[rs].wrapping_add(imm), 8)?
            }
            0x23 => {
                // lw
                let imm = ((inst & 0x0000ffff) as i16) as u32;
                self.regs[rt] = self.load(self.regs[rs].wrapping_add(imm), 32)?
            }
            0x28 => {
                // sb
                let imm = ((inst & 0x0000ffff) as i16) as u32;
                self.store(self.regs[rs].wrapping_add(imm), 8, self.regs[rt])?
            }
            0x2b => {
                // sw
                let imm = ((inst & 0x0000ffff) as i16) as u32;
                self.store(self.regs[rs].wrapping_add(imm), 32, self.regs[rt])?
            }
            _ => {
                dbg!(format!("not implemented yet: opcode {:#x}", opcode));
                return Err(());
            }
        }

        // assume there's not branch instruction in branch delay slot
        if !is_branch {
            match self.pc_branch_delay {
                Some(pc) => {
                    // current instruction is in branch delay slot
                    self.pc = pc;
                    self.pc_branch_delay = None;
                }
                None => {}
            }
        }

        println!("{}", format!(
            "nextpc={:#x}, opcode={:#x}, rs={}, rt={}, rd={}",
            self.pc, opcode, rs, rt, rd
        ));

        Ok(())
    }
}
