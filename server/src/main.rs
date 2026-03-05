mod models;


fn main() {
    let user = models::user::User::new(
        1,
        String::from("alice"),
        String::from("alice@example.com"),
        String::from("hashed_password"),
    );

    println!("Created user: {}", user.username);
    println!("Valid username: {}", models::user::User::is_valid_username(&user.username));
}
