use futures_util::{SinkExt, StreamExt};
use get_if_addrs::get_if_addrs;
use log::{error, info};
use miden_objects::accounts::AccountId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tokio_tungstenite::tungstenite::protocol::Message as TungsteniteMessage;
use warp::hyper::StatusCode;
use warp::ws::Ws;
use warp::Filter;

use crate::client::{create_aze_client, AzeClient};
use crate::utils::Ws_config;

type Peers = Arc<RwLock<HashMap<String, broadcast::Sender<TungsteniteMessage>>>>;

#[derive(Deserialize)]
struct PublishRequest {
    game_id: String,
    event: String,
}

#[derive(Deserialize)]
struct StatRequest {
    game_id: String
}

#[derive(Serialize)]
struct StatResponse {
    pub community_cards: Vec<u64>,
    pub player_balances: Vec<u64>,
    pub current_player: u64,
    pub pot_value: u64,
}

pub fn start_wss(game_id: String, ws_config_path: &PathBuf) -> Option<String> {
    // Use localhost IP for debugging
    let ip: [u8; 4] = get_ipv4_bytes().unwrap();
    let port = 12044;
    let ws_url = format!(
        "ws://{}.{}.{}.{}:{}/ws/{}",
        ip[0], ip[1], ip[2], ip[3], port, game_id
    );

    // Spawn the server task in the background
    tokio::spawn(async move {
        let peers: Peers = Arc::new(RwLock::new(HashMap::new()));
        let peers_filter = warp::any().map(move || peers.clone());

        let ws_route = warp::path("ws")
            .and(warp::path::param())
            .and(warp::ws())
            .and(peers_filter.clone())
            .and_then(ws_handler);

        let publish_route = warp::path("publish")
            .and(warp::post())
            .and(warp::body::json())
            .and(peers_filter.clone())
            .and_then(publish_handler);

        let stats_route = warp::path("stats")
            .and(warp::post())
            .and(warp::body::json())
            .and_then(stat_handler);

        let routes = ws_route
            .or(publish_route)
            .or(stats_route)
            .with(warp::log("broadcast_server"));

        info!(
            "Starting WebSocket server at {}.{}.{}.{}:{}",
            ip[0], ip[1], ip[2], ip[3], port
        );
        let ip_addr: IpAddr = ip.into();
        warp::serve(routes).run((ip_addr, port)).await;
    });

    // Return the WebSocket URL
    let mut ws_config = Ws_config::load(ws_config_path);
    ws_config.url = Some(ws_url.to_string());
    ws_config.save(ws_config_path);
    Some(ws_url)
}

async fn ws_handler(
    game_id: String,
    ws: Ws,
    peers: Peers,
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(ws.on_upgrade(move |socket| handle_websocket(socket, game_id, peers)))
}

async fn handle_websocket(socket: warp::ws::WebSocket, game_id: String, peers: Peers) {
    let (mut ws_sender, mut ws_receiver) = socket.split();
    let tx = {
        let mut peers = peers.write().await;
        peers
            .entry(game_id.clone())
            .or_insert_with(|| {
                let (tx, _rx) = broadcast::channel(100);
                tx
            })
            .clone()
    };
    let mut rx = tx.subscribe();

    // receive messages from the WebSocket and broadcast them
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        while let Some(result) = ws_receiver.next().await {
            match result {
                Ok(msg) => {
                    if msg.is_text() || msg.is_binary() {
                        let msg = convert_warp_message_to_tungstenite(msg);
                        if let Err(e) = tx_clone.send(msg) {
                            error!("Error broadcasting message: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Error receiving message: {}", e);
                    break;
                }
            }
        }
    });

    // send broadcast messages to the WebSocket
    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let msg = convert_tungstenite_message_to_warp(msg);
            if ws_sender.send(msg).await.is_err() {
                error!("Error sending message to client");
                break;
            }
        }
    });

    info!("WebSocket connection established for game_id: {}", game_id);
}

async fn publish_handler(
    body: PublishRequest,
    peers: Peers,
) -> Result<impl warp::Reply, warp::Rejection> {
    let peers = peers.read().await;
    if let Some(tx) = peers.get(&body.game_id) {
        let msg = TungsteniteMessage::Text(body.event);
        if let Err(e) = tx.send(msg) {
            error!("Error broadcasting event: {}", e);
        }
    } else {
        return Ok(warp::reply::with_status(
            "Game ID not found",
            StatusCode::NOT_FOUND,
        ));
    }

    Ok(warp::reply::with_status("Event published", StatusCode::OK))
}

async fn stat_handler(body: StatRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let game_id = body.game_id;
    let mut client: AzeClient = create_aze_client();
    let game_account_id = AccountId::from_hex(&game_id).unwrap();
    let game_account = client.get_account(game_account_id).unwrap().0;
    //slot values
    let CURRENT_TURN_PLAYER_SLOT: u8 = 60;
    let CURRENT_TURN_PLAYER_ID = game_account
        .storage()
        .get_item(CURRENT_TURN_PLAYER_SLOT)
        .as_elements()[0]
        .as_int();
    let POT_VALUE: u8 = 60;
    let COMMUNITY_CARDS: u8 = 76;
    let P1_BALANCE: u8 = 68;
    let NUM_PLAYERS: u8 = 57;

    let pot_value = game_account.storage().get_item(POT_VALUE).as_elements()[0].as_int();

    let mut player_balances: Vec<u64> = vec![];
    let num_players = game_account.storage().get_item(NUM_PLAYERS).as_elements()[0].as_int();
    for i in 0..num_players {
        let SLOT_VALUE = P1_BALANCE + (i * 13) as u8;
        player_balances.push(game_account.storage().get_item(SLOT_VALUE).as_elements()[0].as_int());
    }
    let community_cards = game_account
        .storage()
        .get_item(COMMUNITY_CARDS)
        .as_elements().to_vec();
    let community_cards_int: Vec<u64> = community_cards.iter().map(|n| n.as_int()).collect();

    let current_player = game_account
        .storage()
        .get_item(CURRENT_TURN_PLAYER_ID as u8)
        .as_elements()[0]
        .as_int();

    Ok(warp::reply::json(&StatResponse{community_cards: community_cards_int, player_balances, current_player, pot_value}))
}

fn convert_warp_message_to_tungstenite(msg: warp::ws::Message) -> TungsteniteMessage {
    if msg.is_text() {
        TungsteniteMessage::Text(msg.to_str().unwrap().to_string())
    } else if msg.is_binary() {
        TungsteniteMessage::Binary(msg.as_bytes().to_vec())
    } else {
        TungsteniteMessage::Ping(vec![])
    }
}

fn convert_tungstenite_message_to_warp(msg: TungsteniteMessage) -> warp::ws::Message {
    match msg {
        TungsteniteMessage::Text(text) => warp::ws::Message::text(text),
        TungsteniteMessage::Binary(bin) => warp::ws::Message::binary(bin),
        TungsteniteMessage::Ping(ping) => warp::ws::Message::ping(ping),
        TungsteniteMessage::Pong(pong) => warp::ws::Message::pong(pong),
        TungsteniteMessage::Close(_) => warp::ws::Message::close(),
        _ => warp::ws::Message::binary(vec![]),
    }
}

fn get_ipv4_bytes() -> Option<[u8; 4]> {
    let interfaces = get_if_addrs().ok()?;

    for iface in interfaces {
        if !iface.is_loopback() {
            if let IpAddr::V4(ipv4) = iface.addr.ip() {
                return Some(ipv4.octets());
            }
        }
    }
    None
}