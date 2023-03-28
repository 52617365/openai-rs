use crate::api::chat_req;
use crate::api::chat_res::ChatCompletion;
use reqwest::blocking::Client;
use serde::{ser::SerializeSeq, Serialize, Serializer};
use std::env;
use std::io::{self, BufRead};

const API_URL: &str = "https://api.openai.com/v1/chat/completions";
const MODEL: &str = "gpt-3.5-turbo";
const REGULAR_LINE_BREAK: &str = "\r\n";

pub fn ask_user_for_questions() -> Vec<chat_req::Message> {
    print_instructions();
    let stdin = io::stdin();
    let mut categorized_questions = vec![];

    loop {
        let mut question = String::new();
        stdin.lock().read_line(&mut question).unwrap();

        if &question == REGULAR_LINE_BREAK {
            break;
        } else {
            let categorized_question = categorize_question(&question);

            categorized_questions.push(categorized_question);
        }
    }

    return categorized_questions;
}

pub fn ensure_api_token() -> () {
    match env::var("CHATGPT_API_KEY") {
        Ok(_) => (),
        Err(_) => {
            panic!("CHATGPT_API_KEY env variable is empty");
        }
    }
}

fn print_instructions() {
    println!("================================");
    println!("Prefixes: s_{{question}}, a_{{question}}, default is set to user question.");
    println!("Press enter with no text to go forward.");
    println!("================================");
}

// We categorize the question into a struct that holds the question type and the question itself.
// Question type can be one of the following: system, assisant, user(default).
// We then add the results into a Message struct for serialization purposes.
fn categorize_question(question: &str) -> chat_req::Message {
    let create_copy = |ref_string: &str| return ref_string.trim().to_string();

    let remove_prefix = |ref_string: &str| {
        let (_, trimmed_string) = ref_string.split_at(2);
        return trimmed_string.to_string();
    };

    let copy_and_remove_prefix = |ref_string: &str| {
        let copy = create_copy(ref_string);
        let trimmed = remove_prefix(&copy);
        return trimmed;
    };

    if question.starts_with("s_") {
        let msg = copy_and_remove_prefix(question);
        return chat_req::Message {
            role: chat_req::Role::System,
            content: msg,
        };
    } else if question.starts_with("a_") {
        let msg = copy_and_remove_prefix(question);
        return chat_req::Message {
            role: chat_req::Role::Assistant,
            content: msg,
        };
    } else {
        let question_copy = create_copy(question);
        return chat_req::Message {
            role: chat_req::Role::User,
            content: question_copy,
        };
    }
}
#[derive(Serialize)]
struct Payload<'a> {
    model: &'static str,
    #[serde(serialize_with = "serialize_vec_ref")]
    messages: &'a Vec<chat_req::Message>,
}

// This is a customer serializer that allows a Vec to be serialized without performing an expensive copy.
fn serialize_vec_ref<S>(vec: &&Vec<chat_req::Message>, serializer: S) -> Result<S::Ok, S::Error>
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
    questions: &Vec<chat_req::Message>,
) -> Result<ChatCompletion, Box<dyn std::error::Error>> {
    let api_key = format!(
        "Bearer {}",
        env::var("CHATGPT_API_KEY").expect("CHATGPT_API_KEY environment variable is not set."),
    );

    let client = Client::new();

    let payload = Payload {
        model: MODEL,
        messages: questions,
    };

    let serialized = serde_json::to_string(&payload).unwrap();

    let response = client
        .post(API_URL)
        .body(serialized)
        .header("Authorization", api_key)
        .header("Content-Type", "application/json")
        .send()?;

    if response.status().is_success() {
        return match response.json() {
            Ok(res) => Ok(res),
            Err(_) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Connected to the API but failed to deserialize the response.",
            ))),
        };
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Couldn't connect to the API.",
        )))
    }
}
