#![allow(unused_variables, unused_mut)]

fn main() {
    // creating a new vector

    // create an empty vector
    let empty_vec: Vec<i32> = Vec::new();

    // create a vector with initial elements
    let inital_vec = vec![1, 2, 3, 4, 5];



    // updating a vector

    // add elements to a vector
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);



    // droping a vector drops its elements

    {
        let scoped_vector = vec![1, 2, 3];
    } // scoped_vector and all its elements will be freed here!



    // reading elements of vectors

    // accessing elements of a vector using a index value
    // when using an out-of-bound index rust will panic!
    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {}", third);


    // accessing elements with get
    // when using an out-of-bound index a None will be returned.
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // IMPORTANT: You aren't allowed to modify the vector if its already
    // borrowed immutable
    let mut some_vector = vec![1, 2, 3, 4, 5];
    let first = &some_vector[0]; // borrow first element
    //some_vector.push(6); // <- uncommenting this will case compile-time error
    println!("The first element is: {}", first);



    // iterating over the values in a vector

    // for loop to get immutable references
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // for loop using mutable references
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
