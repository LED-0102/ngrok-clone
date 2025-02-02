use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Ngrok Clone - Client CLI")]
pub struct Cli {
    /// Port of the local service to expose
    #[arg(short, long)]
    pub port: u16,

    /// User ID for the tunnel (optional, auto-generated if not provided)
    #[arg(short, long)]
    pub user_id: Option<String>,
}