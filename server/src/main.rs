mod round;
mod server;
mod message;

use round::Round;
use server::ServerState;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let server_state = ServerState::new();
    let listener = TcpListener::bind("127.0.0.1:9876").await.unwrap();
    let mut round = Round::new();

    round.register_player(&listener).await;
    round.start_game().await;


    server_state.shutdown();
    /*
    loop {
        tokio::select! {
            // Listening for new connections
            new_conn = listener.accept() => {
                let (stream, address) = new_conn.unwrap();
                let id = server_state.add_connection(stream, address);
                server_state.run_connection(id).unwrap();
            },
            // Listening for messages
            msg = server_state.receiver.recv() => {
                server_state.send_global_message(msg.unwrap()).await.unwrap();
            }
        }
    }
    */
}
