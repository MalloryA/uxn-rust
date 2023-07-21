use crate::opcodes::Opcode;
use std::io::BufRead;
use std::sync::mpsc::SyncSender;

struct Token {
    token_type: TokenType,
    line_number: u32,
    column_number: u32,
    length: u32,
}

impl Token {
    pub fn new(token_type: TokenType, line_number: u32, column_number: u32, length: u32) -> Token {
        Token {
            token_type,
            line_number,
            column_number,
            length,
        }
    }
}

#[derive(PartialEq, Debug)]
enum TokenType {
    Opcode(Opcode),
    End,
}

struct Tokenizer<'a> {
    reader: &'a mut dyn BufRead,
    tx: &'a mut SyncSender<Token>,
}

impl Tokenizer<'_> {
    pub fn new<'a>(reader: &'a mut dyn BufRead, tx: &'a mut SyncSender<Token>) -> Tokenizer<'a> {
        Tokenizer { reader, tx }
    }

    pub fn go(&mut self) -> Result<(), String> {
        let mut line = String::new();
        let result = self.reader.read_line(&mut line);
        if let Err(err) = result {
            return Err(err.to_string());
        }

        for token in line.split_whitespace() {
            let result = Opcode::from_str(token);
            if let Err(err) = result {
                return Err(err.to_string());
            }
            let tt = TokenType::Opcode(result.unwrap());
            let token = Token::new(tt, 0, 0, 0);
            let result = self.tx.send(token);
            if let Err(err) = result {
                return Err(err.to_string());
            }
        }

        let token = Token::new(TokenType::End, 0, 0, 0);
        let result = self.tx.send(token);
        if let Err(err) = result {
            return Err(err.to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::sync::mpsc::sync_channel;

    #[test]
    fn it_handles_opcodes() {
        let mut buffer = Cursor::new("LIT DUP");
        let (mut tx, mut rx) = sync_channel::<Token>(3);

        let mut tokenizer = Tokenizer::new(&mut buffer, &mut tx);
        let result = tokenizer.go();
        assert!(result.is_ok());

        let a = rx.try_recv();
        assert!(a.is_ok());
        let token = a.unwrap();
        assert_eq!(
            token.token_type,
            TokenType::Opcode(Opcode::LIT(false, false))
        );

        let b = rx.try_recv();
        assert!(b.is_ok());
        let token = b.unwrap();
        assert_eq!(
            token.token_type,
            TokenType::Opcode(Opcode::DUP(false, false, false))
        );

        let c = rx.try_recv();
        assert!(c.is_ok());
        let token = c.unwrap();
        assert_eq!(token.token_type, TokenType::End);
    }
}
