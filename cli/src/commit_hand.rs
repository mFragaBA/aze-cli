use crate::accounts::{ commit_hand };
use aze_lib::constants::{ PLAYER_CARD1_SLOT, PLAYER_CARD2_SLOT };
use clap::Parser;
use miden_objects::{ 
    accounts::AccountId,
    Felt, FieldElement
};

#[derive(Debug, Clone, Parser)]
pub struct CommitHandCmd {
    #[arg(short, long, default_value_t = 0)]
    player_id: u64,

    #[arg(short, long, default_value_t = 0)]
    game_id: u64,
}

impl CommitHandCmd {
    pub async fn execute(&self) -> Result<(), String> {
        let sender_account_id = AccountId::try_from(self.player_id).unwrap();
        let game_account_id = AccountId::try_from(self.game_id).unwrap();
        
        commit_hand(sender_account_id, game_account_id).await;

        Ok(())
    }
}