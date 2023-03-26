mod bot;
mod chatgpt;

use flanner::{get_db, Flanner, Ingredient, IngredientAmount, IngredientAmountType, Recipe};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    bot::setup_bot().await.unwrap();

    // let db = get_db().await.unwrap();

    // let flanner = Flanner::new(db);

    // let ration_response = match flanner.suggest_ration().await {
    //     Ok(r) => r,
    //     Err(e) => {
    //         println!("Error: {}", e);
    //         return;
    //     }
    // };

    // println!("{:?}", ration_response);
}
