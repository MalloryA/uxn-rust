use crate::chunker::Chunk;
use std::io::BufRead;

#[derive(Debug)]
pub struct Error {
    message: String,
    chunk: Chunk,
}

impl Error {
    pub fn new(message: String, chunk: Chunk) -> Error {
        Error { message, chunk }
    }

    pub fn to_string_with_context(&self, reader: &mut dyn BufRead) -> String {
        let line = reader.lines().nth(self.chunk.line);
        let line = line.unwrap();
        let line = line.unwrap();

        let tab_count = line.matches('\t').count();

        // Only add 7 for each tab, because each character in the string gets 1 added later on
        let mut arrows = " ".to_string().repeat(7).repeat(tab_count);
        for _ in 0..self.chunk.column {
            arrows.push(' ');
        }
        for _ in 0..self.chunk.value.len() {
            arrows.push('^');
        }

        let line = line.replace('\t', &" ".repeat(8));

        format!("Error: {}\n\n{}\n{}", self.message, line, arrows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::Cursor;

    #[test]
    fn it_works() {
        let mut reader = Cursor::new("FOO\nBAR\nBAZ\nBAT cat");
        let err = Error::new(
            "Unknown token \"cat\"".to_string(),
            Chunk::new("cat".to_string(), 3, 4),
        );
        let error_with_context = err.to_string_with_context(&mut reader);
        let expected = "Error: Unknown token \"cat\"\n\nBAT cat\n    ^^^";
        assert_eq!(error_with_context, expected,);
    }

    #[test]
    fn it_handles_tabs() {
        let err = Error {
            message: "could not parse AddressLiteralZeroPage".to_string(),
            chunk: Chunk::new(".octave".to_string(), 108, 32),
        };

        let mut reader = BufReader::new(File::open("tests/roms/piano.tal").unwrap());
        let error_with_context = err.to_string_with_context(&mut reader);
        let expected = "Error: could not parse AddressLiteralZeroPage\n\n        [ LIT \"a ] NEQk NIP ?&no-c #30 .octave LDZ #0c MUL ADD play &no-c\n                                       ^^^^^^^";
        assert_eq!(error_with_context, expected);
    }

    #[test]
    fn it_handles_two_tabs() {
        let err = Error {
            message: "could not parse AddressLiteralZeroPage".to_string(),
            chunk: Chunk::new(".center/x".to_string(), 31, 7),
        };

        let mut reader = BufReader::new(File::open("tests/roms/piano.tal").unwrap());
        let error_with_context = err.to_string_with_context(&mut reader);
        let expected = "Error: could not parse AddressLiteralZeroPage\n\n                DUP2 .center/x STZ2\n                     ^^^^^^^^^";
        assert_eq!(error_with_context, expected);
    }
}
