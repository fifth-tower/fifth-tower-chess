use crate::page::*;
use chess_core::*;
use leptos::prelude::*;
use leptos_bevy_canvas::prelude::*;

#[component]
pub(crate) fn OnePlayer(class: String) -> impl IntoView {
    let on_ready = move |context: ChessContext| {
        context.state.set(ChessState::Ready);
        context.duplexs.0.send(ChessEvent::ready()).ok();
    };
    let actions = move || {
        view! {
            <div class="justify-center card-actions">
                <div class="join join-horizontal">
                    {move || {
                        let context = expect_context::<ChessContext>();
                        let state = context.state.get();
                        match state {
                            ChessState::Joined => {
                                view! {
                                    <TurnCountDownMenu />
                                    <button
                                        class="btn join-item"
                                        on:click=move |_| on_ready(context.clone())
                                    >
                                        开始
                                    </button>
                                }
                                    .into_any()
                            }
                            ChessState::InGame => {
                                view! {
                                    <RequestButton
                                        request_type=ChessChatData::RequestHe
                                        text="求和".to_string()
                                    />
                                    <RequestButton
                                        request_type=ChessChatData::Lost(ChessLostType::Request)
                                        text="认输".to_string()
                                    />
                                }
                                    .into_any()
                            }
                            ChessState::Lobby | ChessState::Ready => view! {}.into_any(),
                        }
                    }}
                </div>
            </div>
        }
    };

    view! {
        {move || {
            let context = expect_context::<ChessContext>();
            let mut player = context.user_storage.0.get();
            let player = player.get_or_insert_default();

            view! {
                <div class=format!("shadow-sm card bg-base-100 card-sm {}", class)>
                    <figure>
                        <img src=player.avatar() />
                    </figure>
                    <div class="card-body">
                        <p>{player.nickname.clone()}</p>
                        {actions}
                    </div>
                </div>
            }
        }}
    }
}

#[component]
fn RequestButton(request_type: ChessChatData, text: String) -> impl IntoView {
    let context = expect_context::<ChessContext>();
    let on_request = move |context: ChessContext, rt: ChessChatData| {
        context.send_chat(rt);
    };
    view! {
        <button
            class="btn join-item"
            on:click=move |_| on_request(context.clone(), request_type.clone())
        >
            {text}
        </button>
    }
}
#[component]
pub(crate) fn TwoPlayer(class: String) -> impl IntoView {
    let context = expect_context::<ChessContext>();
    view! {
        {move || {
            if context.two_player.get().is_some() {
                let player = context.two_player.get().unwrap();
                context.state.set(ChessState::Joined);

                view! {
                    <div class=format!("shadow-sm card bg-base-100 card-sm {}", class)>
                        <figure>
                            <img src=player.avatar() />
                        </figure>
                        <div class="card-body">
                            <p>{player.nickname}</p>
                        </div>
                    </div>
                }
                    .into_any()
            } else {
                view! { <div /> }.into_any()
            }
        }}
    }
}
