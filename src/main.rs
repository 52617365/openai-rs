mod api;
use crate::api::chat::ask_user_for_questions;
fn main() {
    let questions = ask_user_for_questions();
    for question in questions {
        println!("{}", question);
    }
}
