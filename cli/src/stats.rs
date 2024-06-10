use ansi_term::Colour::{Blue, Green, Red, Yellow};
use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct StatsCmd;

impl StatsCmd {
    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let pot_value = 1500;
        let player_amounts = vec![3000, 2500, 4000, 3500];
        let current_player = 2;

        let poker_table = format!(
            "{}\n\
             {}\n\
             {}\n\
             {}\n\
             {} {} {} {} {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}",
            Blue.bold().paint("+-----------------------------+"),
            Blue.bold().paint("|        Poker Table          |"),
            Blue.bold().paint("+-----------------------------+"),
            Blue.bold().paint("|        Community Cards      |"),
            Blue.bold().paint("|    "),Red.bold().paint("  10♠ "), Green.bold().paint("  J♣ "), Red.bold().paint("  Q♥ "),Blue.bold().paint("     |"),
            Blue.bold().paint("+-----------------------------+"),
            Yellow.bold().paint(format!("|   P1{}                P2{}    |",if current_player == 1 { "*" } else { " " }, if current_player == 2 { "*" } else { " " })),
            Yellow.bold().paint("|   1000               1000   |"),
            Blue.bold().paint("+-----------------------------+"),
            Yellow.bold().paint("|        Pot: 1500            |"),
            Blue.bold().paint("+-----------------------------+"),
            Yellow.bold().paint(format!("|   P3{}                P4{}    |",if current_player == 3 { "*" } else { " " }, if current_player == 4 { "*" } else { " " })),
            Yellow.bold().paint("|   1000               1000   |"),
            Blue.bold().paint("+-----------------------------+"),
        );

        println!("{}", poker_table);

        Ok(())
    }
}

fn main() {
    let stats_cmd = StatsCmd {};
    stats_cmd.execute().unwrap();
}
