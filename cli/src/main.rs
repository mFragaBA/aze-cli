mod utils;
use utils::{ accounts::create_aze_game_account };
use aze_lib::constants::SMALL_BLIND_AMOUNT;
use aze_lib::constants::NO_OF_PLAYERS;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Init {
        #[arg(short, long, num_args = NO_OF_PLAYERS as usize)]
        player: Vec<u64>,
        
        #[arg(short, long, default_value_t = SMALL_BLIND_AMOUNT)]
        small_blind: u8,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Init { player, small_blind } => {

            create_aze_game_account(player, small_blind).await;
        }
    }
}