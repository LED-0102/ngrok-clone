mod client;
mod config;
mod cli;
mod handler;

use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::{Message, Utf8Bytes};
use crate::cli::Cli;
use crate::handler::handle_message;
use config::Config;
use client::connect_to_server;
use tunnel_protocol::message::Message as TunnelMessage;

#[tokio_macros::main]
async fn main() {
    let args = Cli::parse();

    let config = Config::new(args.port, args.user_id);

    let mut handlers = match connect_to_server(&config).await {
        Ok(handlers) => handlers,
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
            return;
        }
    };

    let mut snd = handlers.0;
    let mut recv = handlers.1;

    while let Some(msg) = recv.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let tunnel_msg: TunnelMessage = serde_json::from_str(&text).unwrap();

                let response = handle_message(tunnel_msg).await;

                if let Some(response) = response {
                    let response_text = serde_json::to_string(&response).unwrap();
                    snd.send(Message::Text(Utf8Bytes::from(response_text))).await.unwrap();
                }
            }
            Ok(Message::Binary(bin)) => {
                eprintln!("Received Unexpected binary data: {:?} ({} bytes)", bin, bin.len());
            }
            Ok(Message::Ping(_)) => (),
            Ok(Message::Pong(_)) => (),
            Ok(Message::Close(reason)) => {
                println!("Connection closed: {:?}", reason);
                break;
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
            _ => {}
        }
    }



}
