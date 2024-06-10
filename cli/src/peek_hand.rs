use crate::accounts::{ send_note };
use clap::Parser;
use miden_objects::accounts::AccountId;

#[derive(Debug, Clone, Parser)]
pub struct PeekHandCmd {
    #[arg(short, long, default_value_t = 0)]
    player_id: u64,
}

impl PeekHandCmd {
    pub async fn execute(&self) -> Result<(), String> {
        let players_ids = [359196095275670923, 359196095275670923, 317826241474458840];
        let sender_account_id = AccountId::try_from(self.player_id).unwrap();

        for player_id in players_ids.iter() {
            let receiver_account_id = AccountId::try_from(*player_id).unwrap();
            send_note(sender_account_id, receiver_account_id).await;
        }

        Ok(())
    }
}