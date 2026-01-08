use chess_core::{ChessChatData, ChessEvent, ChessEventType, ChessLostType};
use leptos::{context::Provider, prelude::*};

use crate::{
    common::to_turn_label,
    page::{ChessContext, User},
};

#[derive(Clone)]
pub struct ChessChatContext {
    pub messages: RwSignal<Vec<(ChessEvent, ChatMessageSource, bool)>>,
}
impl ChessChatContext {
    pub fn new() -> Self {
        Self {
            messages: RwSignal::<Vec<(ChessEvent, ChatMessageSource, bool)>>::new(Vec::new()),
        }
    }

    pub fn add_message(&self, message: ChessEvent, source: ChatMessageSource) {
        self.messages.update(|data| {
            data.push((message, source, true));
        });
    }

    pub fn set_message_handled(&self, index: usize) {
        self.messages.update(|data| {
            data.get_mut(index).unwrap().2 = false;
        });
    }

    pub fn clear_message(&self) {
        self.messages.update(|data| {
            data.clear();
        });
    }
}

///事件消息来源：接收或发送
#[derive(Copy, Clone, Debug)]
pub enum ChatMessageSource {
    Received,
    Send,
}

#[component]
pub fn ChatPanel() -> impl IntoView {
    let context = expect_context::<ChessContext>().chat;
    view! {
        <div>
            {move || {
                context
                    .messages
                    .get()
                    .iter()
                    .enumerate()
                    .map(|(index, (e, source, need_handle))| {
                        match &e.kind {
                            ChessEventType::Chat(data) => {
                                view! {
                                    <Chat
                                        data=(index, data.clone(), *need_handle)
                                        user_id=e.user_id
                                        nickname=e.nickname.clone()
                                        avatar=e.avatar
                                        source=*source
                                    />
                                }
                                    .into_any()
                            }
                            _ => view! {}.into_any(),
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}

#[component]
fn Chat(
    ///index,data,need_handle
    data: (usize, ChessChatData, bool),
    user_id: i32,
    nickname: String,
    avatar: u16,
    source: ChatMessageSource,
) -> impl IntoView {
    view! {
        <div
            class="chat"
            class=("chat-start", move || matches!(source, ChatMessageSource::Received))
            class=("chat-end", move || matches!(source, ChatMessageSource::Send))
        >
            <div class="chat-image avatar">
                <div class="w-10 rounded-full">
                    <img src=User::to_avatar(avatar) />
                </div>
            </div>
            <Provider value=data.clone()>
                <div class="chat-bubble">
                    {match data.1 {
                        ChessChatData::Text(text) => view! { <TextMessage text /> }.into_any(),
                        ChessChatData::RequestHe => {
                            view! {
                                <RequestHeMessage
                                    nickname=nickname.clone()
                                    source
                                ></RequestHeMessage>
                            }
                                .into_any()
                        }
                        ChessChatData::Lost(tp) => {
                            view! {
                                <LostMessage nickname=nickname.clone() lost_type=tp></LostMessage>
                            }
                                .into_any()
                        }
                        ChessChatData::Reply(_, is_agree) => {
                            view! { <ReplyMessage is_agree /> }.into_any()
                        }
                        ChessChatData::SetTurn(second) => {
                            view! { <SetTurnMessage nickname=nickname.clone() second /> }.into_any()
                        }
                    }}
                </div>
            </Provider>
        </div>
    }
}

#[component]
fn SetTurnMessage(nickname: String, second: u16) -> impl IntoView {
    view! { <div>{nickname}" 设置步时："{to_turn_label(second)}</div> }
}
#[component]
fn TextMessage(text: String) -> impl IntoView {
    view! { <pre>{text}</pre> }
}

#[component]
fn RequestHeMessage(nickname: String, source: ChatMessageSource) -> impl IntoView {
    let chat_data = expect_context::<(usize, ChessChatData, bool)>();
    view! {
        <div>{nickname}" 请求和棋，是否同意？"</div>
        {(chat_data.2 && matches!(source, ChatMessageSource::Received))
            .then(|| {
                view! {
                    <div class="join">
                        <ReplyButton is_agree=true text="同意".to_string() />
                        <ReplyButton is_agree=false text="拒绝".to_string() />
                    </div>
                }
            })}
    }
}
#[component]
fn LostMessage(nickname: String, lost_type: ChessLostType) -> impl IntoView {
    let text = match lost_type {
        ChessLostType::Request => "认输了",
        ChessLostType::Timeout => "超时了",
    };
    view! { <div>{format!("{} {}", nickname, text)}</div> }
}

#[component]
fn ReplyButton(is_agree: bool, text: String) -> impl IntoView {
    let context = expect_context::<ChessContext>();
    let chat_data = expect_context::<(usize, ChessChatData, bool)>();
    let on_reply = move |context: ChessContext, is_agree: bool| {
        context.chat.set_message_handled(chat_data.0);
        context.send_chat(ChessChatData::Reply(
            Box::new(chat_data.1.clone()),
            is_agree,
        ));
    };
    view! {
        <button
            class="btn join-item btn-primary btn-sm btn-soft"
            on:click=move |_| on_reply(context.clone(), is_agree)
        >
            {text}
        </button>
    }
}

#[component]
fn ReplyMessage(is_agree: bool) -> impl IntoView {
    view! { <div>{is_agree.then(|| "同意和棋").unwrap_or("拒绝和棋")}</div> }
}
