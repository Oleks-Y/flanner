use flanner::{
    ask_chat_gpt, get_db, Bot, Flanner, Ingredient, IngredientAmount, IngredientAmountType, Recipe,
};

#[tokio::main]
async fn main() {
    let db = get_db().await.unwrap();

    let flanner = Flanner::new(db);

    // let recipes = vec![Recipe {
    //     name: String::from("first"),
    //     ingredients: vec![Ingredient {
    //         name: String::from("first"),
    //         amount: IngredientAmount {
    //             a_type: { IngredientAmountType::MassGramms },
    //             value: 1,
    //         },
    //     }],
    // }];

    // let ingredients = vec![Ingredient {
    //     name: String::from("first"),
    //     amount: {
    //         IngredientAmount {
    //             a_type: { IngredientAmountType::MassGramms },
    //             value: 1,
    //         }
    //     },
    // }];

    // flanner.save_recipes(recipes).await.unwrap();
    // flanner.save_ingredients(ingredients).await.unwrap();

    ask_chat_gpt("Show me that you're working!".to_string()).await.unwrap();

}
