use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username = String::new();
    // error is returned early to whole function from ?,
    // otherwise Ok's value is returned for the expression
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// this way we can use ? inside main
// main can return any type that implements std::process::Termination trait
fn main2() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

fn main() {
    // let greeting_file = File::open("hello.txt")?; // main does not return Result
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file {:?}", e),
            },
            e => panic!("Problem opening file {:?}", e),
        },
    };

    let greeting_file_result_2 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|e| {
                panic!("Problem creating the file: {:?}", e);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let greeting_file_2 = File::open("hello.txt").unwrap();

    let greeting_file_3 =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}
