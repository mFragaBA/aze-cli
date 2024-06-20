use std::path::PathBuf;

use ansi_term::Colour::{Blue, Green, Red, Yellow};
use aze_lib::{
    client::{create_aze_client, AzeClient},
    utils::{get_stats, Ws_config},
};
use clap::Parser;
use dialoguer::Input;
use miden_objects::accounts::AccountId;

#[derive(Debug, Clone, Parser)]
pub struct SeeHandsCmd;

impl SeeHandsCmd {
    //TODO: Find a way to get player's identifiers here
    pub async fn execute(&self, ws_config: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let gameid: u64 = Input::<String>::new()
            .with_prompt("What is the game id?")
            .interact()
            .expect("Failed to get game id")
            .parse()
            .expect("Invalid game id");
        let mut client: AzeClient = create_aze_client();
        let game_account_id = AccountId::try_from(gameid).unwrap();
        let ws_url = Ws_config::load(ws_config).url.unwrap();
        let stat_data: aze_lib::utils::StatResponse = get_stats(game_account_id.to_string(), ws_url).await?;

        let hands_output = format!(
            "{} \n\
             {} \n\
             {} \n\
             {} \n\
             {} \n\
             {} \n\
             {} \n\
             {} \n\
             {} \n\
             {} \n\
             {} 
            ",
            Blue.bold().paint("+--------------------------------------+"),
            Red.bold().paint("+           PLAYER HANDS               +"),
            Blue.bold().paint("+--------------------------------------+"),
            Yellow.bold().paint(format!("+ {:^15}  | {:15}   +", "coolguy001", get_hand(stat_data.player_hands[0]).unwrap())),
            Blue.bold().paint("+--------------------------------------+"),
            Yellow.bold().paint(format!("+ {:^15}  | {:15}   +", "gambler#1", get_hand(stat_data.player_hands[0]).unwrap())),
            Blue.bold().paint("+--------------------------------------+"),
            Yellow.bold().paint(format!("+ {:^15}  | {:15}   +", "punisher002", get_hand(stat_data.player_hands[0]).unwrap())),
            Blue.bold().paint("+--------------------------------------+"),
            Yellow.bold().paint(format!("+ {:^15}  | {:15}   +", "noob_o0", get_hand(stat_data.player_hands[0]).unwrap())),
            Blue.bold().paint("+--------------------------------------+"),
        );

        println!("{}", hands_output);
        Ok(())
    }
}

fn get_hand(int_hand: u64) -> Result<(String), String> {
    let hand = match int_hand {
        0 => "Royal Flush",
        1 => "Straight Flush",
        2 => "4 of Kind",
        3 => "Full House",
        4 => "Flush",
        5 => "Straight",
        6 => "3 of Kind",
        7 => "Two Pair",
        8 => "Pair",
        9 => "High Card",
        _ => panic!("Invalid hand"),
    };
    Ok(hand.to_string())
}
