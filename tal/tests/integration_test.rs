use std::env::current_exe;
use std::env::temp_dir;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs::read;
use std::fs::read_dir;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[derive(PartialEq)]
pub struct Rom {
    rom: Vec<u8>,
}

impl Rom {
    fn from_file(path: PathBuf) -> Result<Rom, String> {
        let mut rom = Rom { rom: vec![] };
        let contents = read(path);
        match contents {
            Err(err) => Err(err.to_string()),
            Ok(contents) => {
                for byte in contents.into_iter() {
                    rom.rom.push(byte);
                }
                Ok(rom)
            }
        }
    }

    fn get_bytes(&self) -> &[u8] {
        &self.rom
    }
}

impl Debug for Rom {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str("File contents:\n")?;
        for (i, byte) in self.get_bytes().iter().enumerate() {
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
            (Some(l), None) => {
                expect_eq(l.to_string(), "".to_string(), format!("failed at line {i}"))?;
            }
            (None, Some(r)) => {
                expect_eq("".to_string(), r.to_string(), format!("failed at line {i}"))?;
            }
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

fn relative(root: &Path, file: &Path) -> String {
    let len = root.display().to_string().len() + 1;
    file.display().to_string()[len..].to_string()
}

fn expect_successful_assembly(cwd: &PathBuf, tal: PathBuf, rom: PathBuf) -> Result<(), String> {
    println!("tal {} {}", relative(cwd, &tal), relative(cwd, &rom));
    let tmp = temp_dir().join("tal-test.rom");

    let result = Command::new(root_dir().join("target/debug/tal"))
        .arg(tal)
        .arg(tmp.clone())
        .current_dir(cwd)
        .status();
    expect(result.is_ok(), "Command failed".to_string())?;
    let status = result.unwrap();
    expect(status.success(), format!("exit code: {:?}", status.code()))?;

    expect_eq_files(tmp, rom)
}

fn expect_unsuccessful_assembly(cwd: &PathBuf, tal: PathBuf) -> Result<(), String> {
    println!("tal {}", relative(cwd, &tal));
    let tmp = temp_dir().join("tal-test.rom");

    let result = Command::new(root_dir().join("target/debug/tal"))
        .arg(tal)
        .arg(tmp.clone())
        .current_dir(cwd)
        .status();
    expect(result.is_ok(), "Command failed".to_string())?;
    let status = result.unwrap();
    expect(
        !status.success(),
        "got 0 exit code when expected failure".to_string(),
    )
}

fn root_dir() -> PathBuf {
    current_exe()
        .unwrap()
        .ancestors()
        .nth(4)
        .unwrap()
        .to_path_buf()
}

// Returns (tal_files_with_roms, tal_files_without_roms)
fn find_tal_files(path: &Path) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut directories = vec![path.to_path_buf()];
    let mut tal_files_with_roms = vec![];
    let mut tal_files_without_roms = vec![];

    while let Some(dir_path) = directories.pop() {
        let dir = read_dir(dir_path).unwrap();
        for result in dir {
            let file = result.unwrap();
            if file.metadata().unwrap().is_dir() {
                directories.push(file.path());
            } else if file.path().extension().unwrap().to_str().unwrap() == "tal" {
                let rom_path = file.path().with_extension("rom");
                if rom_path.exists() {
                    tal_files_with_roms.push(file.path());
                } else {
                    tal_files_without_roms.push(file.path());
                }
            }
        }
    }

    (tal_files_with_roms, tal_files_without_roms)
}

#[test]
fn it_works() {
    let path = root_dir().join("tal/tests/roms");
    let (tal_files_with_roms, tal_files_without_roms) = find_tal_files(&path);

    let mut results_expect_successful = vec![];
    let mut results_expect_unsuccessful = vec![];

    for tal_path in tal_files_with_roms {
        let relative_path = relative(&path, &tal_path);
        let rom_path = tal_path.with_extension("rom");
        let result = expect_successful_assembly(&path, tal_path, rom_path.clone());
        results_expect_successful.push((relative_path, result));
    }

    for tal_path in tal_files_without_roms {
        let relative_path = relative(&path, &tal_path);
        let result = expect_unsuccessful_assembly(&path, tal_path);
        results_expect_unsuccessful.push((relative_path, result));
    }

    let expected_succeeded = results_expect_successful
        .iter()
        .filter(|r| r.1.is_ok())
        .count();
    let total_expect_successful = results_expect_successful.len();
    let expected_unsucceeded = results_expect_unsuccessful
        .iter()
        .filter(|r| r.1.is_ok())
        .count();
    let total_expect_unsuccessful = results_expect_unsuccessful.len();

    let mut fail = false;
    for result in results_expect_successful {
        if result.1.is_ok() {
            println!("expecting success... got SUCCESS {}", result.0);
        } else {
            fail = true;
            println!(
                "expecting success... got FAIL    {} - {}",
                result.0,
                result.1.unwrap_err()
            );
        }
    }
    for result in results_expect_unsuccessful {
        if result.1.is_ok() {
            println!("expecting failure... got FAIL    {}", result.0);
        } else {
            fail = true;
            println!(
                "expecting failure... got SUCCESS {} - {}",
                result.0,
                result.1.unwrap_err()
            );
        }
    }

    println!("{expected_succeeded}/{total_expect_successful} expected to succeed succeeded");
    println!("{expected_unsucceeded}/{total_expect_unsuccessful} expected to fail failed");
    if fail {
        panic!("1 or more tests failed");
    }
}
