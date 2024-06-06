mod accounts;
mod action;
mod actions;
mod init;
mod consume_notes;
mod register;
use self::{ action::ActionCmd, init::InitCmd, consume_notes::ConsumeNotesCmd, register::RegisterCmd };
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
    Register(RegisterCmd),
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    match args.cmd {
        Commands::Init(init_cmd) => {
            if let Err(error) = init_cmd.execute().await {
                println!("{}", error);
            }
        }
        Commands::ConsumeNotes(consume_notes_cmd) => {
            if let Err(error) = consume_notes_cmd.execute().await {
                println!("{}", error);
            }
        }
        Commands::Register(register_cmd) => {
            if let Err(error) = register_cmd.execute().await {
                println!("{}", error);
            }
        }
        Commands::Action(action_cmd) => {
            if let Err(error) = action_cmd.execute().await {
                println!("{}", error);
            }
        }
    }
}
