#[derive(PartialEq, Debug)]
enum Opcode {
    // Opcodes that don't take any arguments
    BRK,
    JCI,
    JMI,
    JSI,
    // Opcodes that take 2 and r
    LIT(bool, bool),
    // Opcodes that take 2 and k and r
    INC(bool, bool, bool),
    POP(bool, bool, bool),
    NIP(bool, bool, bool),
    SWP(bool, bool, bool),
    ROT(bool, bool, bool),
    DUP(bool, bool, bool),
    OVR(bool, bool, bool),
    EQU(bool, bool, bool),
    NEQ(bool, bool, bool),
    GTH(bool, bool, bool),
    LTH(bool, bool, bool),
    JMP(bool, bool, bool),
    JCN(bool, bool, bool),
    JSR(bool, bool, bool),
    STH(bool, bool, bool),
    LDZ(bool, bool, bool),
    STZ(bool, bool, bool),
    LDR(bool, bool, bool),
    STR(bool, bool, bool),
    LDA(bool, bool, bool),
    STA(bool, bool, bool),
    DEI(bool, bool, bool),
    DEO(bool, bool, bool),
    ADD(bool, bool, bool),
    SUB(bool, bool, bool),
    MUL(bool, bool, bool),
    DIV(bool, bool, bool),
    AND(bool, bool, bool),
    ORA(bool, bool, bool),
    EOR(bool, bool, bool),
    SFT(bool, bool, bool),
}

fn parse_modifiers(s: &str) -> Result<(bool, bool, bool), &str> {
    if s.contains(|chr| chr != '2' && chr != 'k' && chr != 'r') {
        Err("oh no")
    } else {
        let two = s.contains("2");
        let keep = s.contains("k");
        let return_stack = s.contains("r");

        Ok((two, keep, return_stack))
    }
}

fn modify(byte: u8, two: bool, keep: bool, return_stack: bool) -> u8 {
    let two: u8 = if two { 1 } else { 0 } << 5;
    let return_stack: u8 = if return_stack { 1 } else { 0 } << 6;
    let keep: u8 = if keep { 1 } else { 0 } << 7;
    byte | two | keep | return_stack
}

macro_rules! with_2r {
    ( $a:expr, $b:expr ) => {{
        let (two, _, return_stack) = parse_modifiers($b)?;
        Ok($a(two, return_stack))
    }};
}

macro_rules! with_2kr {
    ( $a:expr, $b:expr ) => {{
        let (two, keep, return_stack) = parse_modifiers($b)?;
        Ok($a(two, keep, return_stack))
    }};
}

