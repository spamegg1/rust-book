enum Cell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    let third: &i32 = &v1[2]; // why reference instead of the value directly?
    println!("The third element is {third}");

    let third = v1.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    let fourth = v1[3]; // so it's possible to get the value directly.
    let does_not_exist = v1.get(100); // out of bounds

    let first = &v1[0]; // immutable borrow
    v1.push(6); // mutable borrow, requires alloc and copy

    // println!("The first element is {first}"); // immutable borrow used here

    // this works
    // for i in v1 {
    //     println!("{i}"); // immutable references
    // }

    // this works but not with the above (use after move)
    // for mut i in v1 {
    //     i += 50; // mutable references
    //     println!("{i}");
    // }

    // this also works, but not together with the above (borrow of moved value)
    // for i in &v1 {
    //     println!("{i}"); // immutable references to each value in vector
    // }

    // this also works, but not together with the above
    for i in &mut v1 {
        *i += 50; // mutable references
        println!("{i}");
    }

    // using enums to hold multiple types in vectors
    let row = vec![
        Cell::Int(3),
        Cell::Text(String::from("blue")),
        Cell::Float(1.23),
    ];
} // v1 goes out of scope and gets dropped here, along with any refs to its contents.
