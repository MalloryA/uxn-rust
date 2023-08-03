mod chunker;
mod error;
mod opcode;
mod parser;
mod pre_process_comments;
mod token;

use crate::chunker::ChunkResulter;
use crate::chunker::Chunker;
use crate::error::Error;
use crate::parser::parse;
use crate::pre_process_comments::PreProcessComments;
use std::env::args;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::process::exit;

fn read_and_write(writer: &mut dyn Write, reader: &mut dyn BufRead) -> Result<(), Error> {
    let mut chunker = Chunker::new(reader);
    let mut chunk_resulter = ChunkResulter::new(&mut chunker);
    let mut pp = PreProcessComments::new(&mut chunk_resulter);
    match parse(&mut pp) {
        Ok(rom) => match writer.write_all(rom.get_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => panic!("{:?}", err),
        },
        Err(err) => Err(err),
    }
}

fn main() {
    let mut args = args();
    let program = args.next().unwrap();

    if args.len() != 2 {
        println!("Usage: {} input.tal output.rom", program);
        exit(1);
    }
    let input_path = args.next().unwrap();
    let output_path = args.next().unwrap();

    let mut input = BufReader::new(File::open(input_path.clone()).unwrap());
    let mut output = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(output_path)
        .unwrap();
    let result = read_and_write(&mut output, &mut input);
    match result {
        Ok(_) => println!("OK"),
        Err(err) => {
            let mut input = BufReader::new(File::open(input_path).unwrap());
            println!("{:?}", err);
            println!("{}", err.to_string_with_context(&mut input));
            exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn it_works() {
        let mut input = Cursor::new(b"|0100 LIT 68 LIT 18 DEO");
        let mut output = Cursor::<Vec<u8>>::new(vec![]);
        let expected: Vec<u8> = vec![0x80, 0x68, 0x80, 0x18, 0x17];

        let result = read_and_write(&mut output, &mut input);
        assert!(result.is_ok());
        println!("{output:?}");
        let actual = output.into_inner();
        assert_eq!(actual, expected);
    }
}
