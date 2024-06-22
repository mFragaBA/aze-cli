mod accounts;
mod action;
mod actions;
mod commit_hand;
mod connect;
mod consume_notes;
mod init;
mod peek_hand;
mod register;
mod stats;
use self::{
    action::ActionCmd,
    init::InitCmd,
    consume_notes::ConsumeNotesCmd,
    register::RegisterCmd,
    connect::ConnectCmd,
    stats::StatsCmd,
    peek_hand::PeekHandCmd,
    commit_hand::CommitHandCmd,
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
    ConsumeNotes(ConsumeNotesCmd),
    Init(InitCmd),
    PeekHand(PeekHandCmd),
    Register(RegisterCmd),
    Connect(ConnectCmd),
    Stats(StatsCmd),
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
        Commands::ConsumeNotes(consume_notes_cmd) => {
            if let Err(error) = consume_notes_cmd.execute().await {
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
        Commands::CommitHand(commit_hand_cmd) => {
            if let Err(error) = commit_hand_cmd.execute().await {
                println!("{}", error);
            }
        }
    }
}