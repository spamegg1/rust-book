fn main() {
    let mut s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("{hello} {world}");

    let word = first_word(&s); // immutable borrowing here.
    // s.clear(); // error! cannot borrow as mutable & as immutable.
    println!("{word}");
}

// now the correct version of func from prev section:
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
