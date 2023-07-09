use crate::device_memory::DeviceMemory;
use std::convert::TryFrom;

pub struct Memory {
    memory: [u8; 0x10000],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; 0x10000],
        }
    }

    pub fn read_byte(&self, address: u32) -> u8 {
        self.memory[usize::try_from(address).unwrap()]
    }

    pub fn write_byte(&mut self, address: u32, value: u8) {
        self.memory[usize::try_from(address).unwrap()] = value;
    }

    pub fn read_short(&self, address: u32) -> u16 {
        let b1 = self.read_byte(address) as u16;
        let b2 = self.read_byte(address + 1) as u16;
        b1 << 8 | b2
    }

    pub fn write_short(&mut self, address: u32, value: u16) {
        let b1 = (value >> 8) as u8;
        let b2 = (value & 0xff) as u8;
        self.write_byte(address, b1);
        self.write_byte(address + 1, b2);
    }

    pub fn get_device_memory(&mut self, address: u8) -> DeviceMemory {
        let address = address as usize;
        let mut memory = [0; 0x10];
        let mut i = 0;
        for byte in &self.memory[address..address + 0x10] {
            memory[i] = *byte;
            i += 1;
        }
        DeviceMemory::new(memory)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_and_write_byte() {
        let mut memory = Memory::new();
        let byte = memory.read_byte(0x0100);
        assert_eq!(byte, 0x00);
        memory.write_byte(0x0100, 0xff);
        let byte = memory.read_byte(0x0100);
        assert_eq!(byte, 0xff);
    }

    #[test]
    fn read_and_write_short() {
        let mut memory = Memory::new();
        let short = memory.read_short(0x0100);
        assert_eq!(short, 0x0000);
        memory.write_short(0x0100, 0xffff);
        let short = memory.read_short(0x0100);
        assert_eq!(short, 0xffff);
    }
}
