use std::io::BufRead;
use std::io::Read;
use std::sync::mpsc::SyncSender;

#[derive(PartialEq, Debug)]
enum Token {
    Opcode(String),
    EOF,
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
        if result.is_err() {
            return Err(result.unwrap_err().to_string());
        }

        for token in line.split_whitespace() {
            let result = self.tx.send(Token::Opcode(String::from(token)));
            if result.is_err() {
                return Err(result.unwrap_err().to_string());
            }
        }

        let result = self.tx.send(Token::EOF);
        if result.is_err() {
            return Err(result.unwrap_err().to_string());
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
        assert_eq!(a.unwrap(), Token::Opcode(String::from("LIT")));

        let b = rx.try_recv();
        assert!(b.is_ok());
        assert_eq!(b.unwrap(), Token::Opcode(String::from("DUP")));

        let c = rx.try_recv();
        assert!(c.is_ok());
        assert_eq!(c.unwrap(), Token::EOF);
    }
}
