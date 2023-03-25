use std::io::{self, BufRead};

const API_URL: &str = "https://api.openai.com/v1/chat/completions";
const MODEL: &str = "gpt-3.5-turbo";
const REGULAR_LINE_BREAK: &str = "\r\n";

pub enum Role {
    System(String),
    User(String),
    Assistant(String),
}
pub fn ask_user_for_questions() -> Vec<Role> {
    let stdin = io::stdin();
    let mut questions = vec![];

    println!("Prefixes: s_{{question}}, a_{{question}}, default is set to user question.");
    println!("s_ is system, a_ is assistant.");
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

    let remove_prefix = |og_string: &str| {
        let (_, trimmed_string) = og_string.split_at(2);
        return trimmed_string.to_string();
    };
    if question.starts_with("s_") {
        let question_copy = create_copy(question);
        let remove_prefix = remove_prefix(&question_copy);
        return Role::System(remove_prefix.to_string());
    } else if (*question).starts_with("a_") {
        let question_copy = create_copy(question);
        let remove_prefix = remove_prefix(&question_copy);
        return Role::Assistant(remove_prefix.to_string());
    } else {
        let question_copy = create_copy(question);
        return Role::User(question_copy);
    }
}