impl Opcode {
    fn from_str(s: &str) -> Result<Opcode, &str> {
        let name = &s[..3];
        let modifiers = &s[3..];
        match name {
            "BRK" => Ok(Opcode::BRK),
            "JCI" => Ok(Opcode::JCI),
            "JMI" => Ok(Opcode::JMI),
            "JSI" => Ok(Opcode::JSI),
            // Opcodes that take 2 and r
            "LIT" => with_2r!(Opcode::LIT, modifiers),
            //    // Opcodes that take 2 and k and r
            "INC" => with_2kr!(Opcode::INC, modifiers),
            "POP" => with_2kr!(Opcode::POP, modifiers),
            "NIP" => with_2kr!(Opcode::NIP, modifiers),
            "SWP" => with_2kr!(Opcode::SWP, modifiers),
            "ROT" => with_2kr!(Opcode::ROT, modifiers),
            "DUP" => with_2kr!(Opcode::DUP, modifiers),
            "OVR" => with_2kr!(Opcode::OVR, modifiers),
            "EQU" => with_2kr!(Opcode::EQU, modifiers),
            "NEQ" => with_2kr!(Opcode::NEQ, modifiers),
            "GTH" => with_2kr!(Opcode::GTH, modifiers),
            "LTH" => with_2kr!(Opcode::LTH, modifiers),
            "JMP" => with_2kr!(Opcode::JMP, modifiers),
            "JCN" => with_2kr!(Opcode::JCN, modifiers),
            "JSR" => with_2kr!(Opcode::JSR, modifiers),
            "STH" => with_2kr!(Opcode::STH, modifiers),
            "LDZ" => with_2kr!(Opcode::LDZ, modifiers),
            "STZ" => with_2kr!(Opcode::STZ, modifiers),
            "LDR" => with_2kr!(Opcode::LDR, modifiers),
            "STR" => with_2kr!(Opcode::STR, modifiers),
            "LDA" => with_2kr!(Opcode::LDA, modifiers),
            "STA" => with_2kr!(Opcode::STA, modifiers),
            "DEI" => with_2kr!(Opcode::DEI, modifiers),
            "DEO" => with_2kr!(Opcode::DEO, modifiers),
            "ADD" => with_2kr!(Opcode::ADD, modifiers),
            "SUB" => with_2kr!(Opcode::SUB, modifiers),
            "MUL" => with_2kr!(Opcode::MUL, modifiers),
            "DIV" => with_2kr!(Opcode::DIV, modifiers),
            "AND" => with_2kr!(Opcode::AND, modifiers),
            "ORA" => with_2kr!(Opcode::ORA, modifiers),
            "EOR" => with_2kr!(Opcode::EOR, modifiers),
            "SFT" => with_2kr!(Opcode::SFT, modifiers),
            _ => Err("oh no"),
        }
    }

