mod accounts;
mod action;
mod actions;
mod commit_hand;
mod connect;
mod receive_cards;
mod init;
mod peek_hand;
mod register;
mod see_hands;
mod stats;
use self::{
    action::ActionCmd, commit_hand::CommitHandCmd, connect::ConnectCmd,
    receive_cards::ReceiveCardsCmd, init::InitCmd, peek_hand::PeekHandCmd, register::RegisterCmd,
    see_hands::SeeHandsCmd, stats::StatsCmd,
};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Parser, Debug, Clone)]
enum Commands {
    Action(ActionCmd),
    ReceiveCards(ReceiveCardsCmd),
    Init(InitCmd),
    PeekHand(PeekHandCmd),
    Register(RegisterCmd),
    Connect(ConnectCmd),
    Stats(StatsCmd),
    SeeHands(SeeHandsCmd),
    CommitHand(CommitHandCmd),
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let ws_config_path = std::path::PathBuf::from("ws_config.json");
    match args.cmd {
        Commands::Init(init_cmd) => {
            if let Err(error) = init_cmd.execute(&ws_config_path).await {
                println!("{}", error);
            }
        }
        Commands::ReceiveCards(receive_cards_cmd) => {
            if let Err(error) = receive_cards_cmd.execute().await {
                println!("{}", error);
            }
        }
        Commands::PeekHand(peek_hand_cmd) => {
            if let Err(error) = peek_hand_cmd.execute().await {
                println!("{}", error);
            }
        }
        Commands::Register(register_cmd) => {
            if let Err(error) = register_cmd.execute().await {
                println!("{}", error);
            }
        }
        Commands::Action(action_cmd) => {
            if let Err(error) = action_cmd.execute(&ws_config_path).await {
                println!("{}", error);
            }
        }
        Commands::Connect(connect_cmd) => {
            if let Err(error) = connect_cmd.execute(&ws_config_path).await {
                println!("{}", error);
            }
        }
        Commands::Stats(stats_cmd) => {
            if let Err(error) = stats_cmd.execute(&ws_config_path).await {
                println!("{}", error);
            }
        }
        Commands::SeeHands(see_hands_cmd) => {
            if let Err(error) = see_hands_cmd.execute(&ws_config_path).await {
                println!("{}", error);
            }
        }
        Commands::CommitHand(commit_hand_cmd) => {
            if let Err(error) = commit_hand_cmd.execute().await {
                println!("{}", error);
            }
        }
    }
}
