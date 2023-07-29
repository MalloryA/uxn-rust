use crate::chunker::Chunk;
use crate::error::Error;
use crate::opcode::Opcode;
use crate::token::Token;
use crate::token::TokenType;
use std::fmt::Debug;
use std::fmt::Formatter;

#[derive(PartialEq)]
pub struct Rom {
    rom: [u8; 0xff00],
}

impl Debug for Rom {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("{}", hex::encode(self.get_bytes())))?;
        Ok(())
    }
}

impl Rom {
    pub fn new() -> Self {
        Rom { rom: [0; 0xff00] }
    }

    pub fn write_byte(&mut self, position: u16, byte: u8) {
        if position < 0x100 {
            panic!("Cannot write at i < 0x100 where i={:x}", position);
        }
        self.rom[position as usize - 0x100] = byte;
    }

    pub fn get_bytes(&self) -> &[u8] {
        let mut last_non_null: Option<usize> = None;
        let iter = self.rom.iter();
        for (i, byte) in iter.enumerate() {
            if *byte != 0x00 {
                last_non_null = Some(i);
            }
        }
        match last_non_null {
            None => &[],
            Some(size) => &self.rom[0..size + 1],
        }
    }
}

pub fn parse(chunks: &mut dyn Iterator<Item = Chunk>) -> Result<Rom, Error> {
    let mut comment_start: Option<Chunk> = None;
    let mut position: u16 = 0;

    let mut rom = Rom::new();

    loop {
        let next = chunks.next();

        if comment_start.is_some() {
            match next {
                None => {
                    return Err(Error::new(
                        "reached EOF without seeing a CommentEnd token".to_string(),
                        comment_start.unwrap(),
                    ))
                }
                Some(chunk) => match Token::from_chunk(chunk.clone()) {
                    Err(err) => return Err(Error::new(err, chunk)),
                    Ok(token) => match token.token_type {
                        TokenType::CommentEnd => comment_start = None,
                        _ => continue,
                    },
                },
            }
        } else {
            match next {
                None => {
                    return Ok(rom);
                }
                Some(chunk) => match Token::from_chunk(chunk.clone()) {
                    Err(err) => return Err(Error::new(err, chunk)),
                    Ok(token) => match token.token_type {
                        TokenType::Opcode(opcode) => {
                            rom.write_byte(position, opcode.as_byte());
                            position += 1;
                        }
                        TokenType::RawByte(byte) => {
                            rom.write_byte(position, byte);
                            position += 1;
                        }
                        TokenType::PaddingAbsolute(offset) => {
                            position = offset;
                        }
                        TokenType::PaddingRelative(offset) => {
                            position += offset;
                        }
                        TokenType::CommentStart => {
                            comment_start = Some(chunk);
                        }
                        TokenType::Ascii(value) => {
                            for byte in value.bytes() {
                                rom.write_byte(position, byte);
                                position += 1;
                            }
                        }
                        TokenType::LitByte(byte) => {
                            rom.write_byte(position, Opcode::LIT(false, false).as_byte());
                            position += 1;
                            rom.write_byte(position, byte);
                            position += 1;
                        }
                        TokenType::LitShort(short) => {
                            rom.write_byte(position, Opcode::LIT(false, false).as_byte());
                            let high: u8 = (short >> 8).try_into().unwrap();
                            let low: u8 = (short & 0xff).try_into().unwrap();
                            position += 1;
                            rom.write_byte(position, high);
                            position += 1;
                            rom.write_byte(position, low);
                            position += 1;
                        }
                        TokenType::LabelParent(name) => {
                            //TODO
                        }
                        TokenType::LabelChild(name) => {
                            //TODO
                        }
                        TokenType::Bracket => {
                            // Ignore
                        }
                        _ => todo!("{:?}", token),
                    },
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut expected = Rom::new();
        expected.write_byte(0x100, 0x80);
        expected.write_byte(0x101, 0x68);
        expected.write_byte(0x102, 0x80);
        expected.write_byte(0x103, 0x18);
        expected.write_byte(0x104, 0x17);

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

    #[test]
    fn rom() {
        let mut rom = Rom::new();
        rom.write_byte(0x100, 0x80);
        rom.write_byte(0x101, 0x68);
        rom.write_byte(0x102, 0x80);
        rom.write_byte(0x103, 0x18);
        // skip one
        rom.write_byte(0x105, 0x17);
        let expected: [u8; 6] = [0x80, 0x68, 0x80, 0x18, 0x00, 0x17];

        assert_eq!(rom.get_bytes(), expected);
    }
}
