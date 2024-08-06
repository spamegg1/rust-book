fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of strings, partial or whole.
    let word = first_word(&my_string[..6]); // partial
    let word = first_word(&my_string[..]); // whole

    // first_word also works on references (equiv. to whole slices)
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals, partial or whole.
    let word = first_word(&my_string_literal[..6]); // partial
    let word = first_word(&my_string_literal[..]); // whole

    // first_word also works on string literals, since they are slices already.
    let word = first_word(my_string_literal); // also works with &
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
