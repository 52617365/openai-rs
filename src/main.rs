mod api;
use crate::api::chat::ask_user_for_questions;
fn main() {
    let questions = ask_user_for_questions();
    for question in questions {
        match question {
            api::chat::Role::Assistant(some_string) => println!("Assistant - {}", some_string),
            api::chat::Role::User(some_string) => println!("User - {}", some_string),
            api::chat::Role::System(some_string) => println!("System - {}", some_string),
        }
    }
}
