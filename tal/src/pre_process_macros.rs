use crate::chunker::Chunk;
use crate::error::Error;
use crate::opcode::Opcode;
use crate::pre_process_comments::PreProcessComments;
use crate::token::Token;
use crate::token::TokenType;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::io::BufRead;
use std::io::Bytes;
use std::io::Read;
use std::slice::Iter;

enum MacroState {
    WaitingForName,
    WaitingForOpen(String),
    WaitingForClose(String),
}

pub struct PreProcessMacros<'a> {
    chunks: &'a mut dyn Iterator<Item = Result<Chunk, Error>>,
    macro_state: MacroState,
    macro_definitions: HashMap<String, Vec<Chunk>>,
    replacement: Vec<Chunk>,
}

impl PreProcessMacros<'_> {
    pub fn new(chunks: &mut dyn Iterator<Item = Result<Chunk, Error>>) -> PreProcessMacros {
        PreProcessMacros {
            chunks,
            macro_state: MacroState::WaitingForName,
            macro_definitions: HashMap::new(),
            replacement: vec![],
        }
    }
}

impl Iterator for PreProcessMacros<'_> {
    type Item = Result<Chunk, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.replacement.is_empty() {
            return Some(Ok(self.replacement.remove(0)));
        }

        loop {
            let next = self.chunks.next();

            if let Some(Ok(chunk)) = next {
                if let Ok(token) = Token::from_chunk(chunk.clone()) {
                    match &self.macro_state {
                        MacroState::WaitingForName => {
                            if let TokenType::MacroDefinition(name) = token.token_type {
                                self.macro_state = MacroState::WaitingForOpen(name.clone());
                                continue;
                            } else if let TokenType::MacroOrInstant(name) = token.token_type {
                                self.replacement =
                                    self.macro_definitions.get(&name).unwrap().to_vec();
                                continue;
                            }
                        }
                        MacroState::WaitingForOpen(name) => {
                            if token.token_type == TokenType::MacroOpen {
                                self.macro_definitions.insert(name.clone(), vec![]);
                                self.macro_state = MacroState::WaitingForClose(name.clone());
                                continue;
                            }
                        }
                        MacroState::WaitingForClose(name) => {
                            if token.token_type == TokenType::MacroClose {
                                self.macro_state = MacroState::WaitingForName;
                                continue;
                            } else {
                                let mut current_macro = vec![];
                                for chunk in self.macro_definitions.get(name).unwrap() {
                                    current_macro.push(chunk.clone());
                                }
                                current_macro.push(chunk.clone());
                                self.macro_definitions.insert(name.clone(), current_macro);
                                continue;
                            }
                        }
                    }
                }

                return Some(Ok(chunk));
            }

            // if next.is_none() && self.comment_start.is_some() {
            //     return Some(Err(Error::new(
            //         "reached EOF without finding comment close".to_string(),
            //         self.comment_start.clone().unwrap(),
            //     )));
            // }

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
            Ok(Chunk::new(String::from("%EMIT"), 0, 0)),
            Ok(Chunk::new(String::from("{"), 0, 6)),
            Ok(Chunk::new(String::from("#18"), 0, 8)),
            Ok(Chunk::new(String::from("DEO"), 0, 12)),
            Ok(Chunk::new(String::from("}"), 0, 15)),
            Ok(Chunk::new(String::from("#1234"), 0, 17)),
            Ok(Chunk::new(String::from("EMIT"), 0, 23)),
        ]
        .into_iter();
        let mut pp = PreProcessMacros::new(&mut source);

        assert_eq!(
            pp.next(),
            Some(Ok(Chunk::new(String::from("#1234"), 0, 17)))
        );
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("#18"), 0, 8))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("DEO"), 0, 12))));
        assert_eq!(pp.next(), None);
    }
}
