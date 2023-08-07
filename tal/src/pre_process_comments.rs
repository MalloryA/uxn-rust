use crate::chunker::Chunk;
use crate::error::Error;
use crate::token::Token;
use crate::token::TokenType;

pub struct PreProcessComments<'a> {
    chunks: &'a mut dyn Iterator<Item = Result<Chunk, Error>>,
    comment_start: Option<Chunk>,
}

impl PreProcessComments<'_> {
    pub fn new(chunks: &mut dyn Iterator<Item = Result<Chunk, Error>>) -> PreProcessComments {
        PreProcessComments {
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
                    if let Ok(token) = Token::from_chunk(chunk.clone()) {
                        if token.token_type == TokenType::CommentEnd {
                            self.comment_start = None;
                        }
                    }
                    continue;
                }

                // If our chunk represents a CommentStart then drop us into comment mode and skip to
                // the next iteration
                if let Ok(token) = Token::from_chunk(chunk.clone()) {
                    if token.token_type == TokenType::CommentStart {
                        self.comment_start = Some(chunk);
                        continue;
                    }
                }
                return Some(Ok(chunk));
            }

            if next.is_none() && self.comment_start.is_some() {
                return Some(Err(Error::new(
                    "reached EOF without finding comment close".to_string(),
                    self.comment_start.clone().unwrap(),
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
