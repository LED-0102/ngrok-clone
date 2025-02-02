use crate::config::Config;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use futures_util::{StreamExt};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

/// WebSocket type alias
type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WsSender = futures_util::stream::SplitSink<WsStream, Message>;
type WsReceiver = futures_util::stream::SplitStream<WsStream>;

/// Establishes WebSocket connection to the tunnel server
pub async fn connect_to_server(config: &Config) -> Result<(WsSender, WsReceiver), Box<dyn std::error::Error>> {
    let server_url = &config.server_url;
    let mut req = server_url.into_client_request()?;

    let mut headers = req.headers_mut();

    headers.insert("user_id", config.user_id.parse().unwrap());

    println!("Connecting to tunnel server at: {}", server_url);

    let (ws_stream, _) = connect_async(server_url).await?;

    println!("Connected to tunnel server as user: {}", config.user_id);

    let (write, read) = ws_stream.split();

    Ok((write, read))
}
