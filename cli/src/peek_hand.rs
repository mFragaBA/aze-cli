use crate::accounts::{ p2p_unmask_flow };
use aze_lib::client::{ create_aze_client, AzeClient };
use aze_lib::constants::{ PLAYER_CARD1_SLOT, PLAYER_CARD2_SLOT };
use clap::Parser;
use miden_objects::{ 
    accounts::AccountId,
    Felt, FieldElement
};

#[derive(Debug, Clone, Parser)]
pub struct PeekHandCmd {
    #[arg(short, long, default_value_t = 0)]
    player_id: u64,
}

impl PeekHandCmd {
    pub async fn execute(&self) -> Result<(), String> {
        let sender_account_id = AccountId::try_from(self.player_id).unwrap();
        let mut client: AzeClient = create_aze_client();
        let (player_account, _) = client.get_account(sender_account_id).unwrap();
        let mut cards: [[Felt; 4]; 3] = [[Felt::ZERO; 4]; 3];
        for (i, slot) in (PLAYER_CARD1_SLOT..PLAYER_CARD2_SLOT + 1).enumerate() {
            let card = player_account.storage().get_item(slot);
            cards[i] = card.into();
        }
        p2p_unmask_flow(sender_account_id, cards).await;

        Ok(())
    }
}