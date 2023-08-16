use crate::chunker::Chunk;
use crate::error::Error;

fn ignore_chunk(chunk: &Chunk) -> bool {
    &chunk.value[..] == "[" || &chunk.value[..] == "]"
}

pub struct PreProcessBrackets<'a> {
    chunks: &'a mut dyn Iterator<Item = Result<Chunk, Error>>,
    replacement: Vec<Result<Chunk, Error>>,
}

impl PreProcessBrackets<'_> {
    pub fn new(chunks: &mut dyn Iterator<Item = Result<Chunk, Error>>) -> PreProcessBrackets {
        PreProcessBrackets {
            chunks,
            replacement: vec![],
        }
    }
}

impl Iterator for PreProcessBrackets<'_> {
    type Item = Result<Chunk, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = if !self.replacement.is_empty() {
                Some(self.replacement.remove(0))
            } else {
                self.chunks.next()
            };

            if let Some(Ok(chunk)) = next {
                if ignore_chunk(&chunk) {
                    continue;
                }

                return Some(Ok(chunk));
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
            Ok(Chunk::new(String::from("~hello.tal"), 0, 0)),
            Ok(Chunk::new(String::from("["), 0, 0)),
            Ok(Chunk::new(String::from("]"), 0, 0)),
        ]
        .into_iter();
        let mut pp = PreProcessBrackets::new(&mut source);

        assert_eq!(
            pp.next(),
            Some(Ok(Chunk::new(String::from("~hello.tal"), 0, 0)))
        );
        assert_eq!(pp.next(), None);
    }
}
