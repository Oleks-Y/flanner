use std::{borrow::Borrow, collections::HashMap, env, error::Error, fmt::format, str::FromStr};

use mongodb::{options::ClientOptions, Client, Database};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<Ingredient>,
}

#[derive(Debug, Serialize)]
pub struct Ingredient {
    pub name: String,
    pub amount: IngredientAmount,
}

#[derive(Debug, Serialize)]
pub enum IngredientAmountType {
    LiquidMl,
    Count,
    MassGramms,
}

#[derive(Debug, Serialize)]
pub struct IngredientAmount {
    pub a_type: IngredientAmountType,
    pub value: u64,
}

#[derive(Debug)]
pub struct Flanner {
    pub recipes: Vec<Recipe>,
    db: Database,
}

impl Flanner {
    pub fn new(db: Database) -> Flanner {
        Flanner {
            recipes: Vec::new(),
            db,
        }
    }

    pub async fn save_recipes(&self, recipes: Vec<Recipe>) -> Result<(), Box<dyn Error>> {
        match self
            .db
            .collection("recipes")
            .insert_many(recipes, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub async fn save_ingredients(
        &self,
        ingredients: Vec<Ingredient>,
    ) -> Result<(), Box<dyn Error>> {
        match self
            .db
            .collection("ingredients")
            .insert_many(ingredients, None)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}

#[derive(Debug)]
pub struct Bot {}

impl Bot {
    pub fn new() -> Bot {
        Bot {}
    }
}

pub async fn get_db() -> Result<Database, Box<dyn Error>> {
    // Connect to MongoDB
    let connection_string = env::var("MONGO_DB").expect("Mongodb connection string must be set");
    let client_options = ClientOptions::parse(connection_string).await?;
    let client = Client::with_options(client_options)?;
    let db = client
        .default_database()
        .expect("Failed to get default database");

    Ok(db)
}

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
    }).to_string();

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
