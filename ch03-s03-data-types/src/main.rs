use std::io;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    let x = 2.0; // f64
    let y: f32 = 2.0; // f32
    let sum = 5 + 10; // addition
    let diff = 95.5 - 4.3; // subtraction
    let product = 4 * 30; // multiplication
    let quotient = 56.7 / 32.2; // division
    let truncated = -5 / 3; // results in -1
    let remainder = 43 % 5; // remainder
    let t = true;
    let f: bool = false;
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{guess} {x} {y} {sum} {diff} {product} {heart_eyed_cat}");
    println!("{quotient} {truncated} {remainder} {t} {f} {c} {z}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);

    let (a, b, c) = tup; // destructuring
    println!("{a} {b} {c}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred} {six_point_four} {one}");
}
