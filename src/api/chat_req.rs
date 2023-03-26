use serde::Serialize;
// The ChatGPT supports three different question types, system, user and assistant.
#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Role {
    System,
    User,
    Assistant,
}

// This will be the struct that gets serialized into the messages payload shown below.
// "messages": [{"role": "user", "content": "Hello!"}]
// This struct looks like the message struct in the response but it has a small difference which is the Role type in the role field.
#[derive(Serialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}
