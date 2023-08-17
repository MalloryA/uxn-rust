use crate::chunker::Chunk;
use crate::chunker::Chunker;
use crate::error::Error;
use crate::opcode::Opcode;
use crate::pre_process_brackets::PreProcessBrackets;
use crate::pre_process_comments::PreProcessComments;
use crate::pre_process_includes::PreProcessIncludes;
use crate::pre_process_macros::PreProcessMacros;
use crate::token::Token;
use crate::token::TokenType;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

fn split_short(short: u16) -> (u8, u8) {
    let high: u8 = (short >> 8).try_into().unwrap();
    let low: u8 = (short & 0xff).try_into().unwrap();
    (high, low)
}

enum FillLater {
    // u16: Address to fill in later
    // bool: Relative?
    // u16: subtract -> when using relative mode, subtract an extra value
    // String: Name
    // Chunk
    Byte(u16, bool, u16, String, Chunk),
    Short(u16, bool, u16, String, Chunk),
}

#[derive(PartialEq)]
pub struct Rom {
    rom: [u8; 0xff00],
    highest_byte_written: usize,
}

impl Debug for Rom {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for (i, byte) in self.get_bytes().iter().enumerate() {
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
        Rom {
            rom: [0; 0xff00],
            highest_byte_written: 0,
        }
    }

    pub fn write_byte(&mut self, position: u16, byte: u8) {
        if position < 0x100 {
            panic!("Cannot write at i < 0x100 where i={:x}", position);
        }
        let real_position = position as usize - 0x100;
        if real_position > self.highest_byte_written {
            self.highest_byte_written = real_position;
        }
        self.rom[real_position] = byte;
    }

    pub fn get_bytes(&self) -> &[u8] {
        &self.rom[0..self.highest_byte_written + 1]
    }
}

fn get_full_name(name: String, parent: &Option<String>, child: bool) -> String {
    if child {
        if parent.is_none() {
            format!("/{name}")
        } else {
            let p = parent.clone().unwrap();
            format!("{p}/{name}")
        }
    } else {
        name
    }
}

fn parse(
    file: PathBuf,
    chunks: &mut dyn Iterator<Item = Result<Chunk, Error>>,
) -> Result<Rom, Error> {
    let mut position: u16 = 0x100;

    // Addresses, references, etc

    // The current parent address
    let mut parent: Option<String> = None;
    // Map of names to the addresses they refer to
    let mut address_references: HashMap<String, u16> = HashMap::new();
    // Map of addresses to the names of references that should be filled in
    let mut fill_later: Vec<FillLater> = vec![];

    let mut rom = Rom::new();

    for chunk in chunks {
        let chunk = chunk?;
        match Token::from_chunk(chunk.clone()) {
            Err(err) => return Err(Error::new(err, chunk, file)),
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
                TokenType::AddressLiteralZeroPage(name, child) => {
                    let full_name = get_full_name(name, &parent, child);

                    rom.write_byte(position, Opcode::LIT(false, false).as_byte());
                    position += 1;

                    fill_later.push(FillLater::Byte(position, false, 0, full_name, chunk));
                    position += 1;
                }
                TokenType::AddressLiteralAbsolute(name, child) => {
                    let full_name = get_full_name(name, &parent, child);

                    rom.write_byte(position, Opcode::LIT(true, false).as_byte());
                    position += 1;

                    fill_later.push(FillLater::Short(position, false, 0, full_name, chunk));
                    position += 2;
                }
                TokenType::AddressLiteralRelative(name, child) => {
                    let full_name = get_full_name(name, &parent, child);

                    rom.write_byte(position, Opcode::LIT(false, false).as_byte());
                    position += 1;

                    fill_later.push(FillLater::Byte(position, true, 2, full_name, chunk));
                    position += 1;
                }
                TokenType::AddressRawAbsoluteByte(name, child) => {
                    let full_name = get_full_name(name, &parent, child);

                    fill_later.push(FillLater::Byte(position, false, 0, full_name, chunk));
                    position += 1;
                }
                TokenType::AddressRawAbsoluteShort(name, child) => {
                    let full_name = get_full_name(name, &parent, child);

                    fill_later.push(FillLater::Short(position, false, 0, full_name, chunk));
                    position += 2;
                }
                TokenType::ImmediateUnconditional(name, child) => {
                    let full_name = get_full_name(name, &parent, child);

                    rom.write_byte(position, Opcode::JMI.as_byte());
                    position += 1;

                    fill_later.push(FillLater::Short(position, true, 2, full_name, chunk));
                    position += 2;
                }
                TokenType::ImmediateConditional(name, child) => {
                    let full_name = get_full_name(name, &parent, child);

                    rom.write_byte(position, Opcode::JCI.as_byte());
                    position += 1;

                    fill_later.push(FillLater::Short(position, true, 2, full_name, chunk));
                    position += 2;
                }
                TokenType::LabelParent(name) => {
                    parent = Some(name.clone());
                    address_references.insert(name, position);
                }
                TokenType::LabelChild(name) => {
                    let full_name = get_full_name(name, &parent, true);
                    address_references.insert(full_name, position);
                }
                TokenType::Instant(name) => {
                    rom.write_byte(position, Opcode::JSI.as_byte());
                    position += 1;
                    fill_later.push(FillLater::Short(position, true, 2, name, chunk));
                    position += 2;
                }
            },
        }
    }

    // Fill in all the fill_laters
    for fill in fill_later {
        match fill {
            FillLater::Byte(target, relative, relative_subtract, name, chunk) => {
                let source = address_references.get(&name);
                if source.is_none() {
                    return Err(Error::new(format!("unknown name \"{name}\""), chunk, file));
                }
                let mut source = *source.unwrap();
                if relative {
                    // Unclear why uxnasm wraps these
                    source = source.wrapping_sub(target).wrapping_sub(relative_subtract);
                }
                let (_high, low) = split_short(source);
                rom.write_byte(target, low);
            }
            FillLater::Short(target, relative, relative_subtract, name, chunk) => {
                let source = address_references.get(&name);
                if source.is_none() {
                    return Err(Error::new(format!("unknown name \"{name}\""), chunk, file));
                }
                let mut source = *source.unwrap();
                if relative {
                    source = source.wrapping_sub(target).wrapping_sub(relative_subtract);
                }
                let (high, low) = split_short(source);
                rom.write_byte(target, high);
                rom.write_byte(target + 1, low);
            }
        }
    }

    Ok(rom)
}

