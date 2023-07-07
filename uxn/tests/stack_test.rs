#[cfg(test)]
mod tests {
    use uxn::stack::Stack;

    fn call(stack: &mut Stack, command: &str) {
        let result = match command {
            "POP" => stack.pop(),
            "DUP" => stack.dup(),
            "DUPk" => stack.dupk(),
            "OVR" => stack.ovr(),
            "SWP" => stack.swp(),
            "NIP" => stack.nip(),
            _ => todo!("{}", command),
        };
        result.unwrap();
    }

    macro_rules! try_stack {
        ( $x:expr, $y:expr ) => {{
            let commands: Vec<&str> = $x.split(" ").collect();
            let expected: Vec<u8> = $y.split(" ").map(|s| s.bytes().next().unwrap()).collect();
            let mut stack = Stack::new(vec![b'a', b'b', b'c']);
            for command in commands {
                call(&mut stack, command);
            }
            assert_eq!(expected, stack.as_vec());
        }};
    }

    #[test]
    fn it_works() {
        try_stack!("POP POP", "a");
        try_stack!("POP POP DUP", "a a");
        try_stack!("POP POP DUPk", "a a a");
        try_stack!("POP OVR SWP", "a a b");
        try_stack!("NIP OVR SWP", "a a c");
        try_stack!("POP", "a b");
        try_stack!("POP OVR", "a b a");
        try_stack!("POP OVR DUP", "a b a a");
        try_stack!("POP OVR DUPk", "a b a a a");
        try_stack!("POP OVR SWPk", "a b a a b");
        try_stack!("POP OVR OVR", "a b a b");
        try_stack!("POP OVRk", "a b a b a");
        try_stack!("POP OVR ROTk", "a b a b a a");
        try_stack!("POP OVRk DUPk", "a b a b a a a");
        try_stack!("POP OVRk SWPk", "a b a b a a b");
        try_stack!("POP OVR OVRk", "a b a b a b");
        try_stack!("POP OVRk ROTk", "a b a b a b a a");
        try_stack!("POP OVRk OVRk", "a b a b a b a b");
        try_stack!("POP DUP", "a b b");
        try_stack!("POP SWPk", "a b b a");
        try_stack!("POP SWPk DUP", "a b b a a");
        try_stack!("POP SWPk DUPk", "a b b a a a");
        try_stack!("POP SWPk SWPk", "a b b a a b");
        try_stack!("POP SWPk OVR", "a b b a b");
        try_stack!("POP SWPk OVRk", "a b b a b a b");
        try_stack!("ROTk NIP ROT", "a b b a c");
        try_stack!("POP DUPk", "a b b b");
        try_stack!("POP DUP DUPk", "a b b b b");
        try_stack!("POP DUP ROTk", "a b b b b a");
        try_stack!("POP DUP OVRk", "a b b b b b");
        try_stack!("POP DUPk OVRk", "a b b b b b b");
        try_stack!("OVR DUP ROT", "a b b b c");
        try_stack!("OVR SWP", "a b b c");
        try_stack!("OVR SWP OVR", "a b b c b");
        try_stack!("OVR SWP OVRk", "a b b c b c b");
        try_stack!("OVR SWP DUP", "a b b c c");
        try_stack!("OVR SWP SWPk", "a b b c c b");
        try_stack!("OVR SWP DUPk", "a b b c c c");
        try_stack!("POPk", "a b c");
        try_stack!("ROTk NIP NIP", "a b c a");
        try_stack!("ROTk NIP SWP", "a b c a b");
        try_stack!("ROTk ROT ROT", "a b c a b c");
        try_stack!("ROTk SWP ROT", "a b c a c b");
        try_stack!("OVR", "a b c b");
        try_stack!("ROTk NIP", "a b c b a");
        try_stack!("ROTk NIP DUP", "a b c b a a");
        try_stack!("ROTk NIP DUPk", "a b c b a a a");
        try_stack!("ROTk NIP SWPk", "a b c b a a b");
        try_stack!("ROTk DUP ROT", "a b c b a a c");
        try_stack!("ROTk NIP OVR", "a b c b a b");
        try_stack!("ROTk NIP OVRk", "a b c b a b a b");
        try_stack!("ROTk NIP ROTk", "a b c b a b a c");
        try_stack!("ROTk SWP", "a b c b a c");
        try_stack!("ROTk SWP OVR", "a b c b a c a");
        try_stack!("ROTk SWP OVRk", "a b c b a c a c a");
        try_stack!("ROTk SWP ROTk", "a b c b a c a c b");
        try_stack!("ROTk SWP DUP", "a b c b a c c");
        try_stack!("ROTk SWP SWPk", "a b c b a c c a");
        try_stack!("ROTk SWP DUPk", "a b c b a c c c");
        try_stack!("OVR DUP", "a b c b b");
        try_stack!("OVR DUPk", "a b c b b b");
        try_stack!("OVR DUP DUPk", "a b c b b b b");
        try_stack!("OVR DUP OVRk", "a b c b b b b b");
        try_stack!("OVR DUPk OVRk", "a b c b b b b b b");
        try_stack!("OVR DUP ROTk", "a b c b b b b c");
        try_stack!("OVR ROTk ROT", "a b c b b b c");
        try_stack!("OVR SWPk", "a b c b b c");
        try_stack!("OVR SWPk OVR", "a b c b b c b");
        try_stack!("OVR SWPk OVRk", "a b c b b c b c b");
        try_stack!("OVR SWPk DUP", "a b c b b c c");
        try_stack!("OVR SWPk SWPk", "a b c b b c c b");
        try_stack!("OVR SWPk DUPk", "a b c b b c c c");
        try_stack!("OVR OVR", "a b c b c");
        try_stack!("ROTk", "a b c b c a");
        try_stack!("ROTk DUP", "a b c b c a a");
        try_stack!("ROTk DUPk", "a b c b c a a a");
        try_stack!("ROTk DUP DUPk", "a b c b c a a a a");
        try_stack!("ROTk DUP OVRk", "a b c b c a a a a a");
        try_stack!("ROTk DUPk OVRk", "a b c b c a a a a a a");
        try_stack!("ROTk DUP ROTk", "a b c b c a a a a c");
        try_stack!("ROTk ROTk ROT", "a b c b c a a b c");
        try_stack!("ROTk SWPk", "a b c b c a a c");
        try_stack!("ROTk SWPk OVR", "a b c b c a a c a");
        try_stack!("ROTk SWPk OVRk", "a b c b c a a c a c a");
        try_stack!("ROTk SWPk DUP", "a b c b c a a c c");
        try_stack!("ROTk SWPk SWPk", "a b c b c a a c c a");
        try_stack!("ROTk SWPk DUPk", "a b c b c a a c c c");
        try_stack!("ROTk OVR", "a b c b c a c");
        try_stack!("ROTk OVR OVR", "a b c b c a c a");
        try_stack!("ROTk ROTk", "a b c b c a c a b");
        try_stack!("ROTk ROTk OVR", "a b c b c a c a b a");
        try_stack!("ROTk ROTk OVRk", "a b c b c a c a b a b a");
        try_stack!("ROTk ROTk ROTk", "a b c b c a c a b a b c");
        try_stack!("ROTk ROTk DUP", "a b c b c a c a b b");
        try_stack!("ROTk ROTk SWPk", "a b c b c a c a b b a");
        try_stack!("ROTk ROTk DUPk", "a b c b c a c a b b b");
        try_stack!("ROTk OVRk", "a b c b c a c a c");
        try_stack!("ROTk OVR OVRk", "a b c b c a c a c a");
        try_stack!("ROTk OVRk OVRk", "a b c b c a c a c a c a");
        try_stack!("ROTk OVRk ROTk", "a b c b c a c a c a c c");
        try_stack!("ROTk OVR ROTk", "a b c b c a c a c c");
        try_stack!("ROTk OVRk SWPk", "a b c b c a c a c c a");
        try_stack!("ROTk OVRk DUPk", "a b c b c a c a c c c");
        try_stack!("ROTk ROTk NIP", "a b c b c a c b");
        try_stack!("ROTk ROTk SWP", "a b c b c a c b a");
        try_stack!("ROTk OVR DUP", "a b c b c a c c");
        try_stack!("ROTk OVR SWPk", "a b c b c a c c a");
        try_stack!("ROTk OVR DUPk", "a b c b c a c c c");
        try_stack!("OVRk", "a b c b c b");
        try_stack!("OVR ROTk", "a b c b c b b");
        try_stack!("OVRk DUPk", "a b c b c b b b");
        try_stack!("OVR ROTk DUPk", "a b c b c b b b b");
        try_stack!("OVR ROTk OVRk", "a b c b c b b b b b");
        try_stack!("OVRk DUPk OVRk", "a b c b c b b b b b b");
        try_stack!("OVR ROTk ROTk", "a b c b c b b b b c");
        try_stack!("OVRk ROTk ROT", "a b c b c b b b c");
        try_stack!("OVRk SWPk", "a b c b c b b c");
        try_stack!("OVRk SWPk OVR", "a b c b c b b c b");
        try_stack!("OVRk SWPk OVRk", "a b c b c b b c b c b");
        try_stack!("OVRk SWPk DUP", "a b c b c b b c c");
        try_stack!("OVRk SWPk SWPk", "a b c b c b b c c b");
        try_stack!("OVRk SWPk DUPk", "a b c b c b b c c c");
        try_stack!("OVR OVRk", "a b c b c b c");
        try_stack!("OVR OVR OVRk", "a b c b c b c b");
        try_stack!("OVRk ROTk", "a b c b c b c b b");
        try_stack!("OVRk ROTk DUP", "a b c b c b c b b b");
        try_stack!("OVRk ROTk DUPk", "a b c b c b c b b b b");
        try_stack!("OVRk ROTk OVRk", "a b c b c b c b b b b b");
        try_stack!("OVRk ROTk ROTk", "a b c b c b c b b b b c");
        try_stack!("OVRk OVRk", "a b c b c b c b c");
        try_stack!("OVR OVRk OVRk", "a b c b c b c b c b");
        try_stack!("OVRk OVRk OVRk", "a b c b c b c b c b c b");
        try_stack!("OVRk OVRk ROTk", "a b c b c b c b c b c c");
        try_stack!("OVR OVRk ROTk", "a b c b c b c b c c");
        try_stack!("OVRk OVRk SWPk", "a b c b c b c b c c b");
        try_stack!("OVRk OVRk DUPk", "a b c b c b c b c c c");
        try_stack!("OVR OVR ROTk", "a b c b c b c c");
        try_stack!("OVR OVRk SWPk", "a b c b c b c c b");
        try_stack!("OVR OVRk DUPk", "a b c b c b c c c");
        try_stack!("OVR OVR DUP", "a b c b c c");
        try_stack!("ROTk OVR SWP", "a b c b c c a");
        try_stack!("OVR OVR SWPk", "a b c b c c b");
        try_stack!("OVR OVR DUPk", "a b c b c c c");
        try_stack!("DUP", "a b c c");
        try_stack!("ROTk ROT POP", "a b c c a");
        try_stack!("ROTk ROT", "a b c c a b");
        try_stack!("ROTk ROT OVR", "a b c c a b a");
        try_stack!("ROTk ROT OVRk", "a b c c a b a b a");
        try_stack!("ROTk ROT ROTk", "a b c c a b a b c");
        try_stack!("ROTk ROT DUP", "a b c c a b b");
        try_stack!("ROTk ROT SWPk", "a b c c a b b a");
        try_stack!("ROTk ROT DUPk", "a b c c a b b b");
        try_stack!("SWPk", "a b c c b");
        try_stack!("ROTk ROT SWP", "a b c c b a");
        try_stack!("SWPk DUP", "a b c c b b");
        try_stack!("SWPk DUPk", "a b c c b b b");
        try_stack!("SWPk DUP DUPk", "a b c c b b b b");
        try_stack!("SWPk DUP OVRk", "a b c c b b b b b");
        try_stack!("SWPk DUPk OVRk", "a b c c b b b b b b");
        try_stack!("SWPk DUP ROTk", "a b c c b b b b c");
        try_stack!("SWPk SWPk", "a b c c b b c");
        try_stack!("SWPk SWPk OVR", "a b c c b b c b");
        try_stack!("SWPk SWPk OVRk", "a b c c b b c b c b");
        try_stack!("SWPk SWPk DUP", "a b c c b b c c");
        try_stack!("SWPk SWPk SWPk", "a b c c b b c c b");
        try_stack!("SWPk SWPk DUPk", "a b c c b b c c c");
        try_stack!("SWPk OVR", "a b c c b c");
        try_stack!("SWPk OVR OVR", "a b c c b c b");
        try_stack!("SWPk OVRk", "a b c c b c b c");
        try_stack!("SWPk OVR OVRk", "a b c c b c b c b");
        try_stack!("SWPk OVRk OVRk", "a b c c b c b c b c b");
        try_stack!("SWPk OVRk ROTk", "a b c c b c b c b c c");
        try_stack!("SWPk OVR ROTk", "a b c c b c b c c");
        try_stack!("SWPk OVRk SWPk", "a b c c b c b c c b");
        try_stack!("SWPk OVRk DUPk", "a b c c b c b c c c");
        try_stack!("SWPk OVR DUP", "a b c c b c c");
        try_stack!("SWPk OVR SWPk", "a b c c b c c b");
        try_stack!("SWPk OVR DUPk", "a b c c b c c c");
        try_stack!("DUPk", "a b c c c");
        try_stack!("DUP ROTk NIP", "a b c c c b");
        try_stack!("DUP ROTk SWP", "a b c c c b c");
        try_stack!("DUP DUPk", "a b c c c c");
        try_stack!("DUP ROTk", "a b c c c c b");
        try_stack!("DUP ROTk DUP", "a b c c c c b b");
        try_stack!("DUP ROTk DUPk", "a b c c c c b b b");
        try_stack!("DUP ROTk SWPk", "a b c c c c b b c");
        try_stack!("DUP ROTk OVR", "a b c c c c b c");
        try_stack!("DUP ROTk OVRk", "a b c c c c b c b c");
        try_stack!("DUP OVRk", "a b c c c c c");
        try_stack!("DUPk OVRk", "a b c c c c c c");
        try_stack!("DUP DUPk OVRk", "a b c c c c c c c");
        try_stack!("DUP OVRk OVRk", "a b c c c c c c c c");
        try_stack!("DUPk OVRk OVRk", "a b c c c c c c c c c");
        try_stack!("NIP", "a c");
        try_stack!("NIP OVR", "a c a");
        try_stack!("NIP OVR DUP", "a c a a");
        try_stack!("NIP OVR DUPk", "a c a a a");
        try_stack!("NIP OVR SWPk", "a c a a c");
        try_stack!("NIP OVR OVR", "a c a c");
        try_stack!("NIP OVRk", "a c a c a");
        try_stack!("NIP OVR ROTk", "a c a c a a");
        try_stack!("NIP OVRk DUPk", "a c a c a a a");
        try_stack!("NIP OVRk SWPk", "a c a c a a c");
        try_stack!("NIP OVR OVRk", "a c a c a c");
        try_stack!("NIP OVRk ROTk", "a c a c a c a a");
        try_stack!("NIP OVRk OVRk", "a c a c a c a c");
        try_stack!("SWP", "a c b");
        try_stack!("SWP DUP", "a c b b");
        try_stack!("SWP ROTk ROT", "a c b b a c");
        try_stack!("SWP DUPk", "a c b b b");
        try_stack!("SWP DUP DUPk", "a c b b b b");
        try_stack!("SWP DUP OVRk", "a c b b b b b");
        try_stack!("SWP DUPk OVRk", "a c b b b b b b");
        try_stack!("SWP DUP ROTk", "a c b b b b c");
        try_stack!("SWP SWPk", "a c b b c");
        try_stack!("SWP SWPk OVR", "a c b b c b");
        try_stack!("SWP SWPk OVRk", "a c b b c b c b");
        try_stack!("SWP SWPk DUP", "a c b b c c");
        try_stack!("SWP SWPk SWPk", "a c b b c c b");
        try_stack!("SWP SWPk DUPk", "a c b b c c c");
        try_stack!("SWP OVR", "a c b c");
        try_stack!("SWP ROTk NIP", "a c b c a");
        try_stack!("SWP ROTk SWP", "a c b c a b");
        try_stack!("SWP OVR OVR", "a c b c b");
        try_stack!("SWP ROTk", "a c b c b a");
        try_stack!("SWP ROTk DUP", "a c b c b a a");
        try_stack!("SWP ROTk DUPk", "a c b c b a a a");
        try_stack!("SWP ROTk SWPk", "a c b c b a a b");
        try_stack!("SWP ROTk OVR", "a c b c b a b");
        try_stack!("SWP ROTk OVRk", "a c b c b a b a b");
        try_stack!("SWP ROTk ROTk", "a c b c b a b a c");
        try_stack!("SWP OVRk", "a c b c b c");
        try_stack!("SWP OVR OVRk", "a c b c b c b");
        try_stack!("SWP OVRk OVRk", "a c b c b c b c b");
        try_stack!("SWP OVRk ROTk", "a c b c b c b c c");
        try_stack!("SWP OVR ROTk", "a c b c b c c");
        try_stack!("SWP OVRk SWPk", "a c b c b c c b");
        try_stack!("SWP OVRk DUPk", "a c b c b c c c");
        try_stack!("SWP OVR DUP", "a c b c c");
        try_stack!("SWP OVR SWPk", "a c b c c b");
        try_stack!("SWP OVR DUPk", "a c b c c c");
        try_stack!("NIP DUP", "a c c");
        try_stack!("NIP SWPk", "a c c a");
        try_stack!("NIP SWPk DUP", "a c c a a");
        try_stack!("NIP SWPk DUPk", "a c c a a a");
        try_stack!("NIP SWPk SWPk", "a c c a a c");
        try_stack!("NIP SWPk OVR", "a c c a c");
        try_stack!("NIP SWPk OVRk", "a c c a c a c");
        try_stack!("DUP ROT", "a c c b");
        try_stack!("DUP ROT DUP", "a c c b b");
        try_stack!("DUP ROT DUPk", "a c c b b b");
        try_stack!("DUP ROT SWPk", "a c c b b c");
        try_stack!("DUP ROT OVR", "a c c b c");
        try_stack!("DUP ROT OVRk", "a c c b c b c");
        try_stack!("NIP DUPk", "a c c c");
        try_stack!("NIP DUP DUPk", "a c c c c");
        try_stack!("NIP DUP ROTk", "a c c c c a");
        try_stack!("NIP DUP OVRk", "a c c c c c");
        try_stack!("NIP DUPk OVRk", "a c c c c c c");
        try_stack!("POP NIP", "b");
        try_stack!("POP SWP", "b a");
        try_stack!("POP SWP DUP", "b a a");
        try_stack!("POP SWP DUPk", "b a a a");
        try_stack!("POP SWP SWPk", "b a a b");
        try_stack!("ROT DUP ROT", "b a a c");
        try_stack!("POP SWP OVR", "b a b");
        try_stack!("POP SWP OVRk", "b a b a b");
        try_stack!("ROT SWP", "b a c");
        try_stack!("ROT SWP OVR", "b a c a");
        try_stack!("ROT SWP OVRk", "b a c a c a");
        try_stack!("ROT SWP ROTk", "b a c a c b");
        try_stack!("ROT SWP DUP", "b a c c");
        try_stack!("ROT SWP SWPk", "b a c c a");
        try_stack!("ROT SWP DUPk", "b a c c c");
        try_stack!("POP NIP DUP", "b b");
        try_stack!("POP DUP ROT", "b b a");
        try_stack!("POP NIP DUPk", "b b b");
        try_stack!("ROT POP", "b c");
        try_stack!("ROT", "b c a");
        try_stack!("ROT DUP", "b c a a");
        try_stack!("ROT DUPk", "b c a a a");
        try_stack!("ROT DUP DUPk", "b c a a a a");
        try_stack!("ROT DUP OVRk", "b c a a a a a");
        try_stack!("ROT DUPk OVRk", "b c a a a a a a");
        try_stack!("ROT DUP ROTk", "b c a a a a c");
        try_stack!("ROT ROTk ROT", "b c a a b c");
        try_stack!("ROT SWPk", "b c a a c");
        try_stack!("ROT SWPk OVR", "b c a a c a");
        try_stack!("ROT SWPk OVRk", "b c a a c a c a");
        try_stack!("ROT SWPk DUP", "b c a a c c");
        try_stack!("ROT SWPk SWPk", "b c a a c c a");
        try_stack!("ROT SWPk DUPk", "b c a a c c c");
        try_stack!("ROT OVR", "b c a c");
        try_stack!("ROT OVR OVR", "b c a c a");
        try_stack!("ROT ROTk", "b c a c a b");
        try_stack!("ROT ROTk OVR", "b c a c a b a");
        try_stack!("ROT ROTk OVRk", "b c a c a b a b a");
        try_stack!("ROT ROTk ROTk", "b c a c a b a b c");
        try_stack!("ROT ROTk DUP", "b c a c a b b");
        try_stack!("ROT ROTk SWPk", "b c a c a b b a");
        try_stack!("ROT ROTk DUPk", "b c a c a b b b");
        try_stack!("ROT OVRk", "b c a c a c");
        try_stack!("ROT OVR OVRk", "b c a c a c a");
        try_stack!("ROT OVRk OVRk", "b c a c a c a c a");
        try_stack!("ROT OVRk ROTk", "b c a c a c a c c");
        try_stack!("ROT OVR ROTk", "b c a c a c c");
        try_stack!("ROT OVRk SWPk", "b c a c a c c a");
        try_stack!("ROT OVRk DUPk", "b c a c a c c c");
        try_stack!("ROT ROTk NIP", "b c a c b");
        try_stack!("ROT ROTk SWP", "b c a c b a");
        try_stack!("ROT OVR DUP", "b c a c c");
        try_stack!("ROT OVR SWPk", "b c a c c a");
        try_stack!("ROT OVR DUPk", "b c a c c c");
        try_stack!("ROT POP OVR", "b c b");
        try_stack!("ROT POP OVRk", "b c b c b");
        try_stack!("ROT POP DUP", "b c c");
        try_stack!("ROT OVR SWP", "b c c a");
        try_stack!("ROT POP SWPk", "b c c b");
        try_stack!("ROT POP DUPk", "b c c c");
        try_stack!("NIP NIP", "c");
        try_stack!("NIP SWP", "c a");
        try_stack!("NIP SWP DUP", "c a a");
        try_stack!("NIP SWP DUPk", "c a a a");
        try_stack!("NIP SWP SWPk", "c a a c");
        try_stack!("ROT ROT", "c a b");
        try_stack!("ROT ROT OVR", "c a b a");
        try_stack!("ROT ROT OVRk", "c a b a b a");
        try_stack!("ROT ROT ROTk", "c a b a b c");
        try_stack!("ROT ROT DUP", "c a b b");
        try_stack!("ROT ROT SWPk", "c a b b a");
        try_stack!("ROT ROT DUPk", "c a b b b");
        try_stack!("NIP SWP OVR", "c a c");
        try_stack!("NIP SWP OVRk", "c a c a c");
        try_stack!("SWP ROT POP", "c b");
        try_stack!("SWP ROT", "c b a");
        try_stack!("SWP ROT DUP", "c b a a");
        try_stack!("SWP ROT DUPk", "c b a a a");
        try_stack!("SWP ROT SWPk", "c b a a b");
        try_stack!("SWP ROT OVR", "c b a b");
        try_stack!("SWP ROT OVRk", "c b a b a b");
        try_stack!("SWP ROT ROTk", "c b a b a c");
        try_stack!("NIP NIP DUP", "c c");
        try_stack!("NIP DUP ROT", "c c a");
        try_stack!("NIP NIP DUPk", "c c c");
    }
}
