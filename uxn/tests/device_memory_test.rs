#[cfg(test)]
mod tests {
    use uxn::memory::Memory;

    //macro_rules! try_stack {
    //    ( $x:expr, $y:expr ) => {{
    //        let commands: Vec<&str> = $x.split(" ").collect();
    //        let expected: Vec<u8> = $y.split(" ").map(|s| s.bytes().next().unwrap()).collect();
    //        let mut stack = Stack::new(vec![b'a', b'b', b'c']);
    //        for command in commands {
    //            call(&mut stack, command);
    //        }
    //        assert_eq!(expected, stack.as_vec());
    //    }};
    //}

    #[test]
    fn it_works() {
        let mut memory = Memory::new();
        memory.write_short(0x0050, 0x0123);
        memory.write_short(0x0052, 0x4567);
        memory.write_short(0x0054, 0x89ab);
        memory.write_short(0x0056, 0xcdef);
        memory.write_short(0x0058, 0x0123);
        memory.write_short(0x005a, 0x4567);
        memory.write_short(0x005c, 0x89ab);
        memory.write_short(0x005e, 0xcdef);
        let device_memory = memory.get_device_memory(0x50);
        assert_eq!(device_memory.read_short(0x00), 0x0123);
        assert_eq!(device_memory.read_short(0x02), 0x4567);
        assert_eq!(device_memory.read_short(0x04), 0x89ab);
        assert_eq!(device_memory.read_short(0x06), 0xcdef);
        assert_eq!(device_memory.read_short(0x08), 0x0123);
        assert_eq!(device_memory.read_short(0x0a), 0x4567);
        assert_eq!(device_memory.read_short(0x0c), 0x89ab);
        assert_eq!(device_memory.read_short(0x0e), 0xcdef);
    }
}
