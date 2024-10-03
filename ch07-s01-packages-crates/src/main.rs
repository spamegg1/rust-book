use crate::garden::vegetables::Asparagus;

pub mod garden; // ./garden.rs or ./garden/mod.rs

fn main() {
    let plant = Asparagus;
    println!("I'm growing {:?}!", plant);
}
