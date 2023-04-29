mod server;

use server::ServerState;

use game::state::GameState;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let mut server_state = ServerState::new();
    let listener = TcpListener::bind("127.0.0.1:9876").await.unwrap();
    let mut game_state = GameState::new();

    // Player 1
    let (stream, address) = listener.accept().await.unwrap();
    let p1_id = server_state.add_connection(stream, address);
    server_state.run_connection(p1_id).unwrap();
    server_state
        .send_message(
            p1_id,
            "Welcome Player 1 (O), waiting for players\n".to_string(),
        )
        .await
        .unwrap();

    // Player 2
    let (stream, address) = listener.accept().await.unwrap();
    let p2_id = server_state.add_connection(stream, address);
    server_state.run_connection(p2_id).unwrap();
    server_state
        .send_message(
            p2_id,
            "Welcome Player 2 (X), waiting for players\n".to_string(),
        )
        .await
        .unwrap();

    server_state
        .send_global_message("Game Starting...\n".to_string())
        .await
        .unwrap();
    server_state
        .send_global_message(format!("{:?}\n", game_state))
        .await
        .unwrap();

    loop {
        // Listening for messages
        let msg = server_state.receiver.recv().await.unwrap();
        let m: u8 = msg.trim().parse().unwrap();

        // Parse a move
        match game_state.make_move(m) {
            Ok(()) => server_state
                .send_global_message(format!("{:?}\n", game_state))
                .await
                .unwrap(),
            _ => server_state
                .send_global_message("Invalid move\n".to_string())
                .await
                .unwrap(),
        };

        // Check if game finished
        match game_state.winner() {
            Some(p) => {
                server_state
                    .send_global_message(format!("{:?} Wins!\n", p))
                    .await
                    .unwrap();
                break;
            }
            None => {}
        };
    }

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