    fn as_byte(&self) -> u8 {
        match self {
            // Opcodes that don't take any arguments
            Opcode::BRK => 0x00,
            Opcode::JCI => 0x20,
            Opcode::JMI => 0x40,
            Opcode::JSI => 0x60,
            // Opcodes that take 2 and r
            Opcode::LIT(two, return_stack) => modify(0x80, *two, false, *return_stack),
            // Opcodes that take 2 and k and r
            Opcode::INC(two, keep, return_stack) => modify(0x01, *two, *keep, *return_stack),
            Opcode::POP(two, keep, return_stack) => modify(0x02, *two, *keep, *return_stack),
            Opcode::NIP(two, keep, return_stack) => modify(0x03, *two, *keep, *return_stack),
            Opcode::SWP(two, keep, return_stack) => modify(0x04, *two, *keep, *return_stack),
            Opcode::ROT(two, keep, return_stack) => modify(0x05, *two, *keep, *return_stack),
            Opcode::DUP(two, keep, return_stack) => modify(0x06, *two, *keep, *return_stack),
            Opcode::OVR(two, keep, return_stack) => modify(0x07, *two, *keep, *return_stack),
            Opcode::EQU(two, keep, return_stack) => modify(0x08, *two, *keep, *return_stack),
            Opcode::NEQ(two, keep, return_stack) => modify(0x09, *two, *keep, *return_stack),
            Opcode::GTH(two, keep, return_stack) => modify(0x0a, *two, *keep, *return_stack),
            Opcode::LTH(two, keep, return_stack) => modify(0x0b, *two, *keep, *return_stack),
            Opcode::JMP(two, keep, return_stack) => modify(0x0c, *two, *keep, *return_stack),
            Opcode::JCN(two, keep, return_stack) => modify(0x0d, *two, *keep, *return_stack),
            Opcode::JSR(two, keep, return_stack) => modify(0x0e, *two, *keep, *return_stack),
            Opcode::STH(two, keep, return_stack) => modify(0x0f, *two, *keep, *return_stack),
            Opcode::LDZ(two, keep, return_stack) => modify(0x10, *two, *keep, *return_stack),
            Opcode::STZ(two, keep, return_stack) => modify(0x11, *two, *keep, *return_stack),
            Opcode::LDR(two, keep, return_stack) => modify(0x12, *two, *keep, *return_stack),
            Opcode::STR(two, keep, return_stack) => modify(0x13, *two, *keep, *return_stack),
            Opcode::LDA(two, keep, return_stack) => modify(0x14, *two, *keep, *return_stack),
            Opcode::STA(two, keep, return_stack) => modify(0x15, *two, *keep, *return_stack),
            Opcode::DEI(two, keep, return_stack) => modify(0x16, *two, *keep, *return_stack),
            Opcode::DEO(two, keep, return_stack) => modify(0x17, *two, *keep, *return_stack),
            Opcode::ADD(two, keep, return_stack) => modify(0x18, *two, *keep, *return_stack),
            Opcode::SUB(two, keep, return_stack) => modify(0x19, *two, *keep, *return_stack),
            Opcode::MUL(two, keep, return_stack) => modify(0x1a, *two, *keep, *return_stack),
            Opcode::DIV(two, keep, return_stack) => modify(0x1b, *two, *keep, *return_stack),
            Opcode::AND(two, keep, return_stack) => modify(0x1c, *two, *keep, *return_stack),
            Opcode::ORA(two, keep, return_stack) => modify(0x1d, *two, *keep, *return_stack),
            Opcode::EOR(two, keep, return_stack) => modify(0x1e, *two, *keep, *return_stack),
            Opcode::SFT(two, keep, return_stack) => modify(0x1f, *two, *keep, *return_stack),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_match {
        ( $a:expr, $b:expr, $c:expr ) => {{
            let result = Opcode::from_str($a);
            assert!(result.is_ok());
            let opcode = result.unwrap();
            assert_eq!(opcode, $b, "{}", $a);

            let byte = opcode.as_byte();
            assert_eq!(byte, $c, "{}: expected 0x{:x} got 0x{:x}", $a, $c, byte);
        }};
    }

    #[test]
    fn it_handles_errors() {
        let result = Opcode::from_str("DOG");
        assert!(result.is_err());
    }

    #[test]
    fn it_works() {
        assert_match!("BRK", Opcode::BRK, 0x00);
        assert_match!("INC", Opcode::INC(false, false, false), 0x01);
        assert_match!("POP", Opcode::POP(false, false, false), 0x02);
        assert_match!("NIP", Opcode::NIP(false, false, false), 0x03);
        assert_match!("SWP", Opcode::SWP(false, false, false), 0x04);
        assert_match!("ROT", Opcode::ROT(false, false, false), 0x05);
        assert_match!("DUP", Opcode::DUP(false, false, false), 0x06);
        assert_match!("OVR", Opcode::OVR(false, false, false), 0x07);
        assert_match!("EQU", Opcode::EQU(false, false, false), 0x08);
        assert_match!("NEQ", Opcode::NEQ(false, false, false), 0x09);
        assert_match!("GTH", Opcode::GTH(false, false, false), 0x0a);
        assert_match!("LTH", Opcode::LTH(false, false, false), 0x0b);
        assert_match!("JMP", Opcode::JMP(false, false, false), 0x0c);
        assert_match!("JCN", Opcode::JCN(false, false, false), 0x0d);
        assert_match!("JSR", Opcode::JSR(false, false, false), 0x0e);
        assert_match!("STH", Opcode::STH(false, false, false), 0x0f);
        assert_match!("LDZ", Opcode::LDZ(false, false, false), 0x10);
        assert_match!("STZ", Opcode::STZ(false, false, false), 0x11);
        assert_match!("LDR", Opcode::LDR(false, false, false), 0x12);
        assert_match!("STR", Opcode::STR(false, false, false), 0x13);
        assert_match!("LDA", Opcode::LDA(false, false, false), 0x14);
        assert_match!("STA", Opcode::STA(false, false, false), 0x15);
        assert_match!("DEI", Opcode::DEI(false, false, false), 0x16);
        assert_match!("DEO", Opcode::DEO(false, false, false), 0x17);
        assert_match!("ADD", Opcode::ADD(false, false, false), 0x18);
        assert_match!("SUB", Opcode::SUB(false, false, false), 0x19);
        assert_match!("MUL", Opcode::MUL(false, false, false), 0x1a);
        assert_match!("DIV", Opcode::DIV(false, false, false), 0x1b);
        assert_match!("AND", Opcode::AND(false, false, false), 0x1c);
        assert_match!("ORA", Opcode::ORA(false, false, false), 0x1d);
        assert_match!("EOR", Opcode::EOR(false, false, false), 0x1e);
        assert_match!("SFT", Opcode::SFT(false, false, false), 0x1f);
        assert_match!("JCI", Opcode::JCI, 0x20);
        assert_match!("INC2", Opcode::INC(true, false, false), 0x21);
        assert_match!("POP2", Opcode::POP(true, false, false), 0x22);
        assert_match!("NIP2", Opcode::NIP(true, false, false), 0x23);
        assert_match!("SWP2", Opcode::SWP(true, false, false), 0x24);
        assert_match!("ROT2", Opcode::ROT(true, false, false), 0x25);
        assert_match!("DUP2", Opcode::DUP(true, false, false), 0x26);
        assert_match!("OVR2", Opcode::OVR(true, false, false), 0x27);
        assert_match!("EQU2", Opcode::EQU(true, false, false), 0x28);
        assert_match!("NEQ2", Opcode::NEQ(true, false, false), 0x29);
        assert_match!("GTH2", Opcode::GTH(true, false, false), 0x2a);
        assert_match!("LTH2", Opcode::LTH(true, false, false), 0x2b);
        assert_match!("JMP2", Opcode::JMP(true, false, false), 0x2c);
        assert_match!("JCN2", Opcode::JCN(true, false, false), 0x2d);
        assert_match!("JSR2", Opcode::JSR(true, false, false), 0x2e);
        assert_match!("STH2", Opcode::STH(true, false, false), 0x2f);
        assert_match!("LDZ2", Opcode::LDZ(true, false, false), 0x30);
        assert_match!("STZ2", Opcode::STZ(true, false, false), 0x31);
        assert_match!("LDR2", Opcode::LDR(true, false, false), 0x32);
        assert_match!("STR2", Opcode::STR(true, false, false), 0x33);
        assert_match!("LDA2", Opcode::LDA(true, false, false), 0x34);
        assert_match!("STA2", Opcode::STA(true, false, false), 0x35);
        assert_match!("DEI2", Opcode::DEI(true, false, false), 0x36);
        assert_match!("DEO2", Opcode::DEO(true, false, false), 0x37);
        assert_match!("ADD2", Opcode::ADD(true, false, false), 0x38);
        assert_match!("SUB2", Opcode::SUB(true, false, false), 0x39);
        assert_match!("MUL2", Opcode::MUL(true, false, false), 0x3a);
        assert_match!("DIV2", Opcode::DIV(true, false, false), 0x3b);
        assert_match!("AND2", Opcode::AND(true, false, false), 0x3c);
        assert_match!("ORA2", Opcode::ORA(true, false, false), 0x3d);
        assert_match!("EOR2", Opcode::EOR(true, false, false), 0x3e);
        assert_match!("SFT2", Opcode::SFT(true, false, false), 0x3f);
        assert_match!("JMI", Opcode::JMI, 0x40);
        assert_match!("INCr", Opcode::INC(false, false, true), 0x41);
        assert_match!("POPr", Opcode::POP(false, false, true), 0x42);
        assert_match!("NIPr", Opcode::NIP(false, false, true), 0x43);
        assert_match!("SWPr", Opcode::SWP(false, false, true), 0x44);
        assert_match!("ROTr", Opcode::ROT(false, false, true), 0x45);
        assert_match!("DUPr", Opcode::DUP(false, false, true), 0x46);
        assert_match!("OVRr", Opcode::OVR(false, false, true), 0x47);
        assert_match!("EQUr", Opcode::EQU(false, false, true), 0x48);
        assert_match!("NEQr", Opcode::NEQ(false, false, true), 0x49);
        assert_match!("GTHr", Opcode::GTH(false, false, true), 0x4a);
        assert_match!("LTHr", Opcode::LTH(false, false, true), 0x4b);
        assert_match!("JMPr", Opcode::JMP(false, false, true), 0x4c);
        assert_match!("JCNr", Opcode::JCN(false, false, true), 0x4d);
        assert_match!("JSRr", Opcode::JSR(false, false, true), 0x4e);
        assert_match!("STHr", Opcode::STH(false, false, true), 0x4f);
        assert_match!("LDZr", Opcode::LDZ(false, false, true), 0x50);
        assert_match!("STZr", Opcode::STZ(false, false, true), 0x51);
        assert_match!("LDRr", Opcode::LDR(false, false, true), 0x52);
        assert_match!("STRr", Opcode::STR(false, false, true), 0x53);
        assert_match!("LDAr", Opcode::LDA(false, false, true), 0x54);
        assert_match!("STAr", Opcode::STA(false, false, true), 0x55);
        assert_match!("DEIr", Opcode::DEI(false, false, true), 0x56);
        assert_match!("DEOr", Opcode::DEO(false, false, true), 0x57);
        assert_match!("ADDr", Opcode::ADD(false, false, true), 0x58);
        assert_match!("SUBr", Opcode::SUB(false, false, true), 0x59);
        assert_match!("MULr", Opcode::MUL(false, false, true), 0x5a);
        assert_match!("DIVr", Opcode::DIV(false, false, true), 0x5b);
        assert_match!("ANDr", Opcode::AND(false, false, true), 0x5c);
        assert_match!("ORAr", Opcode::ORA(false, false, true), 0x5d);
        assert_match!("EORr", Opcode::EOR(false, false, true), 0x5e);
        assert_match!("SFTr", Opcode::SFT(false, false, true), 0x5f);
        assert_match!("JSI", Opcode::JSI, 0x60);
        assert_match!("INC2r", Opcode::INC(true, false, true), 0x61);
        assert_match!("POP2r", Opcode::POP(true, false, true), 0x62);
        assert_match!("NIP2r", Opcode::NIP(true, false, true), 0x63);
        assert_match!("SWP2r", Opcode::SWP(true, false, true), 0x64);
        assert_match!("ROT2r", Opcode::ROT(true, false, true), 0x65);
        assert_match!("DUP2r", Opcode::DUP(true, false, true), 0x66);
        assert_match!("OVR2r", Opcode::OVR(true, false, true), 0x67);
        assert_match!("EQU2r", Opcode::EQU(true, false, true), 0x68);
        assert_match!("NEQ2r", Opcode::NEQ(true, false, true), 0x69);
        assert_match!("GTH2r", Opcode::GTH(true, false, true), 0x6a);
        assert_match!("LTH2r", Opcode::LTH(true, false, true), 0x6b);
        assert_match!("JMP2r", Opcode::JMP(true, false, true), 0x6c);
        assert_match!("JCN2r", Opcode::JCN(true, false, true), 0x6d);
        assert_match!("JSR2r", Opcode::JSR(true, false, true), 0x6e);
        assert_match!("STH2r", Opcode::STH(true, false, true), 0x6f);
        assert_match!("LDZ2r", Opcode::LDZ(true, false, true), 0x70);
        assert_match!("STZ2r", Opcode::STZ(true, false, true), 0x71);
        assert_match!("LDR2r", Opcode::LDR(true, false, true), 0x72);
        assert_match!("STR2r", Opcode::STR(true, false, true), 0x73);
        assert_match!("LDA2r", Opcode::LDA(true, false, true), 0x74);
        assert_match!("STA2r", Opcode::STA(true, false, true), 0x75);
        assert_match!("DEI2r", Opcode::DEI(true, false, true), 0x76);
        assert_match!("DEO2r", Opcode::DEO(true, false, true), 0x77);
        assert_match!("ADD2r", Opcode::ADD(true, false, true), 0x78);
        assert_match!("SUB2r", Opcode::SUB(true, false, true), 0x79);
        assert_match!("MUL2r", Opcode::MUL(true, false, true), 0x7a);
        assert_match!("DIV2r", Opcode::DIV(true, false, true), 0x7b);
        assert_match!("AND2r", Opcode::AND(true, false, true), 0x7c);
        assert_match!("ORA2r", Opcode::ORA(true, false, true), 0x7d);
        assert_match!("EOR2r", Opcode::EOR(true, false, true), 0x7e);
        assert_match!("SFT2r", Opcode::SFT(true, false, true), 0x7f);
        assert_match!("LIT", Opcode::LIT(false, false), 0x80);
        assert_match!("INCk", Opcode::INC(false, true, false), 0x81);
        assert_match!("POPk", Opcode::POP(false, true, false), 0x82);
        assert_match!("NIPk", Opcode::NIP(false, true, false), 0x83);
        assert_match!("SWPk", Opcode::SWP(false, true, false), 0x84);
        assert_match!("ROTk", Opcode::ROT(false, true, false), 0x85);
        assert_match!("DUPk", Opcode::DUP(false, true, false), 0x86);
        assert_match!("OVRk", Opcode::OVR(false, true, false), 0x87);
        assert_match!("EQUk", Opcode::EQU(false, true, false), 0x88);
        assert_match!("NEQk", Opcode::NEQ(false, true, false), 0x89);
        assert_match!("GTHk", Opcode::GTH(false, true, false), 0x8a);
        assert_match!("LTHk", Opcode::LTH(false, true, false), 0x8b);
        assert_match!("JMPk", Opcode::JMP(false, true, false), 0x8c);
        assert_match!("JCNk", Opcode::JCN(false, true, false), 0x8d);
        assert_match!("JSRk", Opcode::JSR(false, true, false), 0x8e);
        assert_match!("STHk", Opcode::STH(false, true, false), 0x8f);
        assert_match!("LDZk", Opcode::LDZ(false, true, false), 0x90);
        assert_match!("STZk", Opcode::STZ(false, true, false), 0x91);
        assert_match!("LDRk", Opcode::LDR(false, true, false), 0x92);
        assert_match!("STRk", Opcode::STR(false, true, false), 0x93);
        assert_match!("LDAk", Opcode::LDA(false, true, false), 0x94);
        assert_match!("STAk", Opcode::STA(false, true, false), 0x95);
        assert_match!("DEIk", Opcode::DEI(false, true, false), 0x96);
        assert_match!("DEOk", Opcode::DEO(false, true, false), 0x97);
        assert_match!("ADDk", Opcode::ADD(false, true, false), 0x98);
        assert_match!("SUBk", Opcode::SUB(false, true, false), 0x99);
        assert_match!("MULk", Opcode::MUL(false, true, false), 0x9a);
        assert_match!("DIVk", Opcode::DIV(false, true, false), 0x9b);
        assert_match!("ANDk", Opcode::AND(false, true, false), 0x9c);
        assert_match!("ORAk", Opcode::ORA(false, true, false), 0x9d);
        assert_match!("EORk", Opcode::EOR(false, true, false), 0x9e);
        assert_match!("SFTk", Opcode::SFT(false, true, false), 0x9f);
        assert_match!("LIT2", Opcode::LIT(true, false), 0xa0);
        assert_match!("INC2k", Opcode::INC(true, true, false), 0xa1);
        assert_match!("POP2k", Opcode::POP(true, true, false), 0xa2);
        assert_match!("NIP2k", Opcode::NIP(true, true, false), 0xa3);
        assert_match!("SWP2k", Opcode::SWP(true, true, false), 0xa4);
        assert_match!("ROT2k", Opcode::ROT(true, true, false), 0xa5);
        assert_match!("DUP2k", Opcode::DUP(true, true, false), 0xa6);
        assert_match!("OVR2k", Opcode::OVR(true, true, false), 0xa7);
        assert_match!("EQU2k", Opcode::EQU(true, true, false), 0xa8);
        assert_match!("NEQ2k", Opcode::NEQ(true, true, false), 0xa9);
        assert_match!("GTH2k", Opcode::GTH(true, true, false), 0xaa);
        assert_match!("LTH2k", Opcode::LTH(true, true, false), 0xab);
        assert_match!("JMP2k", Opcode::JMP(true, true, false), 0xac);
        assert_match!("JCN2k", Opcode::JCN(true, true, false), 0xad);
        assert_match!("JSR2k", Opcode::JSR(true, true, false), 0xae);
        assert_match!("STH2k", Opcode::STH(true, true, false), 0xaf);
        assert_match!("LDZ2k", Opcode::LDZ(true, true, false), 0xb0);
        assert_match!("STZ2k", Opcode::STZ(true, true, false), 0xb1);
        assert_match!("LDR2k", Opcode::LDR(true, true, false), 0xb2);
        assert_match!("STR2k", Opcode::STR(true, true, false), 0xb3);
        assert_match!("LDA2k", Opcode::LDA(true, true, false), 0xb4);
        assert_match!("STA2k", Opcode::STA(true, true, false), 0xb5);
        assert_match!("DEI2k", Opcode::DEI(true, true, false), 0xb6);
        assert_match!("DEO2k", Opcode::DEO(true, true, false), 0xb7);
        assert_match!("ADD2k", Opcode::ADD(true, true, false), 0xb8);
        assert_match!("SUB2k", Opcode::SUB(true, true, false), 0xb9);
        assert_match!("MUL2k", Opcode::MUL(true, true, false), 0xba);
        assert_match!("DIV2k", Opcode::DIV(true, true, false), 0xbb);
        assert_match!("AND2k", Opcode::AND(true, true, false), 0xbc);
        assert_match!("ORA2k", Opcode::ORA(true, true, false), 0xbd);
        assert_match!("EOR2k", Opcode::EOR(true, true, false), 0xbe);
        assert_match!("SFT2k", Opcode::SFT(true, true, false), 0xbf);
        assert_match!("LITr", Opcode::LIT(false, true), 0xc0);
        assert_match!("INCkr", Opcode::INC(false, true, true), 0xc1);
        assert_match!("POPkr", Opcode::POP(false, true, true), 0xc2);
        assert_match!("NIPkr", Opcode::NIP(false, true, true), 0xc3);
        assert_match!("SWPkr", Opcode::SWP(false, true, true), 0xc4);
        assert_match!("ROTkr", Opcode::ROT(false, true, true), 0xc5);
        assert_match!("DUPkr", Opcode::DUP(false, true, true), 0xc6);
        assert_match!("OVRkr", Opcode::OVR(false, true, true), 0xc7);
        assert_match!("EQUkr", Opcode::EQU(false, true, true), 0xc8);
        assert_match!("NEQkr", Opcode::NEQ(false, true, true), 0xc9);
        assert_match!("GTHkr", Opcode::GTH(false, true, true), 0xca);
        assert_match!("LTHkr", Opcode::LTH(false, true, true), 0xcb);
        assert_match!("JMPkr", Opcode::JMP(false, true, true), 0xcc);
        assert_match!("JCNkr", Opcode::JCN(false, true, true), 0xcd);
        assert_match!("JSRkr", Opcode::JSR(false, true, true), 0xce);
        assert_match!("STHkr", Opcode::STH(false, true, true), 0xcf);
        assert_match!("LDZkr", Opcode::LDZ(false, true, true), 0xd0);
        assert_match!("STZkr", Opcode::STZ(false, true, true), 0xd1);
        assert_match!("LDRkr", Opcode::LDR(false, true, true), 0xd2);
        assert_match!("STRkr", Opcode::STR(false, true, true), 0xd3);
        assert_match!("LDAkr", Opcode::LDA(false, true, true), 0xd4);
        assert_match!("STAkr", Opcode::STA(false, true, true), 0xd5);
        assert_match!("DEIkr", Opcode::DEI(false, true, true), 0xd6);
        assert_match!("DEOkr", Opcode::DEO(false, true, true), 0xd7);
        assert_match!("ADDkr", Opcode::ADD(false, true, true), 0xd8);
        assert_match!("SUBkr", Opcode::SUB(false, true, true), 0xd9);
        assert_match!("MULkr", Opcode::MUL(false, true, true), 0xda);
        assert_match!("DIVkr", Opcode::DIV(false, true, true), 0xdb);
        assert_match!("ANDkr", Opcode::AND(false, true, true), 0xdc);
        assert_match!("ORAkr", Opcode::ORA(false, true, true), 0xdd);
        assert_match!("EORkr", Opcode::EOR(false, true, true), 0xde);
        assert_match!("SFTkr", Opcode::SFT(false, true, true), 0xdf);
        assert_match!("LIT2r", Opcode::LIT(true, true), 0xe0);
        assert_match!("INC2kr", Opcode::INC(true, true, true), 0xe1);
        assert_match!("POP2kr", Opcode::POP(true, true, true), 0xe2);
        assert_match!("NIP2kr", Opcode::NIP(true, true, true), 0xe3);
        assert_match!("SWP2kr", Opcode::SWP(true, true, true), 0xe4);
        assert_match!("ROT2kr", Opcode::ROT(true, true, true), 0xe5);
        assert_match!("DUP2kr", Opcode::DUP(true, true, true), 0xe6);
        assert_match!("OVR2kr", Opcode::OVR(true, true, true), 0xe7);
        assert_match!("EQU2kr", Opcode::EQU(true, true, true), 0xe8);
        assert_match!("NEQ2kr", Opcode::NEQ(true, true, true), 0xe9);
        assert_match!("GTH2kr", Opcode::GTH(true, true, true), 0xea);
        assert_match!("LTH2kr", Opcode::LTH(true, true, true), 0xeb);
        assert_match!("JMP2kr", Opcode::JMP(true, true, true), 0xec);
        assert_match!("JCN2kr", Opcode::JCN(true, true, true), 0xed);
        assert_match!("JSR2kr", Opcode::JSR(true, true, true), 0xee);
        assert_match!("STH2kr", Opcode::STH(true, true, true), 0xef);
        assert_match!("LDZ2kr", Opcode::LDZ(true, true, true), 0xf0);
        assert_match!("STZ2kr", Opcode::STZ(true, true, true), 0xf1);
        assert_match!("LDR2kr", Opcode::LDR(true, true, true), 0xf2);
        assert_match!("STR2kr", Opcode::STR(true, true, true), 0xf3);
        assert_match!("LDA2kr", Opcode::LDA(true, true, true), 0xf4);
        assert_match!("STA2kr", Opcode::STA(true, true, true), 0xf5);
        assert_match!("DEI2kr", Opcode::DEI(true, true, true), 0xf6);
        assert_match!("DEO2kr", Opcode::DEO(true, true, true), 0xf7);
        assert_match!("ADD2kr", Opcode::ADD(true, true, true), 0xf8);
        assert_match!("SUB2kr", Opcode::SUB(true, true, true), 0xf9);
        assert_match!("MUL2kr", Opcode::MUL(true, true, true), 0xfa);
        assert_match!("DIV2kr", Opcode::DIV(true, true, true), 0xfb);
        assert_match!("AND2kr", Opcode::AND(true, true, true), 0xfc);
        assert_match!("ORA2kr", Opcode::ORA(true, true, true), 0xfd);
        assert_match!("EOR2kr", Opcode::EOR(true, true, true), 0xfe);
        assert_match!("SFT2kr", Opcode::SFT(true, true, true), 0xff);
    }
}
