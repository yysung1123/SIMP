use crate::memory::*;

pub const PHY_BOOT_ROM_BASE: u32 = 0x1fc0_0000;

pub trait Device {
    fn load(&self, addr: u32, size: u32) -> Result<u32, ()>;
    fn store(&mut self, addr: u32, size: u32, value: u32) -> Result<(), ()>;
}

pub struct Bus {
    memory: Dram,
    boot_rom: Rom,
}

impl Bus {
    pub fn new(binary: Vec<u8>) -> Bus {
        Self {
            memory: Dram::new(vec![], MEMORY_SIZE),
            boot_rom: Rom::new(binary, BOOT_ROM_SIZE),
        }
    }

    pub fn load(&self, addr: u32, size: u32) -> Result<u32, ()> {
        if PHY_BOOT_ROM_BASE <= addr && addr < PHY_BOOT_ROM_BASE + BOOT_ROM_SIZE {
            return self.boot_rom.load(addr - PHY_BOOT_ROM_BASE, size);
        } else {
            return self.memory.load(addr, size);
        }
    }

    pub fn store(&mut self, addr: u32, size: u32, value: u32) -> Result<(), ()> {
        if PHY_BOOT_ROM_BASE <= addr && addr < PHY_BOOT_ROM_BASE + BOOT_ROM_SIZE {
            return self.boot_rom.store(addr - PHY_BOOT_ROM_BASE, size, value);
        } else {
            return self.memory.store(addr, size, value);
        }
    }
}
