use std::io;
use std::cmp::Ordering;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {

    let secret_number = 57;

    loop {
        println!("Enter your guess (1-100)");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("failed to read from stdin");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let guess = Guess::new(guess);

        match guess.value().cmp(&secret_number) {
            Ordering::Greater => println!("Your guess is too high"),
            Ordering::Less => println!("Your guess it to low"),
            Ordering::Equal => break,
        };
    }

    println!("You guessed the secret number.");
}
