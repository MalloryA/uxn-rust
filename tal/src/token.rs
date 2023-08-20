use crate::chunker::Chunk;
use crate::opcode::Opcode;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Instant(String),
    Opcode(Opcode),
    RawByte(u8),
    RawShort(u16),
    LiteralByte(u8),
    LiteralShort(u16),
    PaddingAbsolute(u16),
    PaddingRelative(u16),
    RawAscii(String),
    LabelParent(String),
    LabelChild(String),
    AddressLiteralAbsoluteByte(String, bool),
    AddressLiteralAbsoluteShort(String, bool),
    ImmediateConditional(String, bool),
    ImmediateUnconditional(String, bool),
    AddressLiteralRelative(String, bool),
    AddressRawAbsoluteByte(String, bool),
    AddressRawAbsoluteShort(String, bool),
    AddressRawRelative(String, bool),
}

impl TokenType {
    pub fn from_chunk(chunk: &Chunk) -> Result<TokenType, String> {
        // Match opcode

        if let Ok(opcode) = Opcode::from_str(&chunk.value) {
            return Ok(TokenType::Opcode(opcode));
        }

        // Raw byte/short

        if let Ok(byte) = parse_byte(&chunk.value) {
            return Ok(TokenType::RawByte(byte));
        }

        if let Ok(short) = parse_short(&chunk.value) {
            return Ok(TokenType::RawShort(short));
        }

        // Match first character

        let token_type = match &chunk.value.as_str()[0..1] {
            "|" => {
                let number = &chunk.value[1..];
                if let Ok(short) = parse_short(number) {
                    TokenType::PaddingAbsolute(short)
                } else if let Ok(byte) = parse_byte(number) {
                    TokenType::PaddingAbsolute(byte.into())
                } else {
                    return Err("could not parse PaddingAbsolute".to_string());
                }
            }

            "$" => {
                let number = &chunk.value[1..];
                let number = if number.len() & 1 == 1 {
                    "0".to_string() + number
                } else {
                    number.to_string()
                };
                match hex::decode(number.as_str()) {
                    Ok(bytes) => {
                        if bytes.is_empty() {
                            return Err("no bytes".to_string());
                        } else {
                            let mut value: u16 = 0;
                            for byte in bytes {
                                value <<= 8;
                                value += byte as u16;
                            }
                            TokenType::PaddingRelative(value)
                        }
                    }
                    Err(_) => return Err("Could not parse hex".to_string()),
                }
            }

            "\"" => {
                let value = chunk.value[1..].to_string();
                if value.is_empty() {
                    return Err("empty ascii value".to_string());
                } else {
                    TokenType::RawAscii(value)
                }
            }

            "@" => {
                let value = chunk.value[1..].to_string();
                if value.is_empty() {
                    return Err("empty label parent".to_string());
                } else {
                    TokenType::LabelParent(value)
                }
            }

            "&" => {
                let value = chunk.value[1..].to_string();
                TokenType::LabelChild(value)
            }

            "#" => {
                if let Ok(byte) = parse_byte(&chunk.value[1..]) {
                    TokenType::LiteralByte(byte)
                } else if let Ok(short) = parse_short(&chunk.value[1..]) {
                    TokenType::LiteralShort(short)
                } else {
                    return Err("could not parse byte or short".to_string());
                }
            }

            "." => {
                let (name, child) = parse_name(&chunk.value[1..]);
                TokenType::AddressLiteralAbsoluteByte(name.to_string(), child)
            }

            ";" => {
                let (name, child) = parse_name(&chunk.value[1..]);
                TokenType::AddressLiteralAbsoluteShort(name.to_string(), child)
            }

            // Unclear why : and = do the same thing
            ":" | "=" => {
                let (name, child) = parse_name(&chunk.value[1..]);
                TokenType::AddressRawAbsoluteShort(name.to_string(), child)
            }

            "," => {
                let (name, child) = parse_name(&chunk.value[1..]);
                TokenType::AddressLiteralRelative(name.to_string(), child)
            }

            "-" => {
                let (name, child) = parse_name(&chunk.value[1..]);
                TokenType::AddressRawAbsoluteByte(name.to_string(), child)
            }

            "_" => {
                let (name, child) = parse_name(&chunk.value[1..]);
                TokenType::AddressRawRelative(name.to_string(), child)
            }

            "!" => {
                let (name, child) = parse_name(&chunk.value[1..]);
                TokenType::ImmediateUnconditional(name.to_string(), child)
            }

            "?" => {
                let (name, child) = parse_name(&chunk.value[1..]);
                TokenType::ImmediateConditional(name.to_string(), child)
            }

            _ => TokenType::Instant(chunk.value.clone()),
        };
        Ok(token_type)
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub chunk: Chunk,
}

fn parse_byte(s: &str) -> Result<u8, String> {
    if s.len() != 2 {
        Err("not 2 bytes long".to_string())
    } else {
        match hex::decode(s) {
            Ok(bytes) => Ok(bytes[0]),
            Err(_) => Err("Could not parse hex".to_string()),
        }
    }
}

fn parse_short(s: &str) -> Result<u16, String> {
    if s.len() != 4 {
        Err("not 4 bytes long".to_string())
    } else {
        match hex::decode(s) {
            Ok(bytes) => {
                let short: u16 = (bytes[0] as u16) << 8 | bytes[1] as u16;
                Ok(short)
            }
            Err(_) => Err("Could not parse hex".to_string()),
        }
    }
}

fn parse_name(s: &str) -> (&str, bool) {
    if s.is_empty() {
        ("", false)
    } else if &s[0..1] == "&" {
        (&s[1..], true)
    } else {
        (s, false)
    }
}

impl Token {
    pub fn from_chunk(chunk: &Chunk) -> Result<Token, String> {
        let token_type = TokenType::from_chunk(chunk)?;

        Ok(Token {
            token_type,
            chunk: chunk.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_match {
        ( $a:expr, $b:expr ) => {{
            let chunk = Chunk::new(String::from($a), 0, 0);
            let result = Token::from_chunk(&chunk);
            assert!(result.is_ok());
            let tt = result.unwrap().token_type;
            assert_eq!(tt, $b);
        }};
    }

    macro_rules! assert_err {
        ( $a:expr ) => {{
            let chunk = Chunk::new($a.to_string(), 0, 0);
            let result = Token::from_chunk(&chunk);
            assert!(result.is_err());
        }};
    }

    #[test]
    fn it_works() {
        assert_match!("cat", TokenType::Instant(String::from("cat")));
        assert_match!("DUP", TokenType::Opcode(Opcode::DUP(false, false, false)));
        assert_match!("DUP2kr", TokenType::Opcode(Opcode::DUP(true, true, true)));
        assert_match!("12", TokenType::RawByte(0x12));
        assert_match!("|acab", TokenType::PaddingAbsolute(0xacab));
        assert_match!("|11", TokenType::PaddingAbsolute(0x0011));
    }

    #[test]
    fn ascii_works() {
        assert_match!("\"foobar", TokenType::RawAscii("foobar".to_string()));
    }

    #[test]
    fn ascii_fails() {
        assert_err!("\"");
    }

    #[test]
    fn lit_shorthand_works() {
        assert_match!("#13", TokenType::LiteralByte(0x13));
        assert_match!("#1312", TokenType::LiteralShort(0x1312));
    }

    #[test]
    fn lit_shorthand_fails() {
        assert_err!("#");
        assert_err!("#123");
    }

    #[test]
    fn label_parent_works() {
        assert_match!("@System", TokenType::LabelParent("System".to_string()));
    }

    #[test]
    fn label_parent_fails() {
        assert_err!("@");
    }

    #[test]
    fn label_child_works() {
        assert_match!("&vector", TokenType::LabelChild("vector".to_string()));
        assert_match!("&", TokenType::LabelChild("".to_string()));
    }

    #[test]
    fn padding_relative_works() {
        assert_match!("$5", TokenType::PaddingRelative(5));
        assert_match!("$400", TokenType::PaddingRelative(0x400));
    }

    #[test]
    fn padding_relative_fails() {
        assert_err!("$");
    }

    #[test]
    fn raw_byte_works() {
        assert_match!("AB", TokenType::RawByte(0xab));
    }

    #[test]
    fn raw_byte_fails() {
        let chunk = Chunk::new("A".to_string(), 0, 0);
        let result = Token::from_chunk(&chunk);
        assert_eq!(
            result.unwrap().token_type,
            TokenType::Instant("A".to_string())
        );
    }

    #[test]
    fn raw_short_works() {
        assert_match!("ABCD", TokenType::RawShort(0xabcd));
    }

    #[test]
    fn raw_short_fails() {
        let chunk = Chunk::new("ABC".to_string(), 0, 0);
        let result = Token::from_chunk(&chunk);
        assert_eq!(
            result.unwrap().token_type,
            TokenType::Instant("ABC".to_string())
        );
    }

    #[test]
    fn address_literal_zero_page_works() {
        assert_match!(
            ".Foo/bar",
            TokenType::AddressLiteralAbsoluteByte("Foo/bar".to_string(), false)
        );
        assert_match!(
            ".Foo",
            TokenType::AddressLiteralAbsoluteByte("Foo".to_string(), false)
        );
        assert_match!(
            ".&bar",
            TokenType::AddressLiteralAbsoluteByte("bar".to_string(), true)
        );
        assert_match!(
            ".",
            TokenType::AddressLiteralAbsoluteByte("".to_string(), false)
        );
        assert_match!(
            ".&",
            TokenType::AddressLiteralAbsoluteByte("".to_string(), true)
        );
    }

    #[test]
    fn address_literal_absolute_works() {
        assert_match!(
            ";foo-bar",
            TokenType::AddressLiteralAbsoluteShort("foo-bar".to_string(), false)
        );
        assert_match!(
            ";&foo-bar",
            TokenType::AddressLiteralAbsoluteShort("foo-bar".to_string(), true)
        );
        assert_match!(
            ";",
            TokenType::AddressLiteralAbsoluteShort("".to_string(), false)
        );
        assert_match!(
            ";&",
            TokenType::AddressLiteralAbsoluteShort("".to_string(), true)
        );
    }

    #[test]
    fn address_raw_absolute_works() {
        assert_match!(
            ":foo-bar",
            TokenType::AddressRawAbsoluteShort("foo-bar".to_string(), false)
        );
        assert_match!(
            ":&foo-bar",
            TokenType::AddressRawAbsoluteShort("foo-bar".to_string(), true)
        );
        assert_match!(
            "-foo-bar",
            TokenType::AddressRawAbsoluteByte("foo-bar".to_string(), false)
        );
        assert_match!(
            "-&foo-bar",
            TokenType::AddressRawAbsoluteByte("foo-bar".to_string(), true)
        );
        assert_match!(
            ":",
            TokenType::AddressRawAbsoluteShort("".to_string(), false)
        );
        assert_match!(
            ":&",
            TokenType::AddressRawAbsoluteShort("".to_string(), true)
        );
        assert_match!(
            "-",
            TokenType::AddressRawAbsoluteByte("".to_string(), false)
        );
        assert_match!(
            "-&",
            TokenType::AddressRawAbsoluteByte("".to_string(), true)
        );
    }

    #[test]
    fn opcode_takes_precidence_over_raw_short() {
        assert_match!("ADD2", TokenType::Opcode(Opcode::ADD(true, false, false)));
    }

    #[test]
    fn immediate_unconditional_works() {
        assert_match!(
            "!foo-bar",
            TokenType::ImmediateUnconditional("foo-bar".to_string(), false)
        );
        assert_match!(
            "!&foo-bar",
            TokenType::ImmediateUnconditional("foo-bar".to_string(), true)
        );
        assert_match!(
            "!&",
            TokenType::ImmediateUnconditional("".to_string(), true)
        );
    }
}
