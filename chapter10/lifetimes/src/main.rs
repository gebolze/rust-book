#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // No lifetime annotations are required here, because returned reference
    // has the same lifetime as self.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

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


    // Lifetime Annotations and Struct Definitions
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("Important excerpt: {:?} (level: {})", i, i.level());
    let part = i.announce_and_return_part("Look what we've found");
    println!("part: {}", part);


    // the static lifetime
    // static lifetime refers to references that are directly embedded in the
    // executable and are always available. All string literals have a static
    // lifetime    
    let s: &'static str = "I have a static lifetime";
    println!("static str: {}", s);

    // NOTE: Becareful when using the static lifetime, in most cases you
    // shouldn't need to use them!
}

// The following function won't compile. 
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
