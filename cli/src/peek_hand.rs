use crate::accounts::{ send_note, p2p_unmask_flow };
use clap::Parser;
use miden_objects::accounts::AccountId;

#[derive(Debug, Clone, Parser)]
pub struct PeekHandCmd {
    #[arg(short, long, default_value_t = 0)]
    player_id: u64,
}
 
impl PeekHandCmd {
    pub async fn execute(&self) -> Result<(), String> {
        let sender_account_id = AccountId::try_from(self.player_id).unwrap();
        p2p_unmask_flow(sender_account_id).await;

        Ok(())
    }
}