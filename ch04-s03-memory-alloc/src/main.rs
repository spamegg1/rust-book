fn main() {
    let x = 5; // bind 5 to x, stack allocated
    let y = x; // make copy of x's value, bind it to y, stack allocated
               // this is because integers have known fixed size in memory.
    println!("x = {x}, y = {y}"); // stack allocated types have Copy trait
                                  // if a type has Drop trait, Copy trait is disallowed.

    // [ptr, size, capacity]: stack allocated, "hello": heap allocated
    let s1 = String::from("hello"); // no longer valid! moved into s2 below.
    let s2 = s1; // not the same as above. Strings don't have known size.
                 // s2 only copies stack data: [ptr, size, capacity], not heap data.
                 // s1 and s2 both point to the same heap data.

    // borrowing a value that's been moved already is illegal.
    // println!("{s1}, world!"); // error! s1 is no longer valid!

    let s3 = String::from("hello");
    let s4 = s3.clone(); // s3 still valid, we "deep copied"
    println!("s3 = {s3}, s4 = {s4}"); // OK
}
