use crate::error::Error;
use crate::memory::Memory;
use crate::stack::Stack;

pub struct Cpu {
    memory: Memory,
    working_stack: Stack,
    return_stack: Stack,
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
            return_stack: Stack::empty(),
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

    pub fn clone_return_stack(&self) -> Stack {
        self.return_stack.clone()
    }

    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    pub fn tick(&mut self) -> Result<(), Error> {
        let instruction = self.memory.read_byte(self.program_counter);
        match instruction {
            0x00 => {
                return Err(Error::EndOfExecution);
            }
            0x80 => {
                let byte = self.memory.read_byte(self.program_counter + 1);
                self.working_stack.push_byte(byte)?;
                self.program_counter += 2;
            }
            0xa0 => {
                let short = self.memory.read_short(self.program_counter + 1);
                self.working_stack.push_short(short)?;
                self.program_counter += 3;
            }
            0xc0 => {
                let byte = self.memory.read_byte(self.program_counter + 1);
                self.return_stack.push_byte(byte)?;
                self.program_counter += 2;
            }
            0xe0 => {
                let short = self.memory.read_short(self.program_counter + 1);
                self.return_stack.push_short(short)?;
                self.program_counter += 3;
            }
            _ => {
                todo!("{:x}", instruction);
            }
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            let result = self.tick();
            match result {
                Ok(()) => continue,
                Err(Error::EndOfExecution) => return Ok(()),
                Err(err) => return Err(err),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! cpu_load {
        ( $x:expr) => {{
            let mut cpu = Cpu::new();
            let rom = $x;
            let result = cpu.load_rom(rom);
            assert!(result.is_ok());
            cpu
        }};
    }

    #[test]
    fn load_rom_too_big() {
        let mut cpu = Cpu::new();
        let rom = vec![0; 0xff00];
        let result = cpu.load_rom(rom);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::FailedToLoadRom);
    }

    #[test]
    fn lit() {
        let mut cpu = cpu_load!(vec![
            0x80, 0x12, // LIT 12
            0xa0, 0x34, 0x56, // LIT2 3456
            0xc0, 0x78, // LITr 78
            0xe0, 0x9a, 0xbc, // LIT2r 9abc
        ]);
        let result = cpu.run();
        assert!(result.is_ok(), "{:?}", result.unwrap_err());
        assert_eq!(cpu.clone_working_stack().as_vec(), vec![0x12, 0x34, 0x56]);
        assert_eq!(cpu.clone_return_stack().as_vec(), vec![0x78, 0x9a, 0xbc]);
    }
}
