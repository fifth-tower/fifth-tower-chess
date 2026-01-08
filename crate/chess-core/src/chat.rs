use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum ChessChatData {
    /// content,is_one
    Text(String),
    RequestHe,
    Lost(ChessLostType),
    ///reply_for,is_agree
    Reply(Box<ChessChatData>, bool),
    SetTurn(u16),
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum ChessLostType {
    Request,
    Timeout,
}
