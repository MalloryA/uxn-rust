#[cfg(test)]
mod tests {
    use uxn::cpu::Cpu;

    #[test]
    fn it_works() {
        let rom: Vec<u8> = vec![0x80, 0x13, 0x80, 0x12];
        let mut cpu = Cpu::new();

        // Load ROM
        let result = cpu.load_rom(rom);
        assert!(result.is_ok());

        // Check memory
        let memory = cpu.clone_memory();
        assert_eq!(memory.read_byte(0x100), 0x80);
        assert_eq!(memory.read_byte(0x101), 0x13);
        assert_eq!(memory.read_byte(0x102), 0x80);
        assert_eq!(memory.read_byte(0x103), 0x12);

        // Check working stack
        let working_stack = cpu.clone_working_stack();
        assert!(working_stack.as_vec().is_empty());

        // Check PC
        let program_counter = cpu.get_program_counter();
        assert_eq!(program_counter, 0x100);

        // Tick
        let result = cpu.tick();
        assert!(result.is_ok());

        let program_counter = cpu.get_program_counter();
        assert_eq!(program_counter, 0x102);
        let working_stack = cpu.clone_working_stack();
        assert_eq!(working_stack.as_vec(), vec![0x13]);

        // Tick
        let result = cpu.tick();
        assert!(result.is_ok());

        let program_counter = cpu.get_program_counter();
        assert_eq!(program_counter, 0x104);
        let working_stack = cpu.clone_working_stack();
        assert_eq!(working_stack.as_vec(), vec![0x13, 0x12]);
    }
}
