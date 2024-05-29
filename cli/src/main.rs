mod accounts;
mod utils;
use accounts::{ create_aze_game_account, create_aze_player_account };
use utils::load_config;
use aze_lib::constants::{ SMALL_BLIND_AMOUNT, NO_OF_PLAYERS, BUY_IN_AMOUNT };
use clap::{ ValueEnum, Parser, Subcommand };

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
    }
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
        }
    }
}