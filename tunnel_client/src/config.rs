use dotenvy::dotenv;
use std::env;
use uuid::Uuid;

pub struct Config {
    pub server_url: String,
    pub local_port: u16,
    pub user_id: String,
}

impl Config {
    pub fn new(local_port: u16, user_id: Option<String>) -> Self {
        let _ = dotenv();

        let server_url = env::var("TUNNEL_SERVER_URL")
            .unwrap_or_else(|_| "ws://localhost:8080".to_string());

        let user_id = user_id.unwrap_or_else(|| {
            let generated_id = Uuid::new_v4().to_string();
            println!("Generated user_id: {}", generated_id);
            generated_id
        });

        Self {
            server_url,
            local_port,
            user_id,
        }
    }
}
