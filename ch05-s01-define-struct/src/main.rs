struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // short-hand syntax
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    ); // entire instance must be mutable; cannot mark only certain fields as mutable

    user1.email = String::from("anothermail@example.com");

    println!(
        "{} {} {} {}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );
}
