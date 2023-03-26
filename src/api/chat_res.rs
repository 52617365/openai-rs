use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCompletion {
    id: String,
    object: String,
    created: i64,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Debug, Deserialize, Serialize)]
struct Choice {
    index: i64,
    message: Message,
    finish_reason: String,
}

// This struct will be serialized into the following correct format.
// "messages": [{"role": "user", "content": "Hello!"}]
// It's how ever also included in the response.
#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Usage {
    prompt_tokens: i64,
    completion_tokens: i64,
    total_tokens: i64,
}
