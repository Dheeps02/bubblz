mod models;

use models::user::User;
use models::room::Room;
use models::message::{Message, MessageType};

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

    let message = Message::new(
        1,
        MessageType::Text,
        user.id,
        room.id,
        String::from("Wohooo First Message")
    );

    println!("Created user: {:#?}", user);
    println!("Created room: {:#?}", room);
    println!("Created message: {:#?}", message);
    println!("Valid username: {}", models::user::User::is_valid_username(&user.username));
}
