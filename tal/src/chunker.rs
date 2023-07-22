use std::io::BufRead;
use std::io::Bytes;
use std::io::Read;

#[derive(Debug, PartialEq)]
struct Chunk {
    value: String,
    line: usize,
    column: usize,
}

impl Chunk {
    fn new(value: String, line: usize, column: usize) -> Chunk {
        Chunk {
            value,
            line,
            column,
        }
    }
}

struct Chunker<'a> {
    bytes: Bytes<&'a mut dyn BufRead>,
    line: usize,
    column: usize,
}

impl Chunker<'_> {
    pub fn new<'a>(reader: &'a mut dyn BufRead) -> Chunker {
        Chunker {
            bytes: reader.bytes(),
            line: 0,
            column: 0,
        }
    }
}

fn is_whitespace(byte: u8) -> bool {
    match byte {
        b' ' | b'\n' | b'\t' => true,
        _ => false,
    }
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
                    if s.len() > 0 {
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
                    if is_whitespace(byte) {
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
}
