use crate::chunker::Chunk;
use crate::error::Error;
use std::io::BufRead;
use std::io::Bytes;
use std::io::Read;

pub struct PreProcessComments<'a> {
    chunks: &'a mut dyn Iterator<Item = Result<Chunk, Error>>,
}

impl PreProcessComments<'_> {
    pub fn new(chunks: &mut dyn Iterator<Item = Result<Chunk, Error>>) -> PreProcessComments {
        PreProcessComments { chunks }
    }
}

impl Iterator for PreProcessComments<'_> {
    type Item = Result<Chunk, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.chunks.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut source = vec![
            Ok(Chunk::new(String::from("cat"), 0, 0)),
            Ok(Chunk::new(String::from("("), 0, 0)),
            Ok(Chunk::new(String::from("woof"), 0, 0)),
            Ok(Chunk::new(String::from(")"), 0, 0)),
            Ok(Chunk::new(String::from("dog"), 0, 0)),
        ]
        .into_iter();
        let mut pp = PreProcessComments::new(&mut source);

        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("cat"), 0, 0))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("dog"), 0, 0))));
        assert_eq!(pp.next(), None);
    }

    #[test]
    fn it_fails() {
        let mut source = vec![
            Ok(Chunk::new(String::from("cat"), 0, 0)),
            Ok(Chunk::new(String::from("("), 0, 0)),
            Ok(Chunk::new(String::from("woof"), 0, 0)),
            Ok(Chunk::new(String::from("dog"), 0, 0)),
        ]
        .into_iter();
        let mut pp = PreProcessComments::new(&mut source);

        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("cat"), 0, 0))));
        assert_eq!(
            pp.next(),
            Some(Err(Error::new(
                "reached EOF without finding comment close".to_string(),
                Chunk::new("(".to_string(), 0, 0)
            )))
        );
    }
}
