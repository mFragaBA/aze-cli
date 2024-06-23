use crate::accounts::{
    consume_game_notes,
};
use aze_lib::client::{ create_aze_client, AzeClient };
use aze_lib::constants::{ PLAYER_DATA_SLOT, PLAYER_CARD1_SLOT, TEMP_CARD_SLOT, REQUESTER_SLOT, PHASE_DATA_SLOT, FLOP_SLOT };
use clap::Parser;
use miden_objects::{
    accounts::AccountId,
    Felt, FieldElement
};
use tokio::time::{ sleep, Duration };
use tokio::task::LocalSet;

#[derive(Debug, Clone, Parser)]
pub struct ConsumeNotesCmd {
    #[arg(short, long)]
    player_id: u64,

    #[arg(short, long)]
    game_id: u64,
}

impl ConsumeNotesCmd {
    pub async fn execute(&self) -> Result<(), String> {
        let mut client: AzeClient = create_aze_client();
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