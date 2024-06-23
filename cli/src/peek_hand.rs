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
        for (i, slot) in (PLAYER_CARD1_SLOT..PLAYER_CARD2_SLOT + 1).enumerate() {
            let card = player_account.storage().get_item(slot);
            println!("Card {}: {:?}", i + 1, card);
        }

        Ok(())
    }
}