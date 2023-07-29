use crate::chunker::Chunk;
use crate::opcode::Opcode;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    MacroInvocation(String),
    Opcode(Opcode),
    RawByte(u8),
    LitByte(u8),
    LitShort(u16),
    PaddingAbsolute(u16),
    PaddingRelative(u16),
    CommentStart,
    CommentEnd,
    Ascii(String),
    LabelParent(String),
    LabelChild(String),
    Bracket,
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

impl Token {
    pub fn from_chunk(chunk: Chunk) -> Result<Token, String> {
        if chunk.value == "[" || chunk.value == "]" {
            return Ok(Token {
                token_type: TokenType::Bracket,
                chunk,
            });
        }
        if chunk.value == "(" {
            return Ok(Token {
                token_type: TokenType::CommentStart,
                chunk,
            });
        }
        if chunk.value == ")" {
            return Ok(Token {
                token_type: TokenType::CommentEnd,
                chunk,
            });
        }

        if let Ok(opcode) = Opcode::from_str(&chunk.value) {
            return Ok(Token {
                token_type: TokenType::Opcode(opcode),
                chunk,
            });
        }

        if &chunk.value.as_str()[0..1] == "|" {
            let number = &chunk.value[1..];
            if let Ok(short) = parse_short(number) {
                return Ok(Token {
                    token_type: TokenType::PaddingAbsolute(short),
                    chunk,
                });
            } else if let Ok(byte) = parse_byte(number) {
                return Ok(Token {
                    token_type: TokenType::PaddingAbsolute(byte.into()),
                    chunk,
                });
            } else {
                return Err("could not parse PaddingAbsolute".to_string());
            }
        }

        if &chunk.value.as_str()[0..1] == "$" {
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
                            value = value << 8;
                            value += byte as u16;
                        }
                        return Ok(Token {
                            token_type: TokenType::PaddingRelative(value),
                            chunk,
                        });
                    }
                }
                Err(_) => return Err("Could not parse hex".to_string()),
            }
        }

        if &chunk.value.as_str()[0..1] == "\"" {
            let value = chunk.value[1..].to_string();
            if value.is_empty() {
                return Err("empty ascii value".to_string());
            } else {
                return Ok(Token {
                    token_type: TokenType::Ascii(value),
                    chunk,
                });
            }
        }

        if &chunk.value.as_str()[0..1] == "@" {
            let value = chunk.value[1..].to_string();
            if value.is_empty() {
                return Err("empty label parent".to_string());
            } else {
                return Ok(Token {
                    token_type: TokenType::LabelParent(value),
                    chunk,
                });
            }
        }

        if &chunk.value.as_str()[0..1] == "&" {
            let value = chunk.value[1..].to_string();
            if value.is_empty() {
                return Err("empty label child".to_string());
            } else {
                return Ok(Token {
                    token_type: TokenType::LabelChild(value),
                    chunk,
                });
            }
        }

        if &chunk.value.as_str()[0..1] == "#" {
            if let Ok(byte) = parse_byte(&chunk.value[1..]) {
                return Ok(Token {
                    token_type: TokenType::LitByte(byte),
                    chunk,
                });
            }
            if let Ok(short) = parse_short(&chunk.value[1..]) {
                return Ok(Token {
                    token_type: TokenType::LitShort(short),
                    chunk,
                });
            }
            return Err("could not parse byte or short".to_string());
        }

        if let Ok(byte) = parse_byte(&chunk.value) {
            return Ok(Token {
                token_type: TokenType::RawByte(byte),
                chunk,
            });
        }

        Ok(Token {
            token_type: TokenType::MacroInvocation(chunk.value.clone()),
            chunk,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_match {
        ( $a:expr, $b:expr ) => {{
            let chunk = Chunk::new(String::from($a), 0, 0);
            let result = Token::from_chunk(chunk);
            assert!(result.is_ok());
            let tt = result.unwrap().token_type;
            assert_eq!(tt, $b);
        }};
    }

    #[test]
    fn it_works() {
        assert_match!("cat", TokenType::MacroInvocation(String::from("cat")));
        assert_match!("DUP", TokenType::Opcode(Opcode::DUP(false, false, false)));
        assert_match!("DUP2kr", TokenType::Opcode(Opcode::DUP(true, true, true)));
        assert_match!("12", TokenType::RawByte(0x12));
        assert_match!("|acab", TokenType::PaddingAbsolute(0xacab));
        assert_match!("|11", TokenType::PaddingAbsolute(0x0011));
    }

    #[test]
    fn ascii_works() {
        assert_match!("\"foobar", TokenType::Ascii("foobar".to_string()));
    }

    #[test]
    fn ascii_fails() {
        let chunk = Chunk::new("\"".to_string(), 0, 0);
        let result = Token::from_chunk(chunk);
        assert!(result.is_err());
    }

    #[test]
    fn lit_shorthand_works() {
        assert_match!("#13", TokenType::LitByte(0x13));
        assert_match!("#1312", TokenType::LitShort(0x1312));
    }

    #[test]
    fn lit_shorthand_fails() {
        let chunk = Chunk::new("#".to_string(), 0, 0);
        let result = Token::from_chunk(chunk);
        assert!(result.is_err());
        let chunk = Chunk::new("#123".to_string(), 0, 0);
        let result = Token::from_chunk(chunk);
        assert!(result.is_err());
    }

    #[test]
    fn label_parent_works() {
        assert_match!("@System", TokenType::LabelParent("System".to_string()));
    }

    #[test]
    fn label_parent_fails() {
        let chunk = Chunk::new("@".to_string(), 0, 0);
        let result = Token::from_chunk(chunk);
        assert!(result.is_err());
    }

    #[test]
    fn label_child_works() {
        assert_match!("&vector", TokenType::LabelChild("vector".to_string()));
    }

    #[test]
    fn label_child_fails() {
        let chunk = Chunk::new("&".to_string(), 0, 0);
        let result = Token::from_chunk(chunk);
        assert!(result.is_err());
    }

    #[test]
    fn bracket_works() {
        assert_match!("[", TokenType::Bracket);
        assert_match!("]", TokenType::Bracket);
    }

    #[test]
    fn padding_relative_works() {
        assert_match!("$5", TokenType::PaddingRelative(5));
        assert_match!("$400", TokenType::PaddingRelative(0x400));
    }

    #[test]
    fn padding_relative_fails() {
        let chunk = Chunk::new("$".to_string(), 0, 0);
        let result = Token::from_chunk(chunk);
        assert!(result.is_err());
    }
}
