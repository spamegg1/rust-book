use std::fmt::Debug;
use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String; // required

    fn summarize(&self) -> String {
        // has a default impl, so not required
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
    // required
    fn summarize_author(&self) -> String {
        String::from("") // just dummy to compile
    }
    // override default
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
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
    // override default
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound syntax
pub fn notify1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple bounds
pub fn notify2(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

// trait syntax for multiple bounds
pub fn notify3<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// where clauses for multiple bounds
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    0
}

fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}

// return a type that implements a trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// We cannot return different types (that implement the same trait)
// in different conditional branches of the same function:
// fn returns_summarizable_bad(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best hockey team in the NHL."
//             ),
//         }
//     } else { // if and else have incompatible types!
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// Conditionally implement methods on generic type depending on trait bounds
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Conditionally implement a trait that implements another trait
// a.k.a "blanket implementation"
// impl<T: Display> ToString for T {
//     --snip--
// }

fn main() {
    // use aggregator::{Summary, Tweet};

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());
}
