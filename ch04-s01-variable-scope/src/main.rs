// s is not valid here, since it's not declared yet.
fn main() {
    let s = "hello"; // s is valid from this point onward.
    println!("{s}");
} // this scope is now over, s is no longer valid.
