use bevy::{ecs::event::EventId, prelude::*};

use crate::{ChessChatData, PieceColor};
/// Keeps track of what Leptos event have been imported into Bevy to prevent infinite loops.
#[derive(Resource, Deref, DerefMut)]
pub struct ChessClientEventIds(Vec<EventId<ChessEvent>>);

impl Default for ChessClientEventIds {
    fn default() -> Self {
        Self(Vec::with_capacity(4))
    }
}

#[derive(Clone, Debug)]
pub enum ChessEventType {
    Connected,
    DisConnected,
    Lobby,
    Joined,
    InGame,

    Ready,
    Chat(ChessChatData),
    Turn(PieceColor),
    QuickMatch(String),
}

#[derive(Event, Clone, Debug)]
pub struct ChessEvent {
    pub kind: ChessEventType,
    pub avatar: u16,
    pub nickname: String,
    pub user_id: i32,
}

impl Default for ChessEvent {
    fn default() -> Self {
        Self {
            kind: ChessEventType::Lobby,
            avatar: Default::default(),
            nickname: Default::default(),
            user_id: Default::default(),
        }
    }
}

impl ChessEvent {
    pub fn connected(user_id: i32, nickname: String, avatar: u16) -> Self {
        Self {
            kind: ChessEventType::Connected,
            user_id,
            avatar,
            nickname,
            ..Default::default()
        }
    }

    pub fn lobby() -> Self {
        Self {
            kind: ChessEventType::Lobby,
            ..Default::default()
        }
    }
    pub fn joined() -> Self {
        Self {
            kind: ChessEventType::Joined,
            ..Default::default()
        }
    }
    pub fn ready() -> Self {
        Self {
            kind: ChessEventType::Ready,
            ..Default::default()
        }
    }

    pub fn disconnect() -> Self {
        Self {
            kind: ChessEventType::DisConnected,
            ..Default::default()
        }
    }
    pub fn in_game() -> Self {
        Self {
            kind: ChessEventType::InGame,
            ..Default::default()
        }
    }

    pub fn chat(data: ChessChatData, user_id: i32, nickname: String, avatar: u16) -> Self {
        Self {
            kind: ChessEventType::Chat(data),
            user_id,
            avatar,
            nickname,
            ..Default::default()
        }
    }

    pub fn turn(color: PieceColor) -> Self {
        Self {
            kind: ChessEventType::Turn(color),
            ..Default::default()
        }
    }

    pub fn quick_match(server_url: String) -> Self {
        Self {
            kind: ChessEventType::QuickMatch(server_url),
            ..Default::default()
        }
    }
}
