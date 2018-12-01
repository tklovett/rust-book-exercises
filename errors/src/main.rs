use std::fs::File;
use std::io::ErrorKind;
use std::io::Error;

fn main() {
    println!("Hello, world!");

    let f = File::open("hello.txt");
    if let Err(error) = f {
        println!("There was a problem opening the file: {:?}", error)
    }

    let _: File = open_or_create("hello.txt");
}

fn open_or_create(path: &str) -> File {
    let file = File::open(path);
    match file {
        Ok(file) => file,
        Err(error) => create_if_not_exists(path, &error),
    }
}

fn create_if_not_exists(path: &str, error: &Error) -> File {
    match error.kind() {
        ErrorKind::NotFound => create_or_panic(path),
        other_error => panic!("There was a problem opening the file: {:?}", other_error),
    }
}

fn create_or_panic(path: &str) -> File {
    match File::create(path) {
        Ok(file) => file,
        Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
    }
}



fn ignored() {
    let f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}