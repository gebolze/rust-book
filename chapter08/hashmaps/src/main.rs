#![allow(unused_variables)]

// In order to use hash maps we've to bring it into scope
use std::collections::HashMap;

fn main() {
    // create a new hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    // all keys and values of a hash map must have the same type



    // constructing a hash map using collect
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];

    // scores needs type annotations, because `collect` can be used for multiple
    // datastructures though rust can't infer the type automatically.
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();



    // hash maps and ownership
    // types that implement the `Copy` trait (eg. i32) are copied into the hash
    // owned types (eg. String) are moved and the hashmap will take ownership.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // at this point field_name and field_value are invalid.
    // println!("{}", field_name);
    // println!("{}", field_value);



    // accessing values in a hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);

    // iterating over a hash map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }



    // updating a hash map

    // overwriting a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // this will replace 10 with 25
    println!("Score: {:?}", scores);

    // only insert if the key has no value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("Score: {:?}", scores);

    // updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
