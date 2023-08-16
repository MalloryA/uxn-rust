use crate::chunker::Chunk;
use crate::error::Error;
use std::path::PathBuf;

#[derive(PartialEq)]
enum CommentToken {
    CommentStart,
    CommentEnd,
    Other,
}

impl CommentToken {
    fn from_chunk(chunk: &Chunk) -> CommentToken {
        match &chunk.value[..] {
            "(" => CommentToken::CommentStart,
            ")" => CommentToken::CommentEnd,
            _ => CommentToken::Other,
        }
    }
}

pub struct PreProcessComments<'a> {
    file: PathBuf,
    chunks: &'a mut dyn Iterator<Item = Result<Chunk, Error>>,
    comment_start: Option<Chunk>,
}

impl PreProcessComments<'_> {
    pub fn new(
        file: PathBuf,
        chunks: &mut dyn Iterator<Item = Result<Chunk, Error>>,
    ) -> PreProcessComments {
        PreProcessComments {
            file,
            chunks,
            comment_start: None,
        }
    }
}

impl Iterator for PreProcessComments<'_> {
    type Item = Result<Chunk, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.chunks.next();

            if let Some(Ok(chunk)) = next {
                // If we're in comment mode then we only care if our next token is a CommentEnd
                if self.comment_start.is_some() {
                    if CommentToken::from_chunk(&chunk) == CommentToken::CommentEnd {
                        self.comment_start = None;
                    }
                    continue;
                }

                // If our chunk represents a CommentStart then drop us into comment mode and skip to
                // the next iteration
                if CommentToken::from_chunk(&chunk) == CommentToken::CommentStart {
                    self.comment_start = Some(chunk);
                    continue;
                }
                return Some(Ok(chunk));
            }

            if next.is_none() && self.comment_start.is_some() {
                return Some(Err(Error::new(
                    "reached EOF without finding comment close".to_string(),
                    self.comment_start.clone().unwrap(),
                    self.file.clone(),
                )));
            }

            return next;
        }
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
        let mut pp = PreProcessComments::new(PathBuf::new(), &mut source);

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
        let mut pp = PreProcessComments::new(PathBuf::new(), &mut source);

        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("cat"), 0, 0))));
        assert_eq!(
            pp.next(),
            Some(Err(Error::new(
                "reached EOF without finding comment close".to_string(),
                Chunk::new("(".to_string(), 0, 0),
                PathBuf::new(),
            )))
        );
    }
}
