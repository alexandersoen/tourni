use std::collections::HashMap;
use tokio::net::TcpListener;

use game::state::{GameState, Player};

use crate::message::{Id, ServerMessage};
use crate::server::ServerState;

pub struct Round {
    server_state: ServerState,
    player_map: HashMap<Player, Id>,
    game_state: GameState,
}

impl Round {
    pub fn new() -> Self {
        Round {
            server_state: ServerState::new(),
            player_map: HashMap::new(),
            game_state: GameState::new(),
        }
    }

    pub async fn register_player(&mut self, listener: &TcpListener) {
        for &p in Player::iter_all() {
            let (stream, address) = listener.accept().await.unwrap();
            let cur_id = self.server_state.add_connection(stream, address);
            self.server_state.run_connection(cur_id).unwrap();

            let greet_msg = ServerMessage {
                message: format!("Welcome {:?} (O), waiting for players\n", p),
            };

            self.server_state
                .send_message(cur_id, greet_msg)
                .await
                .unwrap();

            self.player_map.insert(p.clone(), cur_id);
        }
    }

    async fn greeting(&self) {
        let greet_msg = ServerMessage {
            message: "Game Starting...\n".to_string(),
        };

        self.server_state
            .send_global_message(greet_msg)
            .await
            .unwrap();
    }

    async fn print_game_state(&self) {
        let game_state_msg = ServerMessage {
            message: format!("{:?}\n", self.game_state),
        };

        self.server_state
            .send_global_message(game_state_msg)
            .await
            .unwrap();
    }

    async fn print_winner(&self, winner: Player) {
        let winning_msg = ServerMessage {
            message: format!("{:?} Wins!\n", winner),
        };

        self.server_state
            .send_global_message(winning_msg)
            .await
            .unwrap();
    }

    pub async fn start_game(&mut self) {
        self.greeting().await;
        self.print_game_state().await;

        loop {
            // Listening for messages
            let msg = self.server_state.receiver.recv().await.unwrap();

            let sender_id = msg.sender;
            if self.player_map[&self.game_state.turn] != sender_id {
                let incorrect_turn_msg = ServerMessage {
                    message: "Not your turn\n".to_string(),
                };
                self.server_state
                    .send_message(sender_id, incorrect_turn_msg)
                    .await
                    .unwrap();
                continue;
            }

            let m: u8 = msg.message.trim().parse().unwrap();

            // Parse a move
            match self.game_state.make_move(m) {
                Ok(()) => self.print_game_state().await,
                _ => {
                    let invalid_move_msg = ServerMessage {
                        message: "Invalid move\n".to_string(),
                    };

                    self.server_state
                        .send_message(sender_id, invalid_move_msg)
                        .await
                        .unwrap()
                }
            };

            // Check if game finished
            match self.game_state.winner() {
                Some(p) => {
                    self.print_winner(p).await;
                    break;
                }
                None => {}
            };
        }
    }
}
