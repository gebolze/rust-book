fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);


    // The following code won't compile because string3 doesn't live long
    // enough. Even if we as humans are able to determine that there won't
    // be a lifetime issue, because longest will return a reference to string1,
    // which lives long enough to be printed out.

    // let result;
    // {
    //     let string3 = String::from("a");
    //     result = longest(string1.as_str(), string3.as_str());
    // }

    // println!("The longest string is {}", result);
}

// The following function won't compile. 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}