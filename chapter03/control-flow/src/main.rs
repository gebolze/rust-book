fn main() {
    // if expressions
    if_expressions();
    multiple_else_if_expressions();
    if_in_let_statements();

    // loops
    loop_loops();
    while_loops();
    for_loops();
}

fn if_expressions() {
    // if expressions

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false")
    }

    // the type of if-condition must be bool, there are no truthy or falsy
    // values in rust. (There is no automatic conversion to true/false).
    // so the following code won't compile.

    // if number {
    //     println!("number was three")
    // }
}

fn multiple_else_if_expressions() {
    // multiple else if expressions
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2")
    }
}

fn if_in_let_statements() {
    //  if in let statement
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    // NOTE: when using an if in a let statement both branches need to evaluate
    // to a value of the same type, otherwise you'll get a compile-time error.
}

fn loop_loops() {
    let mut x = 1;
    loop {
        println!("again!");
        if x >= 5 {
            break;
        }
        x = x + 1;
    }

    // returning a value from a loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_loops() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loops() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // for loops with a range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}