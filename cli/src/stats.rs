use std::path::PathBuf;

use ansi_term::Colour::{Blue, Green, Red, Yellow};
use aze_lib::{
    client::{create_aze_client, AzeClient},
    constants::PLAYER_FILE_PATH,
    utils::{card_from_number, get_stats, Player, Ws_config},
};
use clap::Parser;
use dialoguer::Input;
use miden_objects::accounts::AccountId;
use std::path::Path;

#[derive(Parser, Debug, Clone)]
pub struct StatsCmd;

impl StatsCmd {
    pub async fn execute(&self, ws_config: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let gameid: u64 = get_id();
        let mut client: AzeClient = create_aze_client();
        let game_account_id = AccountId::try_from(gameid).unwrap();
        // let game_account = client.get_account(game_account_id).unwrap().0; // giving error
        let ws_url = Ws_config::load(ws_config).url.unwrap();
        let stat_data: aze_lib::utils::StatResponse =
            get_stats(game_account_id.to_string(), ws_url).await?;

        let community_cards: Vec<u64> =
            get_community_cards(stat_data.current_state, stat_data.community_cards);
        let poker_table = format!(
            "{}\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}",
            Blue.bold()
                .paint("+---------------------------------------------------+"),
            Red.bold()
                .paint("|                   POKER TABLE                     |"),
            Blue.bold()
                .paint("|---------------------------------------------------|"),
            Blue.bold()
                .paint("|                 COMMUNITY CARDS                   |"),
            Blue.bold().paint(format!(
                "|------ {:^37} ------|",
                format!(
                    "{:4} {:4} {:4} {:4} {:4}",
                    card_from_number(community_cards[0]),
                    card_from_number(community_cards[1]),
                    card_from_number(community_cards[2]),
                    card_from_number(community_cards[3]),
                    card_from_number(community_cards[4])
                )
            )),
            Blue.bold()
                .paint("|---------------------------------------------------|"),
            Yellow.bold().paint(format!(
                "|     {:^20} {:^20}     |",
                if stat_data.has_folded[0] == 0 {
                    format!("P1")
                } else {
                    format!("P1 [Folded]")
                },
                if stat_data.has_folded[1] == 0 {
                    format!("P2")
                } else {
                    format!("P2 [Folded]")
                },
            )),
            Yellow.bold().paint(format!(
                "|     {:^20} {:^20}     |",
                format!("Balance: {}", stat_data.player_balances[0]),
                format!("Balance: {}", stat_data.player_balances[1])
            )),
            Blue.bold()
                .paint("|---------------------------------------------------|"),
            Yellow.bold().paint(format!(
                "|          {:^31}          |",
                format!("POT VALUE: {}", stat_data.pot_value)
            )),
            Blue.bold()
                .paint("|---------------------------------------------------|"),
            Yellow.bold().paint(format!(
                "|     {:^20} {:^20}     |",
                if stat_data.has_folded[2] == 0 {
                    format!("P3")
                } else {
                    format!("P3 [Folded]")
                },
                if stat_data.has_folded[3] == 0 {
                    format!("P4")
                } else {
                    format!("P4 [Folded]")
                }
            )),
            Yellow.bold().paint(format!(
                "|     {:^20} {:^20}     |",
                format!("Balance: {}", stat_data.player_balances[2]),
                format!("Balance: {}", stat_data.player_balances[3])
            )),
            Blue.bold()
                .paint("|---------------------------------------------------|"),
            Yellow.bold().paint(format!(
                "|          {:^31}          |",
                format!("Turn: {}", stat_data.current_player)
            )),
            Blue.bold()
                .paint("+---------------------------------------------------+")
        );

        println!("{}", poker_table);
        Ok(())
    }
}

fn get_id() -> u64 {
    let path = Path::new(PLAYER_FILE_PATH);
    let player: Player =
        toml::from_str(&std::fs::read_to_string(path).expect("Failed to read Player.toml"))
            .expect("Failed to deserialize player data");
    let game_id = player.game_id().unwrap();

    game_id
}

fn get_community_cards(phase: u64, community_cards: Vec<u64>) -> Vec<u64> {
    match phase {
        1 => {
            vec![
                community_cards[0],
                community_cards[1],
                community_cards[2],
                0,
                0,
            ]
        }
        2 => {
            vec![
                community_cards[0],
                community_cards[1],
                community_cards[2],
                community_cards[3],
                0,
            ]
        }
        3 => {
            vec![
                community_cards[0],
                community_cards[1],
                community_cards[2],
                community_cards[3],
                community_cards[4],
            ]
        }
        4 => {
            vec![
                community_cards[0],
                community_cards[1],
                community_cards[2],
                community_cards[3],
                community_cards[4],
            ]
        }
        _ => vec![0, 0, 0, 0, 0],
    }
}
