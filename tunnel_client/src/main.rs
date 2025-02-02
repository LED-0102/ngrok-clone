mod client;
mod config;
mod cli;

use crate::cli::Cli;
use config::Config;
use client::connect_to_server;

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let config = Config::new(args.port, args.user_id);

    match connect_to_server(&config).await {
        Ok(_) => println!("Connection closed."),
        Err(e) => eprintln!("Error: {:?}", e.to_string()),
    }
}
