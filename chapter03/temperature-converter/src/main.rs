use std::io;

fn main() {
    let input_temperature = get_temperature();
    let input_symbol = get_symbol();

    println!("Input: {} °{}", input_temperature, input_symbol);

    let result: (f32, char) = match input_symbol {
        'c' | 'C' => (c2f(input_temperature), 'f'),
        'f' | 'F' => (f2c(input_temperature), 'c'),
        _ => panic!("The symbol you've entered isn't supported.")
    };

    println!("{} °{} is {} °{}", input_temperature, input_symbol, result.0, result.1);
}

fn f2c(val:f32) -> f32 {
    (val - 32.0) * (5.0 / 9.0)
}

fn c2f(val: f32) -> f32 {
    val * (9.0/5.0) + 32.0
}

fn get_temperature() -> f32 {
    println!("Enter the temperature: ");
    
    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature)
        .expect("Unable to read line");

    let temperature:f32 = temperature.trim().parse()
        .expect("Please enter a valid number");

    temperature
}

fn get_symbol() -> char {
    let mut symbol = String::new();

    loop {
        println!("Please enter the temperature symbol (c/f)");
        
        io::stdin().read_line(&mut symbol)
            .expect("Failed to read line");

        if symbol.len() >= 1 {
            break;
        }
    }

    match symbol.chars().next() {
        Some(c) => c,
        None => panic!("input should at least contain one char")
    }
}