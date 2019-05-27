fn main() {
    println!("Hello, world!");

    another_function(5, 6);

    // definitions
    // statements := instructions that perform some action and
    //               *not* return a value
    // expressions := evaluate to a resulting value

    let y = {
        let x = 3;  // statement
        x + 1       // expression
    };

    println!("The value of y is: {}", y);

    // calling functions that return a value
    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("plus_one(5) = {}", x);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}