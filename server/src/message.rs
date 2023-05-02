pub type Id = u64;

#[derive(Debug)]
pub struct ClientMessage {
    pub sender: Id,
    pub message: String,
}

#[derive(Clone, Debug)]
pub struct ServerMessage {
    pub message: String,
}
