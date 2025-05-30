use std::fmt::Display;

// lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Compiles, because we don't care how long y lives
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// fn longest3<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str() // result goes out of scope with the function body
// }

// Lifetime annotations in structs
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 { // no annotation to self, due to elision rule 1
        3
    }

    fn announce_and_return(&self, announcement: &str) -> &str {
        // no annotations to either parameter, due to elision rule 3
        println!("Attention please, {announcement}");
        self.part // gets the same lifetime as self
    }
}

// Lifetime elision: this works without lifetime annotations
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Generics, lifetimes, trait bounds all together
fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str where T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x; // error, x does not live long enough
    // }
    // println!("r: {r}");

    // Corrected version
    let x = 5;
    let r = &x;
    println!("r: {r}");

    // generic lifetimes in functions
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result1 = longest(string1.as_str(), string2);
    println!("The longest string is {result1}");

    let string3 = String::from("long string is long");
    {
        let string4 = String::from("xyz");
        let result2 = longest(string1.as_str(), string4.as_str());
        println!("The longest string is {result2}"); // works here
    }
    // println!("The longest string is {result2}"); // does not work here

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // static lifetimes
    let s: &'static str = "I have a static lifetime.";
}
