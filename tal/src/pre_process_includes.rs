use crate::chunker::Chunk;
use crate::error::Error;
use crate::parser::chunk_file;
use crate::token::Token;
use crate::token::TokenType;
use std::path::Path;
use std::path::PathBuf;

pub struct PreProcessIncludes<'a> {
    #[allow(dead_code)]
    file: PathBuf,
    cwd: &'a Path,
    chunks: &'a mut dyn Iterator<Item = Result<Chunk, Error>>,
    replacement: Vec<Result<Chunk, Error>>,
}

impl PreProcessIncludes<'_> {
    pub fn new<'a>(
        file: PathBuf,
        cwd: &'a Path,
        chunks: &'a mut dyn Iterator<Item = Result<Chunk, Error>>,
    ) -> PreProcessIncludes<'a> {
        PreProcessIncludes {
            file,
            cwd,
            chunks,
            replacement: vec![],
        }
    }
}

impl Iterator for PreProcessIncludes<'_> {
    type Item = Result<Chunk, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = if !self.replacement.is_empty() {
                Some(self.replacement.remove(0))
            } else {
                self.chunks.next()
            };

            if let Some(Ok(chunk)) = next {
                if let Ok(token) = Token::from_chunk(chunk.clone()) {
                    if let TokenType::Include(path) = token.token_type {
                        self.replacement = chunk_file(self.cwd, Path::new(&path));
                        continue;
                    }
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
    use std::env::current_dir;

    #[test]
    fn it_works() {
        let cwd = current_dir().unwrap().join("tests/roms");
        let mut source = vec![Ok(Chunk::new(String::from("~hello.tal"), 0, 0))].into_iter();
        let mut pp = PreProcessIncludes::new(PathBuf::new(), &cwd, &mut source);

        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("|0100"), 1, 0))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("LIT"), 1, 6))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("68"), 1, 10))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("LIT"), 1, 13))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("18"), 1, 17))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("DEO"), 1, 20))));
    }

    #[test]
    fn includes_inside_includes_work() {
        let cwd = current_dir().unwrap().join("tests/roms");
        let mut source = vec![Ok(Chunk::new(String::from("~hello-include.tal"), 0, 0))].into_iter();
        let mut pp = PreProcessIncludes::new(PathBuf::new(), &cwd, &mut source);

        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("|0100"), 1, 0))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("LIT"), 1, 6))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("68"), 1, 10))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("LIT"), 1, 13))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("18"), 1, 17))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("DEO"), 1, 20))));
    }
}
