mod connection;

use connection::{Connection, Id, CHANNEL_BUFFER_SIZE};

use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, Sender, Receiver};

pub struct ServerState {
    sender: Sender<String>,
    pub receiver: Receiver<String>,
    connections: Vec<Connection>,
    id_counter: Id,
}

impl ServerState {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(CHANNEL_BUFFER_SIZE);

        ServerState {
            sender,
            receiver,
            connections: vec![],
            id_counter: 0,
        }
    }

    pub fn add_connection(&mut self, stream: TcpStream, address: SocketAddr) -> Id {
        let id = self.id_counter;
        let conn = Connection::new(id, stream, address);

        self.connections.push(conn);
        self.id_counter += 1;

        id
    }

    pub fn run_connection(&mut self, id: Id) -> Result<(), String> {
        if let Some(idx) = self.connections.iter().position(|c| c.id == id) {
            let sender = self.sender.clone();
            self.connections[idx].start(sender)?;
            return Ok(());
        }

        Err("Connection does not exist in server".to_string())
    }

    pub async fn send_message(&self, id: Id, msg: String) -> Result<(), String> {
        if let Some(idx) = self.connections.iter().position(|c| c.id == id) {
            self.connections[idx].send_msg(msg).await?;
            return Ok(());
        }

        Err("Connection does not exist in server".to_string())
    }

    pub async fn send_global_message(&self, msg: String) -> Result<(), String> {
        for c in &self.connections {
            c.send_msg(msg.clone()).await?;
        }
        Ok(())
    }
}
