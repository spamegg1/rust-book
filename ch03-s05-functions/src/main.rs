fn main() {
    println!("Hello, world!");
    another_function();
    another_function_2(5);
    print_labeled_measurement(5, 'h');

    // let x = (let y = 6); // statement vs. expression
    let y = {
        let x = 3;
        x + 1
    }; // the curly brace block is an expression
    println!("The value of y is: {y}");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
