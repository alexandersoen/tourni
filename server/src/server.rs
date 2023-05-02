mod connection;

pub use connection::{Connection, CHANNEL_BUFFER_SIZE};

use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_util::sync::CancellationToken;

use crate::message::{Id, ClientMessage, ServerMessage};

pub struct ServerState {
    sender: Sender<ClientMessage>,
    pub receiver: Receiver<ClientMessage>,
    connections: Vec<Connection>,
    id_counter: Id,
    cancel_token: CancellationToken,
}

impl ServerState {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(CHANNEL_BUFFER_SIZE);

        ServerState {
            sender,
            receiver,
            connections: vec![],
            cancel_token: CancellationToken::new(),
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
            let cancel_token = self.cancel_token.clone();
            self.connections[idx].start(sender, cancel_token)?;
            return Ok(());
        }

        Err("Connection does not exist in server".to_string())
    }

    pub async fn send_message(&self, id: Id, msg: ServerMessage) -> Result<(), String> {
        if let Some(idx) = self.connections.iter().position(|c| c.id == id) {
            self.connections[idx].send_msg(msg).await?;
            return Ok(());
        }

        Err("Connection does not exist in server".to_string())
    }

    pub async fn send_global_message(&self, msg: ServerMessage) -> Result<(), String> {
        for c in &self.connections {
            c.send_msg(msg.clone()).await?;
        }
        Ok(())
    }

    pub fn shutdown(self) {
        self.cancel_token.cancel();
        for c in self.connections {
            c.shutdown();
        }
        drop(self.sender);
    }
}
