use crate::bus::*;

pub const MEMORY_SIZE: u32 = 1024 * 1024 * 128;
pub const BOOT_ROM_SIZE: u32 = 1024 * 1024 * 4;

pub trait Memory {
    fn load8(&self, addr: u32) -> u32;
    fn load16(&self, addr: u32) -> u32;
    fn load32(&self, addr: u32) -> u32;
    fn store8(&mut self, addr: u32, value: u32);
    fn store16(&mut self, addr: u32, value: u32);
    fn store32(&mut self, addr: u32, value: u32);
}

impl<T: Memory> Device for T {
    fn load(&self, addr: u32, size: u32) -> Result<u32, ()> {
        match size {
            8 => Ok(self.load8(addr)),
            16 => Ok(self.load16(addr)),
            32 => Ok(self.load32(addr)),
            _ => Err(()),
        }
    }

    fn store(&mut self, addr: u32, size: u32, value: u32) -> Result<(), ()> {
        match size {
            8 => Ok(self.store8(addr, value)),
            16 => Ok(self.store16(addr, value)),
            32 => Ok(self.store32(addr, value)),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Dram {
    pub memory: Vec<u8>,
}

impl Memory for Dram {
    fn load8(&self, addr: u32) -> u32 {
        let index = addr as usize;
        self.memory[index] as u32
    }

    fn load16(&self, addr: u32) -> u32 {
        let index = addr as usize;
        return (self.memory[index] as u32) | ((self.memory[index + 1] as u32) << 8);
    }

    fn load32(&self, addr: u32) -> u32 {
        let index = addr as usize;
        return (self.memory[index] as u32)
            | ((self.memory[index + 1] as u32) << 8)
            | ((self.memory[index + 2] as u32) << 16)
            | ((self.memory[index + 3] as u32) << 24);
    }

    fn store8(&mut self, addr: u32, value: u32) {
        let index = addr as usize;
        self.memory[index] = value as u8;
    }

    fn store16(&mut self, addr: u32, value: u32) {
        let index = addr as usize;
        self.memory[index] = (value & 0xff) as u8;
        self.memory[index + 1] = ((value >> 8) & 0xff) as u8;
    }

    fn store32(&mut self, addr: u32, value: u32) {
        let index = addr as usize;
        self.memory[index] = (value & 0xff) as u8;
        self.memory[index + 1] = ((value >> 8) & 0xff) as u8;
        self.memory[index + 2] = ((value >> 16) & 0xff) as u8;
        self.memory[index + 3] = ((value >> 24) & 0xff) as u8;
    }
}

impl Dram {
    pub fn new(binary: Vec<u8>, memory_size: u32) -> Dram {
        let mut memory = vec![0; memory_size as usize];
        memory.splice(..binary.len(), binary.iter().cloned());

        Self {memory}
    }
}

#[derive(Debug)]
pub struct Rom {
    pub memory: Vec<u8>,
}

impl Memory for Rom {
    fn load8(&self, addr: u32) -> u32 {
        let index = addr as usize;
        self.memory[index] as u32
    }

    fn load16(&self, addr: u32) -> u32 {
        let index = addr as usize;
        return (self.memory[index] as u32) | ((self.memory[index + 1] as u32) << 8);
    }

    fn load32(&self, addr: u32) -> u32 {
        let index = addr as usize;
        return (self.memory[index] as u32)
            | ((self.memory[index + 1] as u32) << 8)
            | ((self.memory[index + 2] as u32) << 16)
            | ((self.memory[index + 3] as u32) << 24);
    }

    fn store8(&mut self, _addr: u32, _value: u32) {
    }

    fn store16(&mut self, _addr: u32, _value: u32) {
    }

    fn store32(&mut self, _addr: u32, _value: u32) {
    }
}

impl Rom {
    pub fn new(binary: Vec<u8>, memory_size: u32) -> Rom {
        let mut memory = vec![0; memory_size as usize];
        memory.splice(..binary.len(), binary.iter().cloned());

        Self {memory}
    }
}
