fn main() {
    let x = 5;
    let x = x + 1; // this shadows the previous x
    {
        let x = x * 2; // this shadows the outer (second) x
        println!("The value of x in inner scope: {x}"); // 12
    }
    println!("The value of x: {x}"); // 6

    let spaces = "    ";
    let spaces = spaces.len(); // redefine spaces to be a different type
    println!("{spaces}"); // 4

    let mut hello = "hello";
    // hello = hello.len(); // error
    println!("{hello}");
}
