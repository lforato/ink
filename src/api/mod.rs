use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
struct Chunk {
    model: String,
    created_at: String,
    done: bool,
    message: Msg,
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct Msg {
    role: String,
    content: String,
}

#[derive(Deserialize, Serialize, Debug)]
enum Role {
    System,
    User,
}

impl Default for Role {
    fn default() -> Self {
        Self::System
    }
}

pub async fn ask_question(result: &mut String) -> Result<&String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let bytes = client
        .post("http://localhost:11434/api/chat")
        .body(r#"{ "model": "deepseek-r1:8b", "messages": [{ "role": "system", "content": "You are a helpful assistant." }, { "role": "user", "content": "Tell me a joke about penguins." }] }"#)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let mut stream = bytes.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        let text = String::from_utf8_lossy(chunk.as_ref()).to_string();
        let chunk: Chunk = serde_json::from_str(text.as_str()).unwrap();
        result.push_str(&chunk.message.content);
    }

    Ok(result)
}
