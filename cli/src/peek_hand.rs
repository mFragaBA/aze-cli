use crate::accounts::consume_game_notes;
use clap::Parser;
use miden_objects::accounts::AccountId;

#[derive(Debug, Clone, Parser)]
pub struct PeekHandCmd {
    #[arg(short, long, default_value_t = 0)]
    player_id: u64,
}

impl PeekHandCmd {
    pub async fn execute(&self) -> Result<(), String> {
        let account_id = AccountId::try_from(self.player_id).unwrap();
        consume_game_notes(account_id).await;
        Ok(())
    }
}