use std::ops::Mul;

use bevy::prelude::*;
use bevy_matchbox::prelude::PeerId;
use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::ChessChatData;

#[derive(Parser, Resource, Debug, Clone)]
pub struct Opt {
    #[arg(long, default_value = "0")]
    pub user_id: i32,
    #[arg(long, default_value = "0")]
    pub avatar: u16,
    #[arg(long, default_value = "游客")]
    pub nickname: String,
    #[arg(long, default_value = "60")]
    pub cell_width: f32,
    #[arg(long, default_value = "ws://localhost:3536/chess{}?next=2")]
    pub server_url: String,
    #[arg(long, default_value = "2")]
    pub play_num: usize,
    #[arg(long, default_value = "000000")]
    pub clear_color: String,
    #[arg(long, default_value = "777777")]
    pub board_color: String,
    #[arg(long, default_value = "1")]
    pub board_line_width: f32,
}

impl Opt {
    pub fn clear_color(&self) -> Srgba {
        Srgba::hex(self.clear_color.clone()).unwrap()
    }
    pub fn board_color(&self) -> Srgba {
        Srgba::hex(self.board_color.clone()).unwrap()
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Player {
    pub user_id: i32,
    pub avatar: u16,
    pub nickname: String,
    pub peer_id: PeerId,
}

#[derive(Default, Clone, Copy, Component, PartialEq, Deserialize, Serialize, Debug)]
pub enum PieceColor {
    #[default]
    UnSet,
    Red,
    Black,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Init,
    Lobby,
    Joined,
    Ready,
    InGame,
}

#[derive(Default, Clone, Copy, Component, PartialEq, Debug, Deserialize, Serialize)]
pub struct Pos(pub f32, pub f32);

impl Pos {
    pub fn to_vec3(self) -> Vec3 {
        Vec3::new(self.0, self.1, 0.)
    }
}

impl Mul<Vec2> for Pos {
    type Output = Self;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Pos(self.0 * rhs.x, self.1 * rhs.y)
    }
}

#[derive(Resource, Default)]
pub struct GameState {
    pub one_player: Option<Player>,
    pub two_player: Option<Player>,

    pub one_ready: bool,
    pub two_ready: bool,

    pub one_color: PieceColor,
    pub two_color: PieceColor,

    pub current_pos: Option<Pos>,
    pub is_turn: bool,
}

impl GameState {
    pub fn reset(&mut self, with_two: bool) {
        if !with_two {
            self.two_player = None;
            self.one_color = PieceColor::UnSet;
            self.two_color = PieceColor::UnSet;
        } else {
            (self.one_color, self.two_color) = (self.two_color, self.one_color);
        }
        self.one_ready = false;
        self.two_ready = false;
        self.current_pos = None;
        self.is_turn = false;
    }
}

#[derive(Clone, Component, Deserialize, Serialize, Debug)]
pub enum Action {
    JoinLobby(Player),
    LeaveLobby,

    Ready,
    ///(piece,from,to,is_eat, is_jianged, is_end)
    MovePiece(Pos, Pos, bool, bool, bool),
    RemovePiece,
    Winner(PieceColor),
    Chat(ChessChatData),
}

#[derive(Clone, Component, Deserialize, Serialize, Debug)]
pub struct Packet(pub Action);
