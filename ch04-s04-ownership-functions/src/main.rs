fn main() {
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function
                        // and so is no longer valid here

    // println!("{s}"); // error, s is out of scope

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but i32 is copy
                   // so it's OK to still use x afterward

    println!("{x}"); // no error, x is copied since it's stack allocated
}
// here x goes out of scope, then s.
// however s's value was moved, so nothing special happens.

fn takes_ownership(string: String) {
    // string comes into scope
    println!("{string}");
} // string goes out of scope, drop is called, backing memory is freed.

fn makes_copy(int: i32) {
    // int comes into scope
    println!("{int}");
} // int goes out of scope, nothing special happens.
