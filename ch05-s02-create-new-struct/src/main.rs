struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("anotheremail@example.com"),
        ..user1
    };

    // println!("{}", user1.username); // user1.username was MOVED into user2.username
    // cannot be used, String does not have Copy trait
    println!(
        "{} {} {} {}",
        user2.active, user2.username, user2.email, user2.sign_in_count
    );
}
