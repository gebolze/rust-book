fn main() {
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
