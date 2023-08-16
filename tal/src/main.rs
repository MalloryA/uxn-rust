mod chunker;
mod error;
mod opcode;
#[macro_use]
mod parser;
mod pre_process_brackets;
mod pre_process_comments;
mod pre_process_includes;
mod pre_process_macros;
mod token;

use crate::chunker::Chunk;
use crate::error::Error;
use crate::parser::chunk_file;
use crate::parser::parse_chunks;
use std::env::args;
use std::env::current_dir;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

fn read_and_write(
    cwd: &Path,
    file: PathBuf,
    writer: &mut dyn Write,
    chunker: Vec<Result<Chunk, Error>>,
) -> Result<(), Error> {
    match parse_chunks(cwd, file, &mut chunker.into_iter()) {
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
    let cwd = current_dir().unwrap();
    let input_path = args.next().unwrap();
    let input_path = Path::new(&input_path);
    let output_path = args.next().unwrap();

    let mut output = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(output_path)
        .unwrap();
    let chunks = chunk_file(&cwd, input_path);
    let result = read_and_write(&cwd, input_path.to_path_buf(), &mut output, chunks);
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
    use crate::chunker::Chunker;
    use crate::parser::pre_process;
    use std::io::Cursor;

    #[test]
    fn it_works() {
        let mut input = Cursor::new(b"|0100 LIT 68 LIT 18 DEO");
        let mut output = Cursor::<Vec<u8>>::new(vec![]);
        let expected: Vec<u8> = vec![0x80, 0x68, 0x80, 0x18, 0x17];

        let mut chunker = Chunker::new(&mut input);
        let chunks = pre_process(Path::new(""), PathBuf::new(), &mut chunker);
        let result = read_and_write(Path::new(""), PathBuf::new(), &mut output, chunks);
        assert!(result.is_ok());
        println!("{output:?}");
        let actual = output.into_inner();
        assert_eq!(actual, expected);
    }
}
