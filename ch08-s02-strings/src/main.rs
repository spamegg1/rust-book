fn main() {
    let data = "initial contents"; // this is not String, it's &str
    let s = data.to_string();
    let s = "initial contents".to_string(); // also works on a literal directly

    let s = String::from("initial contents");

    let hello = String::from("علیكم السلام");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("ָלוֹםשׁ");
    let hello = String::from("नम े");
    let hello = String::from("こんにちは");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    s1.push('l');

    println!("s2 is: {s2}"); // push_str does not take ownership
    println!("s1 is: {s1}"); // foobarl

    let s3 = String::from("Hello, ");
    let s4 = String::from("World!");
    let s5 = s3 + &s4; // s4 here is coerced / converted
    println!("s5 is: {s5}"); // + takes ownership of s3, s3 can't be used anymore

    let s6 = "tic";
    let s7 = "tac";
    let s8 = "toe";
    let s9 = format!("{s6}-{s7}-{s8}"); // tic-tac-toe
    println!("s9 is: {s9}");

    // let h = s9[5]; // str cannot be indexed due to Unicode foreign language stuff

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Yellow"), 50); // types are inferred here

    let blue = String::from("Blue");
    scores.insert(blue, 10); // after this, blue is invalid, owned by HashMap
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores); // Yellow 50, Blue 25

    let team_name = String::from("Blue");
    // get method does not take ownership, uses & instead
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // inserting a value only if key does not exist already
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.entry(String::from("Yellow")).or_insert(50); // inserted
    scores2.entry(String::from("Blue")).or_insert(50); // not inserted
    println!("{:?}", scores2); // blue 10, yellow 50

    // updating values based on current values: count word occurrences
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map); // world 2 hello 1 wonderful 1
}
