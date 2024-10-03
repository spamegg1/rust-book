use rand::Rng;
use std::collections::*; // the glob operator
use std::io::{self, Write}; // here Write is io::Write
use std::{alloc, cmp::Ordering}; // nested paths

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("The secret number is {secret_number}");
}
