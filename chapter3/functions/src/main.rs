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
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}