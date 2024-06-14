use crate::accounts::{ consume_game_notes, enc_action, dec_action, p2p_unmask_flow };
use aze_lib::client::{ create_aze_client, AzeClient };
use aze_lib::constants::PLAYER_DATA_SLOT;
use clap::Parser;
use miden_objects::accounts::AccountId;
use tokio::time::{ sleep, Duration };
use tokio::task::LocalSet;

#[derive(Debug, Clone, Parser)]
pub struct ConsumeNotesCmd {
    #[arg(short, long)]
    player_id: u64,
}

impl ConsumeNotesCmd {
    pub async fn execute(&self) -> Result<(), String> {
        let mut client: AzeClient = create_aze_client();
        let account_id = AccountId::try_from(self.player_id).unwrap();
        let local_set = LocalSet::new();
        local_set.run_until(async {
            loop {
                consume_game_notes(account_id).await;
                // check here if note triggered enc/dec action
                let (player_account, _) = client.get_account(account_id).unwrap();
                let player_data = player_account.storage().get_item(PLAYER_DATA_SLOT).as_elements().to_vec();
                let action_type = player_data[0].as_int();

                if action_type == 1 || action_type == 2 || action_type == 3 {
                    let target_account = AccountId::try_from(
                        player_data[action_type as usize]
                    ).unwrap();
                    enc_action(action_type, account_id, target_account).await;
                }
                else if action_type == 4 {
                    let target_account = AccountId::try_from(824964168008620216).unwrap(); //game account hardcoded for now
                    enc_action(action_type, account_id, target_account).await;
                }
                sleep(Duration::from_secs(5)).await;
            }
        }).await;
        Ok(())
    }
}