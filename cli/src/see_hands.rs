use std::path::PathBuf;

use ansi_term::Colour::{Blue, Green, Red, Yellow};
use aze_lib::{
    client::{create_aze_client, AzeClient},
    utils::{card_from_number, get_stats, Ws_config},
};
use clap::Parser;
use dialoguer::Input;
use miden_objects::accounts::AccountId;

#[derive(Debug, Clone, Parser)]
pub struct SeeHandsCmd;

impl SeeHandsCmd {
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
        let stat_data: aze_lib::utils::StatResponse =
            get_stats(game_account_id.to_string(), ws_url).await?;

        // Check: If game has not ended there is no hand to show. Comment for testing
        // if stat_data.current_state != 3 {
        //     println!("{}",Red.bold().paint("Game not ended yet!!!"));
        //     return Ok(());
        // }

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
             {} \n\
             {} \n\
             {} \n\
             {} \n\
             {} \n\
             {} 
            ",
            Blue.bold()
                .paint("+---------------------------------------------------+"),
            Red.bold()
                .paint("|                  PLAYER HANDS                     |"),
            Blue.bold()
                .paint("|---------------------------------------------------|"),
            Blue.bold()
                .paint("|                 COMMUNITY CARDS                   |"),
            Blue.bold().paint(format!(
                "|------ {:^37} ------|",
                format!(
                    "{:4} {:4} {:4} {:4} {:4}",
                    card_from_number(stat_data.community_cards[0]),
                    card_from_number(stat_data.community_cards[1]),
                    card_from_number(stat_data.community_cards[2]),
                    card_from_number(stat_data.community_cards[3]),
                    card_from_number(stat_data.community_cards[4])
                )
            )),
            Blue.bold()
                .paint("|---------------------------------------------------|"),
            Red.bold().paint(format!(
                "| {:^15}| {:15}| {:15} |",
                "Players", "Hand Type", "Hand Cards"
            )),
            Blue.bold()
                .paint("|---------------------------------------------------|"),
            Yellow.bold().paint(format!(
                "| {:^15}| {:15}| {:15} |",
                "P1",
                get_hand(stat_data.player_hands[0]).unwrap(),
                format!(
                    "{} {}",
                    card_from_number(stat_data.player_hand_cards[0][0]),
                    card_from_number(stat_data.player_hand_cards[0][1])
                )
            )),
            Blue.bold()
                .paint("|---------------------------------------------------|"),
            Yellow.bold().paint(format!(
                "| {:^15}| {:15}| {:15} |",
                "P2",
                get_hand(stat_data.player_hands[0]).unwrap(),
                format!(
                    "{} {}",
                    card_from_number(stat_data.player_hand_cards[1][0]),
                    card_from_number(stat_data.player_hand_cards[1][1])
                )
            )),
            Blue.bold()
                .paint("|---------------------------------------------------|"),
            Yellow.bold().paint(format!(
                "| {:^15}| {:15}| {:15} |",
                "P3",
                get_hand(stat_data.player_hands[0]).unwrap(),
                format!(
                    "{} {}",
                    card_from_number(stat_data.player_hand_cards[2][0]),
                    card_from_number(stat_data.player_hand_cards[2][1])
                )
            )),
            Blue.bold()
                .paint("|---------------------------------------------------|"),
            Yellow.bold().paint(format!(
                "| {:^15}| {:15}| {:15} |",
                "P4",
                get_hand(stat_data.player_hands[0]).unwrap(),
                format!(
                    "{} {}",
                    card_from_number(stat_data.player_hand_cards[3][0]),
                    card_from_number(stat_data.player_hand_cards[3][1])
                )
            )),
            Blue.bold()
                .paint("+---------------------------------------------------+"),
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
