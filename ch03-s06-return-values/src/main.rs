fn main() {
    let x = five();
    println!("The value of x is: {x}"); // 5

    let x = plus_one(x);
    println!("The value of x is: {x}"); // 6
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