pub fn chunk_file(cwd: &Path, file: &Path) -> Vec<Result<Chunk, Error>> {
    let full_path = cwd.join(file);
    let mut input = BufReader::new(File::open(full_path).unwrap());
    let mut chunker = Chunker::new(&mut input);
    pre_process(cwd, file.to_path_buf(), &mut chunker)
}

pub fn parse_chunks(
    cwd: &Path,
    file: PathBuf,
    input: &mut dyn Iterator<Item = Result<Chunk, Error>>,
) -> Result<Rom, Error> {
    parse(
        file.clone(),
        &mut pre_process(cwd, file.clone(), input).into_iter(),
    )
}

pub fn pre_process(
    cwd: &Path,
    file: PathBuf,
    input: &mut dyn Iterator<Item = Result<Chunk, Error>>,
) -> Vec<Result<Chunk, Error>> {
    let mut pp = input;
    let mut pp = PreProcessBrackets::new(&mut pp);
    let mut pp = PreProcessComments::new(file.clone(), &mut pp);
    let mut pp = PreProcessIncludes::new(cwd, &mut pp);
    let pp = PreProcessMacros::new(&mut pp);
    pp.collect()
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
        expected.write_byte(0x108, 0x80);
        expected.write_byte(0x109, 0x00);
        expected.write_byte(0x10a, 0x37);

        let mut chunks = vec![
            Ok(Chunk::new(String::from("|00"), 0, 0)),
            Ok(Chunk::new(String::from("@System"), 0, 4)),
            Ok(Chunk::new(String::from("&vector"), 0, 12)),
            Ok(Chunk::new(String::from("|0100"), 1, 0)),
            Ok(Chunk::new(String::from("LIT"), 1, 7)),
            Ok(Chunk::new(String::from("68"), 1, 11)),
            Ok(Chunk::new(String::from("LIT"), 1, 14)),
            Ok(Chunk::new(String::from("18"), 1, 18)),
            Ok(Chunk::new(String::from("DEO"), 1, 21)),
            Ok(Chunk::new(String::from(".System/vector"), 2, 0)),
            Ok(Chunk::new(String::from("DEO2"), 2, 15)),
            Ok(Chunk::new(String::from(".&vector"), 3, 0)),
            Ok(Chunk::new(String::from("DEO2"), 3, 15)),
        ]
        .into_iter();
        let result = parse_chunks(Path::new(""), PathBuf::new(), &mut chunks);
        assert!(result.is_ok());
        let rom = result.unwrap();
        assert_eq!(rom, expected);
    }

    #[test]
    fn forward_references_work() {
        let mut expected = Rom::new();
        expected.write_byte(0x100, 0x80);
        expected.write_byte(0x101, 0x02);
        expected.write_byte(0x102, 0xa0);
        expected.write_byte(0x103, 0x12);
        expected.write_byte(0x104, 0x34);
        expected.write_byte(0x105, 0xa0);
        expected.write_byte(0x106, 0x56);
        expected.write_byte(0x107, 0x78);

        let mut chunks = vec![
            Ok(Chunk::new(String::from("|0100"), 0, 0)),
            Ok(Chunk::new(String::from(",foo"), 0, 6)),
            Ok(Chunk::new(String::from("#1234"), 0, 11)),
            Ok(Chunk::new(String::from("@foo"), 0, 17)),
            Ok(Chunk::new(String::from("#5678"), 0, 23)),
        ]
        .into_iter();
        let result = parse_chunks(Path::new(""), PathBuf::new(), &mut chunks);
        assert!(result.is_ok());
        let rom = result.unwrap();
        assert_eq!(rom, expected);
    }

    #[test]
    fn backward_references_work() {
        let mut expected = Rom::new();
        expected.write_byte(0x100, 0xa0);
        expected.write_byte(0x101, 0x12);
        expected.write_byte(0x102, 0x34);
        expected.write_byte(0x103, 0x80);
        expected.write_byte(0x104, 0xfa);
        expected.write_byte(0x105, 0xa0);
        expected.write_byte(0x106, 0x56);
        expected.write_byte(0x107, 0x78);

        let mut chunks = vec![
            Ok(Chunk::new(String::from("|0100"), 0, 0)),
            Ok(Chunk::new(String::from("@bar"), 0, 6)),
            Ok(Chunk::new(String::from("#1234"), 0, 11)),
            Ok(Chunk::new(String::from(",bar"), 0, 17)),
            Ok(Chunk::new(String::from("#5678"), 0, 23)),
        ]
        .into_iter();
        let result = parse_chunks(Path::new(""), PathBuf::new(), &mut chunks);
        assert!(result.is_ok());
        let rom = result.unwrap();
        assert_eq!(rom, expected);
    }

    #[test]
    fn comments_work() {
        let mut expected = Rom::new();
        expected.write_byte(0x100, 0xa0);
        expected.write_byte(0x101, 0x12);
        expected.write_byte(0x102, 0x34);

        let mut chunks = vec![
            Ok(Chunk::new(String::from("|0100"), 0, 0)),
            Ok(Chunk::new(String::from("("), 0, 6)),
            Ok(Chunk::new(String::from("#"), 0, 8)),
            Ok(Chunk::new(String::from(")"), 0, 10)),
            Ok(Chunk::new(String::from("#1234"), 0, 12)),
        ]
        .into_iter();
        let result = parse_chunks(Path::new(""), PathBuf::new(), &mut chunks);
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
        // write a null
        rom.write_byte(0x106, 0x00);
        let expected: [u8; 7] = [0x80, 0x68, 0x80, 0x18, 0x00, 0x17, 0x00];

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

    #[test]
    fn macros_work() {
        let mut expected = Rom::new();
        expected.write_byte(0x100, 0xa0);
        expected.write_byte(0x101, 0x12);
        expected.write_byte(0x102, 0x34);
        expected.write_byte(0x103, 0x80);
        expected.write_byte(0x104, 0x18);
        expected.write_byte(0x105, 0x17);

        let mut chunks = vec![
            Ok(Chunk::new(String::from("%EMIT"), 0, 0)),
            Ok(Chunk::new(String::from("{"), 0, 6)),
            Ok(Chunk::new(String::from("#18"), 0, 8)),
            Ok(Chunk::new(String::from("DEO"), 0, 12)),
            Ok(Chunk::new(String::from("}"), 0, 15)),
            Ok(Chunk::new(String::from("#1234"), 0, 17)),
            Ok(Chunk::new(String::from("EMIT"), 0, 23)),
        ]
        .into_iter();
        let result = parse_chunks(Path::new(""), PathBuf::new(), &mut chunks);
        assert!(result.is_ok());
        let rom = result.unwrap();
        assert_eq!(rom, expected);
    }

    fn assert_match(input: &str, expected: &str) {
        let mut expected_rom = Rom::new();
        let bytes = hex::decode(expected.replace(" ", "")).unwrap();
        for (i, byte) in bytes.iter().enumerate() {
            expected_rom.write_byte((0x100 + i).try_into().unwrap(), *byte);
        }

        let mut buffer = Cursor::new(input);
        let mut chunks = Chunker::new(&mut buffer);

        let result = parse_chunks(Path::new("not-a-file.tal"), PathBuf::new(), &mut chunks);
        assert!(result.is_ok());
        let rom = result.unwrap();
        assert_eq!(rom, expected_rom);
    }

    #[test]
    fn it_works_001() {
        let expected = "800a 8018 1780 0a80 1817";

        let input = "
            %EMIT { #18 DEO }
            %OPCODE { #0a EMIT }
            %TYPE { OPCODE OPCODE }

            |0100

                    TYPE
        ";

        assert_match(input, expected);
    }

    #[test]
    fn it_works_002() {
        let expected = "13 13";

        let input = "
            %FOO { 13 }
            %BAR { FOO FOO }
            |0100 BAR
        ";

        assert_match(input, expected);
    }
}
