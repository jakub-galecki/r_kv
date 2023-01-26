use clap::{Parser, Subcommand};
use env_logger::Env;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    #[arg(short, long, default_value_t = 8080)]
    port: u32,

    #[command(subcommand)]
    cmd: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Get { key: String },
    Set { key: String, value: String },
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let env = Env::default().filter_or("RUST_LOG", "debug");
    env_logger::init_from_env(env);

    let cli = Cli::parse();

    match &cli.cmd {
        Some(Commands::Get { key }) => {
            log::debug!("Received GET with key {}\n", key);
        }
        Some(Commands::Set { key, value }) => {
            log::debug!("Received SET with key {} and value {}\n", key, value);
        }
        None => {}
    }
}
