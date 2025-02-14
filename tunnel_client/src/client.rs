use crate::config::Config;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use futures_util::{StreamExt};
use reqwest::Error;
use reqwest::header::HeaderValue;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::handshake::client::Response;

/// WebSocket type alias
type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WsSender = futures_util::stream::SplitSink<WsStream, Message>;
type WsReceiver = futures_util::stream::SplitStream<WsStream>;

/// Establishes WebSocket connection to the tunnel server
pub async fn connect_to_server(config: &Config) -> Result<(WsSender, WsReceiver), tokio_tungstenite::tungstenite::Error> {
    let server_url = &config.server_url;
    let mut req = server_url.into_client_request()?;

    let mut headers = req.headers_mut().insert("user_id", HeaderValue::from_str(&config.user_id)?);


    println!("user_id header: {:?}", &config.user_id);

    println!("user_id header: {:?}", req.headers().get("user_id"));

    println!("Connecting to tunnel server at: {}", server_url);

    let (ws_stream, _) = match connect_async(req).await {
        Ok(s) => {s}
        Err(e) => {
            return Err(e);
        }
    };

    println!("Connected to tunnel server as user: {}", config.user_id);

    let (write, read) = ws_stream.split();

    Ok((write, read))
}
