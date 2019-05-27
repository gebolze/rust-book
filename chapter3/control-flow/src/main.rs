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
}
