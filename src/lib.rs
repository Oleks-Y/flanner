mod chatgpt;

use std::{env, error::Error};  

use mongodb::{options::ClientOptions, Client, Database};
use serde::{Deserialize, Serialize};
use futures::stream::TryStreamExt;

use crate::chatgpt::ask_chat_gpt;

trait Controller {
        
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<Ingredient>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub amount: IngredientAmount,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IngredientAmountType {
    LiquidMl,
    Count,
    MassGramms,
}

#[derive(Debug, Serialize, Deserialize)]
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

    // concats recipes and available ingredients in string and sends it to GPT-3 with question to suggest ration
    pub async fn suggest_ration(&self) -> Result<String, Box<dyn Error>> {
        let mut recipes_c = self
            .db
            .collection::<Recipe>("recipes")
            .find(None, None)
            .await?;

        let mut ingredients_c = self
            .db
            .collection::<Ingredient>("ingredients")
            .find(None, None)
            .await?;

        let mut recipes: Vec<Recipe> = Vec::new();
        let mut ingredients: Vec<Ingredient> = Vec::new();

        while let Some(r) = recipes_c.try_next().await? {
            recipes.push(r);
        }

        while let Some(i) = ingredients_c.try_next().await? {
            ingredients.push(i);
        }

        dbg!(&recipes);
        dbg!(&ingredients);

        let mut recipe_names = String::new();
        let mut ingredient_names = String::new();

        for recipe in recipes.iter() {
            recipe_names.push_str(&recipe.name);
            recipe_names.push_str(" ");
        }

        for ingredient in ingredients.iter() {
            ingredient_names.push_str(&ingredient.name);
            ingredient_names.push_str(" ");
        }

        let question = format!(
            "What is the best ration for {} from {}?",
            recipe_names, ingredient_names
        );

        // TODO include priority based on repeatition of rations so it won't be same few days in a row
        let answer = ask_chat_gpt(question).await?;

        // TODO save selected recipes

        Ok(answer)
    }

    // TODO save choosen ration to db
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

