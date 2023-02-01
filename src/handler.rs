use crate::{command::Command, database::Db};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn process(db: Arc<Mutex<Db>>, mut socket: TcpStream) -> Result<(), std::io::Error> {
    log::debug!("Processing incoming message");
    let cmd = Command::from_tcp_stream(&mut socket).await.unwrap();
    log::debug!("Parsed command: {}", cmd);
    match cmd {
        Command::GET(key) => {
            let arc = db.lock().await;
            let data = arc.get(key.as_str()).unwrap();
            let res = format!("{}", data);
            socket.write_all(res.as_bytes()).await
        }

        Command::SET(key, value) => match db.lock().await.set(key.as_str(), value) {
            Some(v) => {
                let res = format!("{}", v);
                socket.write_all(res.as_bytes()).await
            }
            None => socket.write_all("OK".as_bytes()).await,
        },
    }
}
