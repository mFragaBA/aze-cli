use crate::accounts::{ consume_game_notes };
use aze_lib::client::{ create_aze_client, AzeClient };
use aze_lib::constants::{
    PLAYER_DATA_SLOT,
    PLAYER_CARD1_SLOT,
    TEMP_CARD_SLOT,
    REQUESTER_SLOT,
    PHASE_DATA_SLOT,
    FLOP_SLOT,
    PLAYER_FILE_PATH
};
use aze_lib::utils::Player;
use clap::Parser;
use miden_objects::{ accounts::AccountId, Felt, FieldElement };
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Parser)]
pub struct ReceiveCardsCmd {
    #[arg(short, long, default_value_t = 0)]
    player_id: u64,
}

impl ReceiveCardsCmd {
    pub async fn execute(&self) -> Result<(), String> {
        let mut client: AzeClient = create_aze_client();
        let account_id = get_id(&self);
        consume_game_notes(account_id).await;
        println!("Consumed notes");
        Ok(())
    }
}

fn get_id(cmd: &ReceiveCardsCmd) -> AccountId {
    if cmd.player_id == 0 {
        let path = Path::new(PLAYER_FILE_PATH);
        let player: Player = toml
            ::from_str(&fs::read_to_string(path).expect("Failed to read Player.toml"))
            .expect("Failed to deserialize player data");
        return AccountId::try_from(player.player_id()).unwrap();
    }

    AccountId::try_from(cmd.player_id).unwrap()
}