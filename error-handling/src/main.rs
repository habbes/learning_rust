use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn main() {
    let greeting_file_result = File::open("hello.text");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => File::create("hello.text").unwrap_or_else(|e| {
                panic!("Problem creating the file: {:?}", e);
            }),
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        }
    };

    // File::open("hello.text").expect("hello.text should be included in this project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    // use ? as shortcut to return Err Result if call fails,
    // saves us from using match Ok, Err for this common scenario
    // this works if the error returned by the function call
    // is compatible with the error type in this function's return type
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}