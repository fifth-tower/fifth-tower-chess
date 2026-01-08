use super::*;
use crate::page::*;
use chess_core::*;
use leptos::{logging::log, prelude::*};

#[component]
pub(crate) fn ChessReceivedEvent() -> impl IntoView {
    let context = expect_context::<ChessContext>();
    let received = move || {
        if let Some(e) = context.duplexs.0.get() {
            // log!("received event: {:?}", e);
            let e_clone = e.clone();
            match e.kind {
                ChessEventType::Lobby => {
                    context.two_player.set(None);
                    context.state.set(ChessState::Lobby);
                }
                ChessEventType::Connected => {
                    let player = User {
                        user_id: e.user_id,
                        nickname: e.nickname.clone(),
                        avatar: e.avatar,
                    };
                    context.two_player.set(Some(player));
                }
                ChessEventType::DisConnected => {
                    context.two_player.set(None);
                    context.state.set(ChessState::Lobby);
                }
                ChessEventType::InGame | ChessEventType::Ready => {
                    context.state.set(ChessState::InGame);
                    context.set_turn_color(PieceColor::Red);
                }
                ChessEventType::Joined => {
                    context.state.set(ChessState::Joined);
                }
                ChessEventType::Turn(color) => {
                    context.set_turn_color(color);
                }
                ChessEventType::Chat(data) => {
                    match data {
                        ChessChatData::Lost(_) => context.state.set(ChessState::Joined),
                        ChessChatData::SetTurn(second) => context.turn.update(|t| {
                            if t.2 == ChessSetTurnStatus::Setted {
                                if t.1 != second {
                                    t.1 = second.max(t.1);
                                    if t.1 == second {
                                        //对方大，以对面为准
                                        t.2 = ChessSetTurnStatus::NoSet;
                                    };
                                }
                            } else {
                                t.1 = second;
                                t.2 = ChessSetTurnStatus::NoSet;
                            }
                        }),
                        _ => {}
                    };
                    context
                        .chat
                        .add_message(e_clone.clone(), ChatMessageSource::Received);
                }
                _ => {}
            }
        }
    };
    Effect::new(received);
    view! {}
}
