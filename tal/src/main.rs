mod chunker;
mod opcode;
mod parser;
mod token;

use crate::chunker::Chunker;
use crate::parser::parse;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

fn read_and_write(writer: &mut dyn Write, reader: &mut dyn BufRead) -> Result<(), String> {
    let mut chunker = Chunker::new(reader);
    match parse(&mut chunker) {
        Ok(rom) => match writer.write_all(&rom) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        },
        Err(err) => Err(err),
    }
}

fn main() {
    let mut input = BufReader::new(File::open("hello.tal").unwrap());
    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .open("hello.rom")
        .unwrap();
    let result = read_and_write(&mut output, &mut input);
    match result {
        Ok(_) => println!("OK"),
        Err(err) => println!("Error!!! {err:?}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn it_works() {
        //        let mut input = Cursor::new("|0100 LIT 68 LIT 18 DEO");
        //        let mut output: Cursor<[u8]> = Cursor<&mut [u8]>::new();
        //        let mut expected = "x80x68x80x18x17";
        //
        //        let result = read_and_write(&mut output, &mut input);
        //        assert!(result.is_ok());
        //        assert_eq!(output, expected);
    }
}
