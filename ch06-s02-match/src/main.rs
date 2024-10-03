#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(number: Option<i32>) -> Option<i32> {
    match number {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

fn main() {
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("{:?}", plus_one(Some(1)));

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // this arm is selected
    }

    // concise control flow with `if let`, when we only want 1 branch.
    let my_coin = Coin::Quarter(UsState::Alabama);
    // this avoids large match expression
    if let Coin::Quarter(state) = my_coin {
        println!("It's a quarter from state {:?}!", state);
    } else {
        println!("It's not a quarter :(");
    }
}
