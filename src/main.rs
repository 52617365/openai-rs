mod api;
use crate::api::chat::ask_user_for_questions;
use crate::api::chat::set_api_token;
fn main() {
    set_api_token();
    run();
}
fn run() {
    let questions = ask_user_for_questions();
    let _ = match api::chat::send_request_to_api(&questions) {
        Ok(res) => println!("{}", res.choices[0].message.content),
        Err(e) => println!("{}", e),
    };
    run();
}
