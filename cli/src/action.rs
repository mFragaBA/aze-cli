use crate::actions;
use aze_lib::constants::{BUY_IN_AMOUNT, NO_OF_PLAYERS, SMALL_BLIND_AMOUNT};
use aze_types::actions::GameActionResponse;
use clap::{Parser, ValueEnum};
use dialoguer::{Input, Select};

#[derive(ValueEnum, Debug, PartialEq, Clone)]
enum ActionType {
    Raise,
    SmallBlind,
    BigBlind,
    Call,
    Check,
    Fold,
}

#[derive(Debug, Clone, Parser)]
pub struct ActionCmd {}

impl ActionCmd {
    pub async fn execute(&self, ws_config_path: &std::path::PathBuf) -> Result<(), String> {
        let playerid: u64 = Input::<String>::new()
            .with_prompt("What is your player id?")
            .interact()
            .expect("Failed to get player id")
            .parse()
            .expect("Invalid player id");

        let gameid: u64 = Input::<String>::new()
            .with_prompt("What is the game id?")
            .interact()
            .expect("Failed to get game id")
            .parse()
            .expect("Invalid game id");

        let action_type = Select::new()
            .with_prompt("What is your action type?")
            .item("Raise")
            .item("Small Blind")
            .item("Big Blind")
            .item("Call")
            .item("Check")
            .item("Fold")
            .interact()
            .expect("Failed to get action type");

        let action_type = match action_type {
            0 => ActionType::Raise,
            1 => ActionType::SmallBlind,
            2 => ActionType::BigBlind,
            3 => ActionType::Call,
            4 => ActionType::Check,
            5 => ActionType::Fold,
            _ => panic!("Invalid action type selected"),
        };

        let amount = if action_type == ActionType::Raise {
            let amount: u8 = Input::<String>::new()
                .with_prompt("What is the raise amount?")
                .interact()
                .expect("Failed to get amount")
                .parse()
                .expect("Invalid amount");

            Some(amount)
        } else {
            None
        };

        match send_action(playerid, gameid, action_type, amount, ws_config_path).await {
            Ok(_) => {
                println!("Action performed successfully");
                Ok(())
            }
            Err(e) => Err(format!("{}", e)),
        }
    }
}

async fn send_action(
    player_id: u64,
    game_id: u64,
    action_type: ActionType,
    amount: Option<u8>,
    ws_config_path: &std::path::PathBuf
) -> Result<GameActionResponse, String> {
    match action_type {
        ActionType::Raise => actions::raise(player_id, game_id, amount, ws_config_path).await,
        ActionType::SmallBlind => actions::small_blind(player_id, game_id, ws_config_path).await,
        ActionType::BigBlind => actions::big_blind(player_id, game_id, ws_config_path).await,
        ActionType::Call => actions::call(player_id, game_id, ws_config_path).await,
        ActionType::Check => actions::check(player_id, game_id, ws_config_path).await,
        ActionType::Fold => actions::fold(player_id, game_id, ws_config_path).await,
    }
}
