// Define a player message with the publicKey bind to it
pub struct PlayerMsg {
    pub action: PlayerAction,
}

#[derive(Clone)]
pub enum PlayerAction {
    MoveRight,
    MoveLeft,
    MoveUp,
    MoveDown,
}
