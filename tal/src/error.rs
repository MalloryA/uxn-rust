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
        let mut arrows = "".to_string();
        for _ in 0..self.chunk.column {
            arrows.push(' ');
        }
        for _ in 0..self.chunk.value.len() {
            arrows.push('^');
        }
        format!("Error: {}\n\n{}\n{}\n", self.message, line, arrows)
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
        );
        let error_with_context = err.to_string_with_context(&mut reader);
        assert_eq!(
            error_with_context,
            "Error: Unknown token \"cat\"\n\nBAT cat\n    ^^^\n"
        );
    }
}
