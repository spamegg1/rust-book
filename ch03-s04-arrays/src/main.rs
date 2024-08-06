use std::io;

fn main() {
    let array = [1, 2, 3, 4, 5]; // stack allocated, fixed size
    let array2: [i32; 5] = [1, 2, 3, 4, 5]; // type annotated
    let array3 = [1; 3]; // [1, 1, 1]
    let first = array[0];
    let second = array[1];

    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = array[index];
    println!("The value of elt at index {index} is: {element}");
}
