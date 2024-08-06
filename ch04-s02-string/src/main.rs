fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // append literal to string
    println!("{s}"); // this will print hello, world!
}
