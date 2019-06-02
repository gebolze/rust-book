use std::fs::File;
use std::io::ErrorKind;

fn main() {
    panic_on_error();
    error_handling_with_match();
    error_handling_with_closures();
}

fn panic_on_error() {
    // open may return an error result, when the file doesn't exist
    let f: Result<std::fs::File, std::io::Error> = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };
}

fn error_handling_with_match() {
    // open may return an error result, when the file doesn't exist
    let f: Result<std::fs::File, std::io::Error> = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
}

fn error_handling_with_closures() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}