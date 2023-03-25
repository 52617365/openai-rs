use serde::{Deserialize, Serialize};
use std::io::{self, BufRead};
const API_URL: &str = "https://api.openai.com/v1/chat/completions/";
const MODEL: &str = "gpt-3.5-turbo";
const REGULAR_LINE_BREAK: &str = "\r\n";

#[derive(Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "system")]
    System(String),
    #[serde(rename = "user")]
    User(String),
    #[serde(rename = "assistant")]
    Assistant(String),
}

pub fn ask_user_for_questions() -> Vec<Role> {
    let stdin = io::stdin();
    let mut questions = vec![];

    println!("Prefixes: s_{{question}}, a_{{question}}, default is set to user question.");
    println!("s_ stands for system, a_  for assistant respectively.");
    println!("--------------------------------");
    println!("Enter questions. Press enter with no text to go forward.");

    loop {
        let mut temp_question = String::new();
        stdin.lock().read_line(&mut temp_question).unwrap();

        if &temp_question == REGULAR_LINE_BREAK {
            break;
        } else {
            let categorized_question = categorize_question(&temp_question);
            questions.push(categorized_question);
        }
    }

    return questions;
}

fn categorize_question(question: &str) -> Role {
    let create_copy = |ref_string: &str| return ref_string.trim().to_string();

    let remove_prefix = |ref_string: &str| {
        let (_, trimmed_string) = ref_string.split_at(2);
        return trimmed_string.to_string();
    };

    if question.starts_with("s_") {
        let question_copy = create_copy(question);
        let remove_prefix = remove_prefix(&question_copy);
        return Role::System(remove_prefix);
    } else if (*question).starts_with("a_") {
        let question_copy = create_copy(question);
        let remove_prefix = remove_prefix(&question_copy);
        return Role::Assistant(remove_prefix);
    } else {
        let question_copy = create_copy(question);
        return Role::User(question_copy);
    }
}

pub fn construct_payload(questions: &Vec<Role>) -> String {
    let serialized_vector = serde_json::to_string(&questions).unwrap();

    return serialized_vector;
}
