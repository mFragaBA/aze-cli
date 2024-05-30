use crate::accounts::create_aze_game_account;
use aze_lib::constants::{ SMALL_BLIND_AMOUNT, NO_OF_PLAYERS, BUY_IN_AMOUNT };
use aze_types::accounts::AccountCreationError;
use miden_objects::accounts:: AccountId;
use clap::{ ValueEnum, Parser };
use std::path::PathBuf;
use serde::Deserialize;
use figment::{
    providers::{ Format, Toml },
    Figment,
};

#[derive(ValueEnum, Debug, Clone)]
enum GameType {
    Holdem,
    Omaha,
    Hilo,
}

#[derive(Debug, Clone, Parser)]
pub struct InitCmd {
        #[arg(short, long, value_enum, default_value = "holdem")]
        game_type: GameType,

        #[arg(short, long, num_args = NO_OF_PLAYERS as usize)]
        player: Option<Vec<u64>>,
        
        #[arg(short, long, default_value_t = SMALL_BLIND_AMOUNT)]
        small_blind: u8,

        #[arg(short, long, default_value_t = BUY_IN_AMOUNT)]
        buy_in: u64,

        #[arg(short, long, value_parser)]
        config: Option<std::path::PathBuf>,
}

impl InitCmd {
    pub async fn execute(&self) -> Result<(), String> {
        let mut player_ids = self.player.clone().unwrap_or_else(Vec::new);
        let mut small_blind_amount = self.small_blind;
        let mut buy_in_amount = self.buy_in;

        if let Some(config_path) = &self.config {
            match load_config(&config_path) {
                Ok(config) => {
                    player_ids = config.player_ids;
                    small_blind_amount = config.small_blind;
                    buy_in_amount = config.buy_in;
                },
                Err(e) => {
                    return Err(format!("Error loading config: {}", e));
                }
            }
        }
        
        match create_aze_game_account(player_ids, small_blind_amount, buy_in_amount).await {
            Ok(game_account_id) => {
                println!("Game account created: {:?}", game_account_id);
                Ok(())
            },
            Err(e) => Err(format!("Error creating game account: {}", e)),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    player_ids: Vec<u64>,
    small_blind: u8,
    buy_in: u64,
}

fn load_config(config_file: &PathBuf) -> Result<Config, String> {
    Figment::from(Toml::file(config_file))
        .extract()
        .map_err(|err| format!("Failed to load {} config file: {err}", config_file.display()))
}