use chess_core::{ChessChatData, ChessEvent, PieceColor};
use codee::{binary::BincodeSerdeCodec, string::Base64};
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;
use leptos_use::storage::use_local_storage;

use crate::page::User;

use super::{ChatMessageSource, ChessChatContext};

#[derive(Clone)]
pub struct ChessContext {
    pub two_player: RwSignal<Option<User>>,
    pub user_storage: (Signal<Option<User>>, WriteSignal<Option<User>>),
    pub state: RwSignal<ChessState>,
    pub duplexs: (LeptosEventDuplex<ChessEvent>, BevyEventDuplex<ChessEvent>),
    pub chat: ChessChatContext,
    ///cur_turn, turn_second, can_set
    pub turn: RwSignal<(PieceColor, u16, ChessSetTurnStatus)>,
}

impl ChessContext {
    pub fn new() -> Self {
        let (read_user, write_user, _) =
            use_local_storage::<Option<User>, Base64<BincodeSerdeCodec>>("user");
        Self {
            two_player: RwSignal::<Option<User>>::new(None),
            user_storage: (read_user, write_user),
            state: RwSignal::<ChessState>::new(ChessState::default()),
            duplexs: event_duplex::<ChessEvent>(),
            chat: ChessChatContext::new(),
            turn: RwSignal::new((PieceColor::UnSet, 60, ChessSetTurnStatus::UnSet)),
        }
    }
    pub fn send_chat(&self, chat_data: ChessChatData) {
        let user = self.user_storage.0.get_untracked();
        if let Some(user) = user {
            let event = ChessEvent::chat(chat_data, user.user_id, user.nickname, user.avatar);
            self.duplexs.0.send(event.clone()).ok();
            self.chat.add_message(event, ChatMessageSource::Send);
        }
    }
    pub fn send_local_chat(&self, chat_data: ChessChatData) {
        let user = self.user_storage.0.get_untracked();
        if let Some(user) = user {
            let event = ChessEvent::chat(chat_data, user.user_id, user.nickname, user.avatar);
            self.chat.add_message(event, ChatMessageSource::Send);
        }
    }

    pub fn send_event(&self, event: ChessEvent) {
        self.duplexs.0.send(event).ok();
    }

    pub fn set_turn_color(&self, color: PieceColor) {
        self.turn.update(|t| {
            t.0 = color;
        });
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Copy)]
pub enum ChessState {
    #[default]
    Lobby,
    Joined,
    Ready,
    InGame,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Copy)]
pub enum ChessSetTurnStatus {
    #[default]
    UnSet,
    Setted,
    NoSet,
}
