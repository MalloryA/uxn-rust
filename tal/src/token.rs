use crate::chunker::Chunk;

#[derive(Debug, PartialEq)]
enum TokenType {
    EndOfFile,
    MacroInvocation(String),
}

#[derive(Debug, PartialEq)]
struct Token {
    token_type: TokenType,
    chunk: Chunk,
}

impl Token {
    fn from_chunk(chunk: Chunk) -> Result<Token, String> {
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
    }
}
