use std::error::Error;
use env_logger::Env;
use tokio::net::TcpListener;
use r_kv::handler::process;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = Env::default().filter_or("RUST_LOG", "debug");
    env_logger::init_from_env(env);

    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await?;
    log::info!("Listening on address: {}\n", addr);

    loop {
        let (socket, _) = listener.accept().await?;
        log::info!("Received new message");

        tokio::spawn(async move {
            process(socket).await;
        });
    }
}
