use crate::accounts::create_aze_player_account;
use aze_types::accounts::AccountCreationError;
use clap::Parser;
use miden_objects::accounts::AccountId;

#[derive(Debug, Clone, Parser)]
pub struct RegisterCmd {
    #[arg(short, long)]
    identifier: String,
}

impl RegisterCmd {
    pub async fn execute(&self) -> Result<(), String> {
        match create_aze_player_account(self.identifier.clone()).await {
            Ok(account_id) => {
                println!("Player account created: {:?}", account_id);
                Ok(())
            }
            Err(e) => Err(format!("Error creating player account: {}", e)),
        }
    }
}
