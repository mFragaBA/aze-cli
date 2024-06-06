mod accounts;
mod action;
mod actions;
mod init;
mod register;
mod connect;
use self::{action::ActionCmd, init::InitCmd, register::RegisterCmd, connect::ConnectCmd};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Parser, Debug, Clone)]
enum Commands {
    Init(InitCmd),
    Register(RegisterCmd),
    Action(ActionCmd),
    Connect(ConnectCmd)
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
        Commands::Connect(connect_cmd) => {
            if let Err(error) = connect_cmd.execute().await {
                println!("{}", error);
            }
        }
    }
}
