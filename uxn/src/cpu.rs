use crate::error::Error;
use crate::memory::Memory;

pub struct Cpu {
    memory: Memory,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: Memory::new(),
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) -> Result<(), Error> {
        let mut i: u16 = 0x100;
        for byte in rom {
            if i == 0xffff {
                return Err(Error::FailedToLoadRom);
            }
            self.memory.write_byte(i, byte);
            i += 1;
        }
        Ok(())
    }

    pub fn get_memory_clone(&self) -> Memory {
        self.memory.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_rom_too_big() {
        let mut cpu = Cpu::new();
        let rom = vec![0; 0xff00];
        let result = cpu.load_rom(rom);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::FailedToLoadRom);
    }
}
