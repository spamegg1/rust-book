fn main() {
    // let reference_to_nothing = dangle(); // error
    let reference_to_something = no_dangle();
    println!("{reference_to_something}");
}

// fn dangle() -> &String { // missing lifetime identifier
//     let s= String::from("hello");
//     &s // returns ref to data owned by current func
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

// s goes out of scope, gets deallocated
// so &s returns a dangling pointer
