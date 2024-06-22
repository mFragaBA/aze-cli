use clap::Parser;
use std::error::Error;
use std::path::PathBuf;
use futures_util::{StreamExt, SinkExt}; // Import the required traits
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use aze_lib::utils::Ws_config;
use ansi_term::Colour::{Blue, Green, Red, Yellow};

#[derive(Debug, Clone, Parser)]
pub struct ConnectCmd {
    #[arg(short, long, help = "WebSocket server URL")]
    url: String,
}

impl ConnectCmd {
    pub async fn execute(&self, config_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        // Connect to the WebSocket server
        let (ws_stream, _) = connect_async(&self.url).await?;
        let mut config = Ws_config::load(config_path);
        config.url = Some(self.url.to_string());
        config.save(config_path);
        println!("Connected to the game server at {}", self.url);

        let (mut _write, mut read) = ws_stream.split();

        // Read messages from the server
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => match msg {
                    Message::Text(text) => println!("{} {}", Yellow.bold().paint("Game Update: "), text),
                    _ => (),
                },
                Err(e) => {
                    eprintln!("{}", Red.bold().paint(format!("Error receiving message: {}", e)));
                    break;
                }
            }
        }

        Ok(())
    }
}
