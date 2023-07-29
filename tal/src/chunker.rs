use std::io::BufRead;
use std::io::Bytes;
use std::io::Read;

#[derive(Debug, PartialEq, Clone)]
pub struct Chunk {
    pub value: String,
    pub line: usize,
    pub column: usize,
}

impl Chunk {
    pub fn new(value: String, line: usize, column: usize) -> Chunk {
        Chunk {
            value,
            line,
            column,
        }
    }
}

pub struct Chunker<'a> {
    bytes: Bytes<&'a mut dyn BufRead>,
    line: usize,
    column: usize,
}

impl Chunker<'_> {
    pub fn new(reader: &mut dyn BufRead) -> Chunker {
        Chunker {
            bytes: reader.bytes(),
            line: 0,
            column: 0,
        }
    }
}

fn is_whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\n' | b'\t')
}

impl Iterator for Chunker<'_> {
    type Item = Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        let mut s: Vec<u8> = vec![];

        loop {
            let column = self.column - s.len();
            match self.bytes.next() {
                Some(Err(_)) => {
                    return None;
                }
                None => {
                    if !s.is_empty() {
                        match String::from_utf8(s) {
                            Ok(string) => {
                                return Some(Chunk::new(string, self.line, column));
                            }
                            Err(_) => {
                                return None;
                            }
                        }
                    }
                    return None;
                }
                Some(Ok(byte)) => {
                    if is_whitespace(byte) && !s.is_empty() {
                        match String::from_utf8(s) {
                            Ok(string) => {
                                self.column += 1;
                                let value = Some(Chunk::new(string, self.line, column));
                                if byte == b'\n' {
                                    self.line += 1;
                                    self.column = 0;
                                }
                                return value;
                            }
                            Err(_) => {
                                return None;
                            }
                        }
                    } else if is_whitespace(byte) {
                        if byte == b'\n' {
                            self.line += 1;
                            self.column = 0;
                        } else {
                            self.column += 1;
                        }
                    } else {
                        s.push(byte);
                        self.column += 1;
                    }
                }
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
        let mut buffer = Cursor::new("cat\ndog\trat possum");
        let mut breaker = Chunker::new(&mut buffer);

        assert_eq!(breaker.next(), Some(Chunk::new(String::from("cat"), 0, 0)));
        assert_eq!(breaker.next(), Some(Chunk::new(String::from("dog"), 1, 0)));
        assert_eq!(breaker.next(), Some(Chunk::new(String::from("rat"), 1, 4)));
        assert_eq!(
            breaker.next(),
            Some(Chunk::new(String::from("possum"), 1, 8))
        );
        assert_eq!(breaker.next(), None);
    }

    #[test]
    fn it_works2() {
        let mut buffer = Cursor::new("cat\n\ndog\trat possum");
        let mut breaker = Chunker::new(&mut buffer);

        assert_eq!(breaker.next(), Some(Chunk::new(String::from("cat"), 0, 0)));
        assert_eq!(breaker.next(), Some(Chunk::new(String::from("dog"), 2, 0)));
        assert_eq!(breaker.next(), Some(Chunk::new(String::from("rat"), 2, 4)));
        assert_eq!(
            breaker.next(),
            Some(Chunk::new(String::from("possum"), 2, 8))
        );
        assert_eq!(breaker.next(), None);
    }
}
