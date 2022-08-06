use std::fs::File;
use std::io::{Write, Error, ErrorKind, Read};


fn main() {
    let mut f = File::open("hello.txt")
        .unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap()
            } else {
                panic!("Ошибка {:?}", error)
            }
        });
    f.write("Hello world!".as_bytes()).expect("Write problem!");

    let username: String = read_username_from_file("user.txt").unwrap();
    println!("Hello, {}", username);
}


fn read_username_from_file(file: &str) -> Result<String, Error> {
    let mut f = File::open(file)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}



