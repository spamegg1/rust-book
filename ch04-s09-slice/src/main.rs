fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // 5
    s.clear(); // empties the string

    // word is still 5 here, so it's totally invalid!
    println!("{word}");
    // now if we try to use word to slice s, it's a bug!
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len() // no spaces found in string, return whole size
}
