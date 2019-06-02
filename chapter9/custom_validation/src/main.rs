use std::io;
use std::cmp::Ordering;

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

        if (guess < 1 || guess > 100) {
            println!("The secret number will be between 1 and 100.0");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Your guess is too high"),
            Ordering::Less => println!("Your guess it to low"),
            Ordering::Equal => break,
        };
    }

    println!("You guessed the secret number.");
}
