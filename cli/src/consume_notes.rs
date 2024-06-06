use crate::accounts::consume_game_notes;
use clap::Parser;
use miden_objects::accounts::AccountId;
use tokio::time::{sleep, Duration};
use tokio::task::LocalSet;

#[derive(Debug, Clone, Parser)]
pub struct ConsumeNotesCmd {
    #[arg(short, long, default_value_t = 0)]
    player_id: u64,
}

impl ConsumeNotesCmd {
    pub async fn execute(&self) -> Result<(), String> {
        let account_id = AccountId::try_from(self.player_id).unwrap();
        let local_set = LocalSet::new();
                local_set.run_until(async {
                    loop {
                        consume_game_notes(account_id).await;
                        sleep(Duration::from_secs(5)).await;
                    }
                }).await;
        Ok(())
    }
}