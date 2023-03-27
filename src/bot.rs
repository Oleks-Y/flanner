use std::{default, error::Error};

use flanner::Recipe;
use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        UpdateHandler,
    },
    prelude::*,
    types::Me,
    utils::command::BotCommands,
};

// #[derive(Debug)]
// pub struct Bot {}

/*
    Bot commands
        - help
        - update recipes
        - update ingredients
        - make ration suggestion
        - save selected recipes
*/

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands")]
enum Command {
    #[command(description = "Show help message", parse_with = "split")]
    Help,
    #[command(description = "Update recipes")]
    UpdateRecipes,
    #[command(description = "Update ingredients")]
    UpdateIngredients,
    #[command(description = "Make ration suggestion")]
    MakeRationSuggestion,
    #[command(description = "Save selected recipes")]
    SaveSelectedRecipes,
}

#[derive(Clone, Default)]
enum State {
    #[default]
    Start,
    WaitingForRecipes,
    WaitingForIngredients,
    WaitingForSuggestionFeedback,
}

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn setup_bot() -> Result<(), Box<dyn Error>> {
    log::info!("Starting Flanner bot...");
    let bot = Bot::from_env();

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(case![Command::Help].endpoint(help))
        .branch(case![Command::UpdateRecipes].endpoint(update_recipes));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::WaitingForRecipes].endpoint(receive_recipes));

    dialogue::enter::<Update, InMemStorage<State>, State, _>().branch(message_handler)
}

async fn help(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;

    Ok(())
}

async fn update_recipes(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Send me recipes!").await?;
    dialogue.update(State::WaitingForRecipes).await?;

    log::info!("Set state to WaitingForRecipes for chat {}", msg.chat.id);

    Ok(())
}

async fn receive_recipes(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    let recipes = msg.text().unwrap();
    log::info!("Received recipes: {}", recipes);

    bot.send_message(msg.chat.id, "Thanks!").await?;
    dialogue.update(State::Start).await?;

    Ok(())
}

async fn parse_recipes(recipesString: &str) -> Result<Vec<Recipe>, Box<dyn Error>> {
    let recipes_clone = recipesString.clone();

    let recipes_strings = recipes_clone.split("#").collect();

    let mut recipes = Vec::new();

    for recipe_string in recipes_strings {
        let recipe = Recipe::from_string(recipe_string)?;
        recipes.push(recipe);
    }

    Ok(())
}


/*
    # Eggs in purgatory 
        - eggs 
        - tomatoes 
        - onions
        - garlic
        - olive oil
        - salt
        - pepper
        - basil
        - oregano
        - parsley
        - chili flakes
 */

// async fn answer_command(bot: Bot, dialogue: MyDialogue) -> ResponseResult<()> {
//     match cmd {
//         Command::Help => {
//             bot.send_message(msg.chat.id, Command::descriptions().to_string())
//                 .await?
//         }
//         Command::UpdateRecipes => todo!(),
//         Command::UpdateIngredients => todo!(),
//         Command::MakeRationSuggestion => todo!(),
//         Command::SaveSelectedRecipes => todo!(),
//     };

//     Ok(())
// }
