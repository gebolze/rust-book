use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // this is the default implementation which can be overridden when
        // implementing the trait for a custom type.
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        // NOTE: You can't call the default implementation, when you provide an
        // overridden implementation for a trait function.
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// as a more expressive alternative we can use the 'trait bounds' syntax
// instead of the 'impl trait' syntax. With trait bounds we can enforce that
// multiple parameters have the same type.
pub fn notify_bounds<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_display(item: &(impl Summary+Display)) {
    // item's type must implement the Summary and Display trait
}

pub fn notify_display_bounds<T: Summary + Display>(item: &T) {
    // item's type must implement the Summary and Display trait
}

// When using multiple trait bounds can make the function signature hard to
// read. For this reason there exist an alternative where syntax
pub fn some_function<T, U>(t: T, u:U) -> i32 
    where T: Display + Clone,
          U: Clone + Debug
{
    0
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Champoinsship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("1 tweet: {}", tweet.summarize());

    // this call to summarize() will use the default implementation
    println!("New article available! {}", article.summarize());

    // You can implement function that use a trait as parameters, these
    // functions then can be called with all Types that implement the specified
    // trait
    notify(&tweet);
    notify(&article);

    notify_bounds(&tweet);
    notify_bounds(&article);
}
