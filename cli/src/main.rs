mod accounts;
mod actions;
mod utils;
use accounts::{ create_aze_game_account, create_aze_player_account };
use utils::load_config;
use aze_lib::constants::{ SMALL_BLIND_AMOUNT, NO_OF_PLAYERS, BUY_IN_AMOUNT };
use clap::{ ValueEnum, Parser, Subcommand };
use dialoguer::{ Input, Select };

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(ValueEnum, Debug, Clone)]
enum GameType {
    Holdem,
    Omaha,
    Hilo,
}

#[derive(ValueEnum, Debug, PartialEq, Clone)]
enum ActionType {
    Raise,
    SmallBlind,
    BigBlind,
    Call,
    Check,
    Fold,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Init {
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
    },
    Register {
        #[arg(short, long)]
        identifier: String,
    },
    Action 
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Init { game_type, player, small_blind, buy_in, config } => {
            let mut player_ids = player.unwrap_or_else(Vec::new);
            let mut small_blind_amount = small_blind;
            let mut buy_in_amount = buy_in;

            if let Some(config_path) = config {
                match load_config(&config_path) {
                    Ok(config) => {
                        player_ids = config.player_ids;
                        small_blind_amount = config.small_blind;
                        buy_in_amount = config.buy_in;
                    },
                    Err(e) => {
                        eprintln!("Error loading config: {}", e);
                    }
                }
            }
            
            let game_account_id = create_aze_game_account(player_ids, small_blind_amount, buy_in_amount).await.unwrap();
            println!("Game account created: {:?}", game_account_id);
        },
        Commands::Register { identifier } => {
            let account_creation_response = create_aze_player_account(identifier).await;
            match account_creation_response {
                Ok(account_id) => {
                    println!("Player account created: {:?}", account_id);
                },
                Err(e) => {
                    eprintln!("Error creating player account: {}", e);
                }
            }
        },
        Commands::Action => {
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

            if let Err(error) = perform_action(playerid, gameid, action_type, amount).await {
                eprintln!("{}", error);
            }
        }
    }
}

async fn perform_action(player_id: u64, game_id: u64, action_type: ActionType, amount: Option<u8>) -> Result<(), String> {
    match action_type {
        ActionType::Raise => {
            actions::raise(player_id, game_id, amount).await?;
        },
        ActionType::SmallBlind => {
            actions::small_blind(player_id, game_id).await?;
        },
        ActionType::BigBlind => {
            actions::big_blind(player_id, game_id).await?;
        },
        ActionType::Call => {
            actions::call(player_id, game_id).await?;
        },
        ActionType::Check => {
            actions::check(player_id, game_id).await?;
        },
        ActionType::Fold => {
            actions::fold(player_id, game_id).await?;
        }
    }

    Ok(())
}