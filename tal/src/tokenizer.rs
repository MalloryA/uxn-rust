use crate::opcodes::Opcode;
use std::io::BufRead;
use std::sync::mpsc::SyncSender;

// line and column numbers are zero-indexed (at least internally)
struct Token {
    token_type: TokenType,
    line_number: usize,
    column_number: usize,
    length: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        line_number: usize,
        column_number: usize,
        length: usize,
    ) -> Token {
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

        loop {
            match line.find(' ') {
                Some(n) => {
                    let s = &line[0..n];
                    println!("{s:?}");
                    let result = Opcode::from_str(s);
                    if let Err(err) = result {
                        return Err(err.to_string());
                    }
                    let tt = TokenType::Opcode(result.unwrap());
                    let token = Token::new(tt, 0, 0, s.len());
                    let result = self.tx.send(token);
                    if let Err(err) = result {
                        return Err(err.to_string());
                    }

                    line = String::from(&line[n..]);
                }
                None => {
                    let token = Token::new(TokenType::End, 0, 0, 0);
                    let result = self.tx.send(token);
                    if let Err(err) = result {
                        return Err(err.to_string());
                    }
                    return Ok(());
                }
            }
        }
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
        assert_eq!(token.line_number, 0);
        assert_eq!(token.column_number, 0);
        assert_eq!(token.length, 3);

        let b = rx.try_recv();
        assert!(b.is_ok());
        let token = b.unwrap();
        assert_eq!(
            token.token_type,
            TokenType::Opcode(Opcode::DUP(false, false, false))
        );
        assert_eq!(token.line_number, 0);
        assert_eq!(token.column_number, 4);
        assert_eq!(token.length, 3);

        let c = rx.try_recv();
        assert!(c.is_ok());
        let token = c.unwrap();
        assert_eq!(token.token_type, TokenType::End);
        assert_eq!(token.line_number, 0);
        assert_eq!(token.column_number, 7);
        assert_eq!(token.length, 0);
    }
}
