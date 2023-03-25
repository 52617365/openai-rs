use std::io::{self, BufRead};

const API_URL: &str = "https://api.openai.com/v1/chat/completions";
const MODEL: &str = "gpt-3.5-turbo";
const REGULAR_LINE_BREAK: &str = "\r\n";

enum Role {
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
    let question_copy = question.trim().to_string().clone();

    if question_copy.starts_with("s_") {
        return Role::System(question_copy);
    } else if (*question).starts_with("a_") {
        return Role::Assistant(question_copy);
    } else {
        return Role::User(question_copy);
    }
}
