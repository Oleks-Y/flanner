use flanner::{
    ask_chat_gpt, get_db, Bot, Flanner, Ingredient, IngredientAmount, IngredientAmountType, Recipe,
};

#[tokio::main]
async fn main() {
    let db = get_db().await.unwrap();

    let flanner = Flanner::new(db);

    let ration_response = match flanner.suggest_ration().await {
        Ok(r) => r,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    println!("{:?}", ration_response);
}
