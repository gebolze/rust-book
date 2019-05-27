fn main() {
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
