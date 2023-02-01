use crate::data::Data;
use std::fmt;
use tokio::{io::AsyncReadExt, net::TcpStream};

#[derive(Debug)]
pub enum Command {
    GET(String),
    SET(String, Data),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::GET(key) => write!(f, "GET({})", key),
            Self::SET(key, value) => write!(f, "SET({}, {})", key, value),
        }
    }
}

impl Command {
    pub async fn from_tcp_stream(tcp_stream: &mut TcpStream) -> Option<Command> {
        let mut buffer: [u8; 1024] = [0; 1024];

        let len = tcp_stream.read(&mut buffer).await.unwrap();
        let msg = String::from_utf8_lossy(&buffer[..len]);
        log::debug!("Received following message:  {}", msg);
        let splitted: Vec<&str> = msg.trim().split(" ").collect();
        let r = match splitted[0].to_lowercase().as_str() {
            "get" => Some(Command::GET(splitted[1].to_string())),
            "set" => match Data::new(splitted[2].to_string()) {
                Some(data) => Some(Command::SET(splitted[1].to_string(), data)),
                None => None,
            },
            _ => None,
        };
        r
    }
}
