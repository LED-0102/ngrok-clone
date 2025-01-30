use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::connect_async;
use futures_util::StreamExt; // To use StreamExt for `next()`

#[tokio_macros::main]
async fn main() {
    let url = "ws://127.0.0.1:8080/v1/ws";

    // Convert the URL into a client request
    let mut req = url.into_client_request().unwrap();

    // Add the custom header
    let headers = req.headers_mut();
    headers.insert("user_id", "hello".parse().unwrap());

    // Establish WebSocket connection
    let (mut ws_stream, response) = match connect_async(req).await {
        Ok((ws_stream, response)) => (ws_stream, response),
        Err(e) => {
            eprintln!("Error connecting to the server: {}", e.to_string());
            return;
        }
    };
    println!("Connected to the server!");

    // Receive messages from the WebSocket stream
    while let Some(message) = ws_stream.next().await {
        match message {
            Ok(msg) => {
                println!("Received: {}", msg.to_text().unwrap());
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        }
    }
}
