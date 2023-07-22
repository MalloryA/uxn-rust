use crate::chunker::Chunk;
use crate::token::Token;

// uxn memory is 1_0000
// and the first 0100 is reserved for devices
// 1_0000 - 0100 = ff00
type Rom = [u8; 0xff00];

trait New {
    fn new() -> Rom;
}

impl New for Rom {
    fn new() -> Rom {
        [0; 0xff00]
    }
}

fn parse(tokens: &mut dyn Iterator<Item = Token>) -> Result<Rom, &str> {
    Err("foo")
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

        let mut tokens = vec![
            Token::from_chunk(Chunk::new(String::from("|0100"), 0, 0)).unwrap(),
            Token::from_chunk(Chunk::new(String::from("LIT"), 0, 7)).unwrap(),
            Token::from_chunk(Chunk::new(String::from("68"), 0, 11)).unwrap(),
            Token::from_chunk(Chunk::new(String::from("LIT"), 0, 14)).unwrap(),
            Token::from_chunk(Chunk::new(String::from("18"), 0, 18)).unwrap(),
            Token::from_chunk(Chunk::new(String::from("DEO"), 0, 21)).unwrap(),
        ]
        .into_iter();
        let result = parse(&mut tokens);
        assert!(result.is_ok());
        let rom = result.unwrap();
        assert_eq!(rom, expected);
    }
}
