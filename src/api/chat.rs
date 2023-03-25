use crate::api::{chat, chat_res::ChatCompletion};
use reqwest::blocking::Client;
use serde::{ser::SerializeSeq, Deserialize, Serialize, Serializer};
use std::io::{self, BufRead};
const API_URL: &str = "https://api.openai.com/v1/chat/completions/";
const MODEL: &str = "gpt-3.5-turbo";
const REGULAR_LINE_BREAK: &str = "\r\n";

#[derive(Serialize)]
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
#[derive(Serialize)]
struct Payload<'a> {
    model: &'static str,
    #[serde(serialize_with = "serialize_vec_ref")] // Used to avoid copying the vec.
    #[serde(rename = "messages")]
    json_questions: &'a Vec<Role>,
}

// This is a customer serializer that allows a vec to be serialized without copying the contents.
fn serialize_vec_ref<S>(vec: &&Vec<Role>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(vec.len()))?;
    for elem in *vec {
        seq.serialize_element(elem)?;
    }
    seq.end()
}

pub fn send_request_to_api(
    questions: &Vec<Role>,
) -> Result<ChatCompletion, Box<dyn std::error::Error>> {
    let client = Client::new();

    let payload = Payload {
        model: MODEL,
        json_questions: questions,
    };

    // let serialized_payload = serde_json::to_string(&payload).unwrap();

    println!("Sending request then parsing response.");
    let response = client.post(API_URL).json(&payload).send()?;
    let chat_completion: ChatCompletion = response.json()?;

    return Ok(chat_completion);
}
