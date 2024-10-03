fn deliver_order() {}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // super is like ../
    }

    fn cook_order() {}
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;

    // Order a breakfast in the summer with rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please!", meal.toast);

    // The next line won't compile, we're not allowed to modify fruit.
    // meal.seasonal_fruit = String::from("blueberries");

    crate::front_of_house::hosting::add_to_waitlist(); // abs path
    front_of_house::hosting::add_to_waitlist(); // rel path
    hosting::add_to_waitlist(); // use keyword shortcut
}
