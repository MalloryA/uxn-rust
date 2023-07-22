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

struct Breaker<'a> {
    bytes: Bytes<&'a mut dyn BufRead>,
    line: usize,
}

impl Breaker<'_> {
    pub fn new<'a>(reader: &'a mut dyn BufRead) -> Breaker<'a> {
        Breaker {
            bytes: reader.bytes(),
            line: 0,
        }
    }
}

impl Iterator for Breaker<'_> {
    type Item = Chunk;

    fn next(&mut self) -> Option<Self::Item> {
        let mut s: Vec<u8> = vec![];

        loop {
            match self.bytes.next() {
                None => {
                    //panic!("z");
                    return None;
                }
                Some(Err(_)) => {
                    //panic!("a");
                    return None;
                }
                Some(Ok(byte)) => match byte {
                    b'\n' | b'\t' => match String::from_utf8(s) {
                        Ok(string) => {
                            let value = Some(Chunk::new(string, self.line, 0));
                            if byte == b'\n' {
                                self.line += 1;
                            }
                            return value;
                        }
                        Err(_) => {
                            //panic!("b");
                            return None;
                        }
                    },
                    _ => {
                        s.push(byte);
                    }
                },
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
        let mut breaker = Breaker::new(&mut buffer);

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
