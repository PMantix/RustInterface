use reqwest::blocking::Client;
use serde_json::json;
use std::env;

pub fn send_prompt(prompt: &str) -> anyhow::Result<String> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = Client::new();

    let req_body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": prompt}],
    });

    let resp: serde_json::Value = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&req_body)
        .send()?
        .json()?;

    let reply = resp["choices"][0]["message"]["content"].as_str().unwrap_or("");
    Ok(reply.to_string())
}
