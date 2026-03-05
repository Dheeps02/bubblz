mod models;

use models::user::User;
use models::room::Room;

fn main() {
    let user = User::new(
        1,
        String::from("alice"),
        String::from("alice@example.com"),
        String::from("hashed_password"),
    );

    let room = Room::new(
        1,
        String::from("TestRoom Wohoo"),
        String::from("This is a test room for testing purposes"),
        user.id,
    );


    println!("Created user: {:#?}", user);
    println!("Created room: {:#?}", room);
    println!("Valid username: {}", models::user::User::is_valid_username(&user.username));
}
