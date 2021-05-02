use std::fs;
use std::process::exit;
use std::io::Read;

pub fn read_stdin() -> String {
    let mut result: String = String::new();
    match std::io::stdin().read_to_string(&mut result) {
        Ok(_) => {}
        Err(_) => {
            println!("Unable to read contents from stdin");
            exit(1);
        }
    };
    result
}

pub fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(content) => {content}
        Err(_) => {
            println!("Unable to read contents of file {}", path);
            exit(1);
        }
    }
}