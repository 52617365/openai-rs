mod api;
use crate::api::chat::ask_user_for_questions;
fn main() {
    let questions = ask_user_for_questions();
    let payload = api::chat::send_request_to_api(&questions);

    println!("{}", payload);
}
