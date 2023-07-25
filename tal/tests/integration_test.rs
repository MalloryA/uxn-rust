use std::env::current_exe;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::PathBuf;

fn test_file(tal: PathBuf, rom: PathBuf) {
    println!("{:?} -> {:?}", tal, rom);
}

#[test]
fn it_works() {
    let path = current_exe().unwrap();
    let path = path.ancestors().nth(4).unwrap().join("tal/tests/roms");
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
    assert!(false);
}
