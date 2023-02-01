use env_logger::Env;
use r_kv::database::Db;
use r_kv::handler::process;
use std::error::Error;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = Env::default().filter_or("RUST_LOG", "debug");
    env_logger::init_from_env(env);

    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await?;
    log::debug!("Listening on address: {}\n", addr);
    let db = Arc::new(Mutex::new(Db::new()));

    loop {
        let (socket, _) = listener.accept().await?;
        log::debug!("Received new message");
        let d = db.clone();

        tokio::spawn(async move {
            let _ = process(d, socket).await;
        });
    }
}
