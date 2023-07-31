use crate::chunker::Chunk;
use crate::error::Error;
use crate::opcode::Opcode;
use crate::token::Token;
use crate::token::TokenType;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;

fn split_short(short: u16) -> (u8, u8) {
    let high: u8 = (short >> 8).try_into().unwrap();
    let low: u8 = (short & 0xff).try_into().unwrap();
    (high, low)
}

enum FillLater {
    Byte(u16, String, Chunk),
    Short(u16, String, Chunk),
}

#[derive(PartialEq)]
pub struct Rom {
    rom: [u8; 0xff00],
}

impl Debug for Rom {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for (i, byte) in self.get_bytes().into_iter().enumerate() {
            if i != 0 {
                if i % 16 == 0 {
                    f.write_str("\n")?;
                } else if i % 2 == 0 {
                    f.write_str(" ")?;
                }
            }
            f.write_fmt(format_args!("{:02x}", byte))?;
        }
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

    // Addresses, references, etc

    // The current parent address
    let mut parent: Option<String> = None;
    // Map of names to the addresses they refer to
    let mut address_references: HashMap<String, u16> = HashMap::new();
    // Map of addresses to the names of references that should be filled in
    let mut fill_later: Vec<FillLater> = vec![];

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
                    // Fill in all the fill_laters
                    for fill in fill_later {
                        match fill {
                            FillLater::Byte(target, name, chunk) => {
                                let source = address_references.get(&name);
                                if source == None {
                                    return Err(Error::new(
                                        format!("unknown name \"{name}\""),
                                        chunk,
                                    ));
                                }
                                let source = source.unwrap();
                                let (_high, low) = split_short(*source);
                                rom.write_byte(target, low);
                            }
                            FillLater::Short(target, name, chunk) => {
                                let source = address_references.get(&name);
                                if source == None {
                                    return Err(Error::new(
                                        format!("unknown name \"{name}\""),
                                        chunk,
                                    ));
                                }
                                let source = source.unwrap();
                                let (high, low) = split_short(*source);
                                rom.write_byte(target, high);
                                rom.write_byte(target + 1, low);
                            }
                        }
                    }

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
                        TokenType::RawShort(short) => {
                            let (high, low) = split_short(short);
                            rom.write_byte(position, high);
                            position += 1;
                            rom.write_byte(position, low);
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
                            rom.write_byte(position, Opcode::LIT(true, false).as_byte());
                            position += 1;

                            let (high, low) = split_short(short);
                            rom.write_byte(position, high);
                            position += 1;
                            rom.write_byte(position, low);
                            position += 1;
                        }
                        TokenType::AddressLiteralZeroPage(name) => {
                            rom.write_byte(position, Opcode::LIT(false, false).as_byte());
                            position += 1;

                            fill_later.push(FillLater::Byte(position, name, chunk));
                            position += 1;
                        }
                        TokenType::AddressLiteralAbsolute(name) => {
                            rom.write_byte(position, Opcode::LIT(true, false).as_byte());
                            position += 1;

                            fill_later.push(FillLater::Short(position, name, chunk));
                            position += 2;
                        }
                        TokenType::LabelParent(name) => {
                            parent = Some(name.clone());
                            address_references.insert(name, position);
                        }
                        TokenType::LabelChild(name) => {
                            if let Some(parent_name) = parent.clone() {
                                let full_name = format!("{}/{}", parent_name, name);
                                address_references.insert(full_name, position);
                            } else {
                                return Err(Error::new(
                                    "child label specified before parent label".to_string(),
                                    chunk,
                                ));
                            }
                        }
                        TokenType::MacroOrInstant(name) => {
                            // TODO: Assume instant (JSI)
                            rom.write_byte(position, Opcode::JSI.as_byte());
                            position += 1;
                            fill_later.push(FillLater::Short(position, name, chunk));
                            position += 2;
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
    use std::io::Cursor;

    #[test]
    fn it_works() {
        let mut expected = Rom::new();
        expected.write_byte(0x100, 0x80);
        expected.write_byte(0x101, 0x68);
        expected.write_byte(0x102, 0x80);
        expected.write_byte(0x103, 0x18);
        expected.write_byte(0x104, 0x17);
        expected.write_byte(0x105, 0x80);
        expected.write_byte(0x106, 0x00);
        expected.write_byte(0x107, 0x37);

        let mut chunks = vec![
            Chunk::new(String::from("|00"), 0, 0),
            Chunk::new(String::from("@System"), 0, 4),
            Chunk::new(String::from("&vector"), 0, 12),
            Chunk::new(String::from("|0100"), 1, 0),
            Chunk::new(String::from("LIT"), 1, 7),
            Chunk::new(String::from("68"), 1, 11),
            Chunk::new(String::from("LIT"), 1, 14),
            Chunk::new(String::from("18"), 1, 18),
            Chunk::new(String::from("DEO"), 1, 21),
            Chunk::new(String::from(".System/vector"), 2, 0),
            Chunk::new(String::from("DEO2"), 2, 15),
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

    #[test]
    fn rom_debug() {
        let mut rom = Rom::new();
        rom.write_byte(0x100, 0x80);
        rom.write_byte(0x101, 0x68);
        rom.write_byte(0x102, 0x80);
        rom.write_byte(0x103, 0x18);
        // skip a few
        rom.write_byte(0x125, 0x17);

        let actual = format!("{:?}", rom);
        let expected = "8068 8018 0000 0000 0000 0000 0000 0000\n0000 0000 0000 0000 0000 0000 0000 0000\n0000 0000 0017".to_string();
        assert_eq!(actual, expected);
    }
}
