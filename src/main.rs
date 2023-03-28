mod api;
use crate::api::chat::ask_user_for_questions;
use crate::api::chat::ensure_api_token;
fn main() {
    ensure_api_token();
    run();
}
fn run() {
    let questions = ask_user_for_questions();
    let _ = match api::chat::send_request_to_api(&questions) {
        Ok(res) => println!("Answer: {}", res.choices[0].message.content),
        Err(e) => println!("{}", e),
    };
    run();
}
