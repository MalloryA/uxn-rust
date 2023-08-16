use crate::chunker::Chunk;
use crate::error::Error;
use std::collections::HashMap;

#[derive(PartialEq)]
enum MacroToken {
    MacroDefinition(String),
    MacroStart,
    MacroEnd,
    Other,
}

impl MacroToken {
    fn from_chunk(chunk: &Chunk) -> MacroToken {
        match &chunk.value[0..1] {
            "%" => MacroToken::MacroDefinition(chunk.value[1..].to_string()),
            "{" => MacroToken::MacroStart,
            "}" => MacroToken::MacroEnd,
            _ => MacroToken::Other,
        }
    }
}

#[allow(clippy::enum_variant_names)]
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
        loop {
            let next = if !self.replacement.is_empty() {
                Some(Ok(self.replacement.remove(0)))
            } else {
                self.chunks.next()
            };

            if let Some(Ok(chunk)) = next {
                let token = MacroToken::from_chunk(&chunk);
                match &self.macro_state {
                    MacroState::WaitingForName => {
                        if let MacroToken::MacroDefinition(name) = token {
                            self.macro_state = MacroState::WaitingForOpen(name.clone());
                            continue;
                        } else if token == MacroToken::Other {
                            if let Some(definition) = self.macro_definitions.get(&chunk.value) {
                                // We found a macro definition for this name
                                // So prepend the definition to the current replacement vec
                                self.replacement.splice(0..0, definition.to_vec());
                                continue;
                            } else {
                                // We didn't find a macro definition for this name
                                // So treat it like it's an instant invocation and allow it to
                                // pass through instead
                            }
                        }
                    }
                    MacroState::WaitingForOpen(name) => {
                        if token == MacroToken::MacroStart {
                            self.macro_definitions.insert(name.clone(), vec![]);
                            self.macro_state = MacroState::WaitingForClose(name.clone());
                            continue;
                        }
                    }
                    MacroState::WaitingForClose(name) => {
                        if token == MacroToken::MacroEnd {
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
        let mut pp = PreProcessMacros::new(&mut source);

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
        let mut pp = PreProcessMacros::new(&mut source);

        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("EQU2"), 0, 32))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("#30"), 0, 37))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("ADD"), 0, 41))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("#18"), 0, 8))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("DEO"), 0, 12))));
        assert_eq!(pp.next(), None);
    }

    #[test]
    fn macros_inside_macros_work2() {
        let mut buffer = Cursor::new("%FOO { 13 } %BAR { FOO FOO } BAR");
        let mut source = Chunker::new(&mut buffer);
        let mut pp = PreProcessMacros::new(&mut source);

        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("13"), 0, 7))));
        assert_eq!(pp.next(), Some(Ok(Chunk::new(String::from("13"), 0, 7))));
        assert_eq!(pp.next(), None);
    }
}
