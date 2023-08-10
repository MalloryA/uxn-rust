use crate::chunker::Chunk;
use crate::error::Error;
use crate::token::Token;
use crate::token::TokenType;
use std::collections::HashMap;
use std::path::PathBuf;

#[allow(clippy::enum_variant_names)]
enum MacroState {
    WaitingForName,
    WaitingForOpen(String),
    WaitingForClose(String),
}

pub struct PreProcessMacros<'a> {
    file: PathBuf,
    chunks: &'a mut dyn Iterator<Item = Result<Chunk, Error>>,
    macro_state: MacroState,
    macro_definitions: HashMap<String, Vec<Chunk>>,
    replacement: Vec<Chunk>,
}

impl PreProcessMacros<'_> {
    pub fn new(
        file: PathBuf,
        chunks: &mut dyn Iterator<Item = Result<Chunk, Error>>,
    ) -> PreProcessMacros {
        PreProcessMacros {
            file,
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
        loop {
            let next = if !self.replacement.is_empty() {
                Some(Ok(self.replacement.remove(0)))
            } else {
                self.chunks.next()
            };

            if let Some(Ok(chunk)) = next {
                if let Ok(token) = Token::from_chunk(chunk.clone()) {
                    match &self.macro_state {
                        MacroState::WaitingForName => {
                            if let TokenType::MacroDefinition(name) = token.token_type {
                                self.macro_state = MacroState::WaitingForOpen(name.clone());
                                continue;
                            } else if let TokenType::MacroOrInstant(name) = token.token_type {
                                if let Some(definition) = self.macro_definitions.get(&name) {
                                    // We found a macro definition for this name
                                    self.replacement = definition.to_vec();
                                    continue;
                                } else {
                                    // We didn't find a macro definition for this name
                                    // So treat it like it's an instant invocation and allow it to
                                    // pass through instead
                                }
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

            return next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunker::Chunker;
    use std::io::Cursor;

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
        let mut pp = PreProcessMacros::new(PathBuf::new(), &mut source);

        assert_eq!(
            pp.next(),
            Some(Ok(Chunk::new(String::from("#1234"), 0, 17)))
        );
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("#18"), 0, 8))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("DEO"), 0, 12))));
        assert_eq!(pp.next(), None);
    }

    #[test]
    fn macros_inside_macros_work() {
        let mut buffer =
            Cursor::new("%EMIT { #18 DEO } %TEST-SHORT { EQU2 #30 ADD EMIT } TEST-SHORT");
        let mut source = Chunker::new(&mut buffer);
        let mut pp = PreProcessMacros::new(PathBuf::new(), &mut source);

        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("EQU2"), 0, 32))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("#30"), 0, 37))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("ADD"), 0, 41))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("#18"), 0, 8))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("DEO"), 0, 12))));
        assert_eq!(pp.next(), None);
    }
}
