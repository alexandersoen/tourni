mod server;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let mut server_state = server::ServerState::new();
    let listener = TcpListener::bind("localhost:9876").await.unwrap();

    loop {
        tokio::select! {
            // Listening for new connections
            new_conn = listener.accept() => {
                let (stream, address) = new_conn.unwrap();
                let id = server_state.add_connection(stream, address);
                server_state.run_connection(id).unwrap();
            }
        }
    }
}
