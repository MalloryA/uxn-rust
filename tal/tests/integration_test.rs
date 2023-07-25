use std::env::current_exe;
use std::env::temp_dir;
use std::fs::read;
use std::fs::read_dir;
use std::path::PathBuf;
use std::process::Command;

fn assert_eq_files(left: PathBuf, right: PathBuf) {
    let left = read(left);
    assert!(left.is_ok());
    let left = left.unwrap();

    let right = read(right);
    assert!(right.is_ok());
    let right = right.unwrap();

    assert_eq!(left, right);
}

fn test_file(tal: PathBuf, rom: PathBuf) {
    println!("{:?} -> {:?}", tal, rom);
    let tmp = temp_dir().join("tal-test.rom");

    let result = Command::new(root_dir().join("target/debug/tal"))
        .arg(tal)
        .arg(tmp.clone())
        .status();
    assert!(result.is_ok());
    let status = result.unwrap();
    assert!(status.success(), "exit code: {:?}", status.code());

    assert_eq_files(tmp, rom);
}

fn root_dir() -> PathBuf {
    current_exe()
        .unwrap()
        .ancestors()
        .nth(4)
        .unwrap()
        .to_path_buf()
}

#[test]
fn it_works() {
    let path = root_dir().join("tal/tests/roms");
    let dir = read_dir(path).unwrap();
    for result in dir {
        let file = result.unwrap();
        let path = file.path();
        if path.extension().unwrap().to_str().unwrap() != "tal" {
            continue;
        }
        let rom = path.with_extension("rom");
        test_file(path, rom);
    }
}
