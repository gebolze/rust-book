use std::fs::File;
use std::io::ErrorKind;

fn main() {
    panic_on_error();
    error_handling_with_match();
    error_handling_with_closures();

    shortcut_with_unwrap();
    shortcut_with_expect();

    propagating_errors();
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

fn shortcut_with_unwrap() {
    // unwrap() will panic on Err variant of Result, for Ok variant it will
    // return the "wrapped" value.
    let f = File::open("hello.txt").unwrap();
}

fn shortcut_with_expect() {
    // Similar to unwrap expect will panic on Err. It additionally allows you
    // to specify a custom error message used for the panic.
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn propagating_errors() {
    use std::io;
    use std::io::Read;

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();
        
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    let username = read_username_from_file().expect("Failed to read the username");
    println!("Username: {}", username);
}