#[cfg(test)]
mod tests {
    use uxn::cpu::Cpu;

    #[test]
    fn it_works() {
        let rom: Vec<u8> = vec![0x80, 0x13, 0x80, 0x12];
        let mut cpu = Cpu::new();
        let result = cpu.load_rom(rom);
        assert!(result.is_ok());
        let memory = cpu.get_memory_clone();
        assert_eq!(memory.read_byte(0x100), 0x80);
        assert_eq!(memory.read_byte(0x101), 0x13);
        assert_eq!(memory.read_byte(0x102), 0x80);
        assert_eq!(memory.read_byte(0x103), 0x12);
    }
}
