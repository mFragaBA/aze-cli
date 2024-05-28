use clap::{Parser, Subcommand};
use aze_lib::constants::SMALL_BLIND_AMOUNT;
use aze_lib::constants::NO_OF_PLAYERS;

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
        player: Vec<String>,
        
        #[arg(short, long, default_value_t = 2 * SMALL_BLIND_AMOUNT)]
        big_blind: u8,
        
        #[arg(short, long, default_value_t = SMALL_BLIND_AMOUNT)]
        small_blind: u8,
    },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Init { player, big_blind, small_blind } => {

            if !player.is_empty() {
                for id in player {
                    println!("Player ID: {}", id);
                }
            }

            println!("Big Blind: {}", big_blind);
            
            println!("Small Blind: {}", small_blind);
        }
    }
}