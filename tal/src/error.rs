use crate::chunker::Chunk;
use std::io::BufRead;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Error {
    message: String,
    chunk: Chunk,
    file: PathBuf,
}

impl Error {
    pub fn new(message: String, chunk: Chunk, file: PathBuf) -> Error {
        Error {
            message,
            chunk,
            file,
        }
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

        format!(
            "{}:{}: Error: {}\n\n{}\n{}",
            self.file.display(),
            self.chunk.line + 1,
            self.message,
            line,
            arrows
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn it_works() {
        let mut reader = Cursor::new("FOO\nBAR\nBAZ\nBAT cat");
        let err = Error::new(
            "Unknown token \"cat\"".to_string(),
            Chunk::new("cat".to_string(), 3, 4),
            PathBuf::from("foo.tal"),
        );
        let error_with_context = err.to_string_with_context(&mut reader);
        let expected = "foo.tal:4: Error: Unknown token \"cat\"\n\nBAT cat\n    ^^^";
        assert_eq!(error_with_context, expected);
    }

    #[test]
    fn it_handles_tabs() {
        let err = Error {
            message: "could not parse AddressLiteralAbsoluteByte".to_string(),
            chunk: Chunk::new(".octave".to_string(), 108, 32),
            file: PathBuf::from("foo.tal"),
        };

        let mut reader = Cursor::new(
            "\n".repeat(108)
                + "\t[ LIT \"a ] NEQk NIP ?&no-c #30 .octave LDZ #0c MUL ADD play &no-c\n",
        );
        let error_with_context = err.to_string_with_context(&mut reader);
        let expected = "foo.tal:109: Error: could not parse AddressLiteralAbsoluteByte\n\n        [ LIT \"a ] NEQk NIP ?&no-c #30 .octave LDZ #0c MUL ADD play &no-c\n                                       ^^^^^^^";
        assert_eq!(error_with_context, expected);
    }

    #[test]
    fn it_handles_two_tabs() {
        let err = Error {
            message: "could not parse AddressLiteralAbsoluteByte".to_string(),
            chunk: Chunk::new(".center/x".to_string(), 31, 7),
            file: PathBuf::from("foo.tal"),
        };

        let mut reader = Cursor::new("\n".repeat(31) + "\t\tDUP2 .center/x STZ2");
        let error_with_context = err.to_string_with_context(&mut reader);
        let expected = "foo.tal:32: Error: could not parse AddressLiteralAbsoluteByte\n\n                DUP2 .center/x STZ2\n                     ^^^^^^^^^";
        assert_eq!(error_with_context, expected);
    }
}
