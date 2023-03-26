use crate::api::chat_res::ChatCompletion;
use reqwest::blocking::Client;
use serde::{ser::SerializeSeq, Serialize, Serializer};
use std::env;
use std::io::{self, BufRead};

const API_URL: &str = "https://api.openai.com/v1/chat/completions";
const MODEL: &str = "gpt-3.5-turbo";
const REGULAR_LINE_BREAK: &str = "\r\n";

// The ChatGPT supports three different question types, system, user and assistant.
#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
enum Role {
    System,
    User,
    Assistant,
}

// This struct will be serialized into the following correct format.
// "messages": [{"role": "user", "content": "Hello!"}]
#[derive(Serialize)]
pub struct Message {
    role: Role,
    content: String,
}

pub fn ask_user_for_questions() -> Vec<Message> {
    print_instructions();

    let stdin = io::stdin();
    let mut questions = vec![];

    loop {
        let mut question = String::new();
        stdin.lock().read_line(&mut question).unwrap();

        if &question == REGULAR_LINE_BREAK {
            break;
        } else {
            let categorized_question = categorize_question(&question);

            questions.push(categorized_question);
        }
    }

    return questions;
}

fn print_instructions() {
    println!("Prefixes: s_{{question}}, a_{{question}}, default is set to user question.");
    println!("s_ stands for system, a_  for assistant respectively.");
    println!("--------------------------------");
    println!("Enter questions. Press enter with no text to go forward.");
}

// We categorize the question into a struct that holds the question type and the question itself.
// Question type can be one of the following: system, assisant, user(default).
// We then add the results into a Message struct for serialization purposes.
fn categorize_question(question: &str) -> Message {
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
        return Message {
            role: Role::System,
            content: msg,
        };
    } else if (*question).starts_with("a_") {
        let msg = copy_and_remove_prefix(question);
        return Message {
            role: Role::Assistant,
            content: msg,
        };
    } else {
        let question_copy = create_copy(question);
        return Message {
            role: Role::User,
            content: question_copy,
        };
    }
}
#[derive(Serialize)]
struct Payload<'a> {
    model: &'static str,
    #[serde(serialize_with = "serialize_vec_ref")]
    messages: &'a Vec<Message>,
}

// This is a customer serializer that allows a Vec to be serialized without performing an expensive copy.
fn serialize_vec_ref<S>(vec: &&Vec<Message>, serializer: S) -> Result<S::Ok, S::Error>
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
    questions: &Vec<Message>,
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

    println!("{}", serialized);
    println!("Sending request then parsing response.");
    let response = client
        .post(API_URL)
        .body(serialized)
        .header("Authorization", api_key)
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
