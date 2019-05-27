fn main() {
    scalar_types();

    tuple();
}

fn scalar_types() {
    // integer types
    let a: i32 = 8i32;
    
    println!("The value of a is: {}", a);

    // supported integer types:
    // i8   u8   - 8-bit signed / unsigned ints
    // i16  u16  - 16-bit signed / unsigned ints
    // i32  u32  - 32-bit signed / unsigned ints
    // i64  u64  - 64-bit signed / unsigned ints
    // i128 u128 - 128-bit signed / unsigned ints
    // default type for integers is i32
    // isize usize - arch dependant (usally used for indices)



    // floating-point types

    let b: f32 = 3.14;

    // supported floating-point types
    // f32 - 32-bit IEEE-754 floating-point number
    // f64 - 64-bit IEEE-754 floating-point number
    // default type for floats is f64 

    println!("The value of b is: {}", b);



    // boolean types

    let c = true;

    // a boolean can either be true or false
    
    println!("The value of c is: {}", c);



    // character types

    let d = 'z';

    // char type of 4-byte unicode scalar value.
    // therefore it can represent a lot more than just ASCII.

    println!("The value of d is: {}", d);
}

fn tuple() {
    // tuples writen as a comma-separated list surrounded by parentheses
    // the type annotation is optional
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // destructuring using pattern matching
    let (x,y,z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // destructuring using indices
    println!("The first value is: {}", tup.0);
    println!("The seconds value is: {}", tup.1);
    println!("The third value is: {}", tup.2);
}