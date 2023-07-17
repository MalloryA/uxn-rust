use crate::error::Error;
use crate::memory::Memory;
use crate::stack::Stack;

pub struct Cpu {
    memory: Memory,
    working_stack: Stack,
    program_counter: u16,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: Memory::new(),
            working_stack: Stack::empty(),
            program_counter: 0x100,
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

    pub fn clone_memory(&self) -> Memory {
        self.memory.clone()
    }

    pub fn clone_working_stack(&self) -> Stack {
        self.working_stack.clone()
    }

    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn tick(&mut self) -> Result<(), Error> {
        let instruction = self.memory.read_byte(self.program_counter);
        match instruction {
            0x80 => {
                let byte = self.memory.read_byte(self.program_counter + 1);
                self.working_stack.push_byte(byte)?;
                self.program_counter += 2;
            }
            _ => {
                todo!();
            }
        }
        Ok(())
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
