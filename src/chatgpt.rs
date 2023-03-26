use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, error::Error};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: i64,
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: i64,
    #[serde(rename = "total_tokens")]
    pub total_tokens: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    pub message: Message,
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
    pub index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub async fn ask_chat_gpt(message: String) -> Result<String, Box<dyn Error>> {
    let api_key = env::var("GPT_API_KEY").expect("GPT API key must be set");
    let bearer_auth = format!("Bearer {}", api_key);

    let client = reqwest::Client::new();

    let data = json!({
        "model" : "gpt-3.5-turbo",
        "messages" :
        [
            {
            "role" : "user",
            "content" : message
            }
        ]
    })
    .to_string();

    dbg!(&data);

    let resp = client
        .post("https://api.openai.com/v1/chat/completions")
        .header(ACCEPT, "*/*")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, &bearer_auth)
        .body(data)
        .send()
        .await?;

    match resp.status() {
        reqwest::StatusCode::OK => {
            match resp.json::<Root>().await {
                Ok(parsed) => {
                    println!("🔥 Success!");
                    println!("💬 Response: {}", parsed.choices[0].message.content);
                    return Ok(String::from(parsed.choices[0].message.content.clone()));
                }
                Err(_) => println!("🛑 Hm, the response didn't match the shape we expected."),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("🛑 Status: UNAUTHORIZED - Need to grab a new token");
            panic!("Shutting down, token incorrect")
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            println!("🛑 Status: 429 - Too many requests");
            return Err("429".into());
        }
        other => {
            panic!(
                "🛑 Uh oh! Something unexpected happened: status [{:#?}], body {}",
                other,
                resp.text().await?
            );
        }
    };

    Ok(String::from("Error"))
}
