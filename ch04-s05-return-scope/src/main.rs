fn main() {
    let s1 = gives_ownership(); // g_o moves its return value into s1
    let s2 = String::from("hello"); // s2 comes into scope

    let (s2, len) = calculate_len(s2); // take ownership, give it back
    println!("The length of {s2} is {len}"); // but returning tuples is tedious!

    // s2 is moved into t_a_g_b, which moves its return val into s3.
    let s3 = takes_and_gives_back(s2);

    println!("{s1}"); // OK

    // println!("{s2}"); // error, s2 is not valid, it was moved into s3.

    println!("{s3}"); // OK
}
// here s3 goes out of scope and is dropped.
// s2 was moved, so nothing special happens.
// s1 goes out of scope and is dropped.

// g_o will move its return value into the function which calls it
fn gives_ownership() -> String {
    let some_string = String::from("yours"); // comes into scope
    some_string // is returned and moved out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // is returned and moved out to the calling function
}

fn calculate_len(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
