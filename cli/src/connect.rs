use clap::Parser;
use std::error::Error;
use futures_util::{StreamExt, SinkExt}; // Import the required traits
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

#[derive(Debug, Clone, Parser)]
pub struct ConnectCmd {
    #[arg(short, long, help = "WebSocket server URL")]
    url: String,
}

impl ConnectCmd {
    pub async fn execute(&self) -> Result<(), Box<dyn Error>> {
        // Connect to the WebSocket server
        let (ws_stream, _) = connect_async(&self.url).await?;
        println!("Connected to the server at {}", self.url);

        let (mut _write, mut read) = ws_stream.split();

        // Read messages from the server
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => match msg {
                    Message::Text(text) => println!("Received message: {}", text),
                    Message::Binary(bin) => println!("Received binary message: {:?}", bin),
                    _ => (),
                },
                Err(e) => {
                    eprintln!("Error receiving message: {}", e);
                    break;
                }
            }
        }

        Ok(())
    }
}
