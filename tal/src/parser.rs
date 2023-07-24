use crate::chunker::Chunk;
use crate::token::Token;
use crate::token::TokenType;

// uxn memory is 1_0000
// and the first 0100 is reserved for devices
// 1_0000 - 0100 = ff00
type Rom = [u8; 0xff00];

pub trait Romable {
    fn new() -> Rom;
    fn get_bytes(&self) -> &[u8];
}

impl Romable for Rom {
    fn new() -> Rom {
        [0; 0xff00]
    }

    fn get_bytes(&self) -> &[u8] {
        let mut last_non_null: Option<usize> = None;
        let mut i = 0;
        for byte in self {
            if *byte != 0x00 {
                last_non_null = Some(i);
            }
            i += 1;
        }
        match last_non_null {
            None => &[],
            Some(size) => &self[0..size + 1],
        }
    }
}

pub fn parse(chunks: &mut dyn Iterator<Item = Chunk>) -> Result<Rom, String> {
    let mut position = 0;

    let mut rom = Rom::new();

    loop {
        match chunks.next() {
            None => {
                return Ok(rom);
            }
            Some(chunk) => match Token::from_chunk(chunk) {
                Err(err) => return Err(err),
                Ok(token) => match token.token_type {
                    TokenType::Opcode(opcode) => {
                        rom[position] = opcode.as_byte();
                        position += 1;
                    }
                    TokenType::RawByte(byte) => {
                        rom[position] = byte;
                        position += 1;
                    }
                    TokenType::PositionReset(offset) => {
                        position = offset as usize - 0x100;
                    }
                    _ => todo!(),
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut expected = Rom::new();
        expected[0] = 0x80;
        expected[1] = 0x68;
        expected[2] = 0x80;
        expected[3] = 0x18;
        expected[4] = 0x17;

        let mut chunks = vec![
            Chunk::new(String::from("|0100"), 0, 0),
            Chunk::new(String::from("LIT"), 0, 7),
            Chunk::new(String::from("68"), 0, 11),
            Chunk::new(String::from("LIT"), 0, 14),
            Chunk::new(String::from("18"), 0, 18),
            Chunk::new(String::from("DEO"), 0, 21),
        ]
        .into_iter();
        let result = parse(&mut chunks);
        assert!(result.is_ok());
        let rom = result.unwrap();
        assert_eq!(rom, expected);
    }

    fn rom() {
        let mut rom = Rom::new();
        rom[0] = 0x80;
        rom[1] = 0x68;
        rom[2] = 0x80;
        rom[3] = 0x18;
        // skip one
        rom[5] = 0x17;
        let expected: [u8; 6] = [0x80, 0x68, 0x80, 0x18, 0x00, 0x17];

        assert_eq!(rom.get_bytes(), expected);
    }
}
