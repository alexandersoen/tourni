mod connection;

use connection::{Connection, Id};

use std::net::SocketAddr;
use tokio::net::TcpStream;

pub struct ServerState {
    connections: Vec<Connection>,
    id_counter: connection::Id,
}

impl ServerState {
    pub fn new() -> Self {
        ServerState {
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
            self.connections[idx].start()?;
            return Ok(());
        }

        Err("Connection does not exist in server".to_string())
    }
}
