fn main() {
    let s1 = String::from("hello");
    let len = calculate_len(&s1); // s1 still valid, it is not moved.
    println!("The length of {s1} is {len}");
}

// references are like pointers (guaranteed non-null)
fn calculate_len(s: &String) -> usize {
    s.len()
} // s is not dropped when it goes out of scope, because it's not owned.

fn change(s: &String) {
    // s.push_str(", world"); // cannot modify borrowed value as mutable
}
