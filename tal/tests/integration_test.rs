use std::env::current_exe;
use std::env::temp_dir;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs::read;
use std::fs::read_dir;
use std::path::PathBuf;
use std::process::Command;

#[derive(PartialEq)]
pub struct Rom {
    rom: [u8; 0xff00],
}

impl Rom {
    fn from_file(path: PathBuf) -> Result<Rom, String> {
        let mut rom = Rom { rom: [0; 0xff00] };
        let contents = read(path);
        match contents {
            Err(err) => Err(err.to_string()),
            Ok(contents) => {
                for (i, byte) in contents.into_iter().enumerate() {
                    rom.rom[i] = byte;
                }
                Ok(rom)
            }
        }
    }

    fn get_bytes(&self) -> &[u8] {
        let mut last_non_null: Option<usize> = None;
        let iter = self.rom.iter();
        for (i, byte) in iter.enumerate() {
            if *byte != 0x00 {
                last_non_null = Some(i);
            }
        }
        match last_non_null {
            None => &[],
            Some(size) => &self.rom[0..size + 1],
        }
    }
}

impl Debug for Rom {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("File contents:\n")?;
        for (i, byte) in self.get_bytes().into_iter().enumerate() {
            if i != 0 {
                if i % 16 == 0 {
                    f.write_str("\n")?;
                } else if i % 2 == 0 {
                    f.write_str(" ")?;
                }
            }
            f.write_fmt(format_args!("{:02x}", byte))?;
        }
        Ok(())
    }
}

fn expect(value: bool, message: String) -> Result<(), String> {
    if value {
        Ok(())
    } else {
        Err(format!("Expected {:?} to be true - {}", value, message))
    }
}

fn expect_eq(left: String, right: String, message: String) -> Result<(), String> {
    if left == right {
        Ok(())
    } else {
        Err(format!(
            "Expected {:?} to equal {:?} - {}",
            left, right, message
        ))
    }
}

fn expect_eq_rom(left: Rom, right: Rom) -> Result<(), String> {
    let left = format!("{:?}", left);
    let mut left = left.lines();
    let right = format!("{:?}", right);
    let mut right = right.lines();

    let mut i = 0;
    loop {
        i += 1;
        let line_left = left.next();
        let line_right = right.next();

        match (line_left, line_right) {
            (None, None) => {
                break;
            }
            (Some(l), Some(r)) => {
                expect_eq(l.to_string(), r.to_string(), format!("failed at line {i}"))?;
            }
            _ => todo!(),
        }
    }
    Ok(())
}

fn expect_eq_files(left: PathBuf, right: PathBuf) -> Result<(), String> {
    let _left = Rom::from_file(left.clone());
    assert!(_left.is_ok(), "{left:?} should be OK");
    let _left = _left.unwrap();

    let _right = Rom::from_file(right.clone());
    assert!(_right.is_ok(), "{right:?} should be OK");
    let _right = _right.unwrap();

    expect_eq_rom(_left, _right)
}

fn relative(root: &PathBuf, file: &PathBuf) -> String {
    let len = root.display().to_string().len() + 1;
    (&file.display().to_string()[len..]).to_string()
}

fn expect_successful_assembly(cwd: &PathBuf, tal: PathBuf, rom: PathBuf) -> Result<(), String> {
    println!("tal {} {}", relative(&cwd, &tal), relative(&cwd, &rom));
    let tmp = temp_dir().join("tal-test.rom");

    let result = Command::new(root_dir().join("target/debug/tal"))
        .arg(tal)
        .arg(tmp.clone())
        .current_dir(cwd)
        .status();
    expect(result.is_ok(), format!("Command failed"))?;
    let status = result.unwrap();
    expect(status.success(), format!("exit code: {:?}", status.code()))?;

    expect_eq_files(tmp, rom)
}

fn root_dir() -> PathBuf {
    current_exe()
        .unwrap()
        .ancestors()
        .nth(4)
        .unwrap()
        .to_path_buf()
}

fn find_all_rom_files(path: &PathBuf) -> Vec<PathBuf> {
    let mut directories = vec![path.clone()];
    let mut files = vec![];

    while !directories.is_empty() {
        let dir_path = directories.pop().unwrap();

        let dir = read_dir(dir_path).unwrap();
        for result in dir {
            let file = result.unwrap();
            if file.metadata().unwrap().is_dir() {
                directories.push(file.path());
            } else if file.path().extension().unwrap().to_str().unwrap() == "rom" {
                files.push(file.path());
            }
        }
    }

    files
}

#[test]
fn it_works() {
    let path = root_dir().join("tal/tests/roms");
    let roms = find_all_rom_files(&path);

    let mut results = vec![];

    for rom_path in roms {
        let relative_path = relative(&path, &rom_path);
        let tal_path = rom_path.with_extension("tal");
        let result = expect_successful_assembly(&path, tal_path, rom_path.clone());
        results.push((relative_path, result));
    }

    let succeeded = results.iter().filter(|r| r.1.is_ok()).count();
    let total = results.len();

    let mut fail = false;
    for result in results {
        if result.1.is_ok() {
            println!("SUCCESS {}", result.0);
        } else {
            fail = true;
            println!("FAIL    {} - {}", result.0, result.1.unwrap_err());
        }
    }
    println!("{succeeded}/{total} succeeded");
    if fail {
        panic!("1 or more tests failed");
    }
}
