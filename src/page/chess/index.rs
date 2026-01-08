use super::*;
use crate::{common::Game, page::*};
use chess_core::*;
use clap::Parser;
use leptos::{context::Provider, prelude::*};
use leptos_bevy_canvas::prelude::*;
use leptos_router::hooks::use_query_map;

#[component]
pub fn Chess() -> impl IntoView {
    view! {
        <div>
            <ChessBoard />
        </div>
    }
}

#[component]
fn ChessBoard() -> impl IntoView {
    let context = ChessContext::new();
    let opt = get_opt(&context);
    view! {
        <Provider value=context.clone()>
            <div class="flex gap-4 justify-center">
                <div class="flex flex-col flex-initial justify-between">
                    <TwoPlayer class="w-64".to_string() />
                    <TurnCountDown class="w-64".to_string() />
                    <OnePlayer class="w-64".to_string() />
                </div>
                <div class="flex-none">
                    <BevyCanvas init=move || { init_chess(opt, context.duplexs.1) } {..} />
                </div>
                <div class="overflow-y-auto flex-1 scroll-smooth max-h-[600px]">
                    <ChatPanel />
                </div>
            </div>
            <div class="flex gap-4 items-end mt-8">
                <ChatInput class="".to_string()></ChatInput>
                <CopyInviteUrlButton class="".to_string() />
            </div>
            <ChessReceivedEvent />
        </Provider>
    }
}

fn get_opt(context: &ChessContext) -> Opt {
    let (player_read, player_write) = context.user_storage;
    let mut opt = Opt::parse();
    let mut player = player_read.get_untracked();

    let player = player.get_or_insert_with(|| {
        let u = User::default();
        player_write.set(Some(u.clone()));
        u
    });
    opt.avatar = player.avatar;
    opt.nickname = player.nickname.clone();
    opt.user_id = player.user_id;

    opt.clear_color = "#f8fafc".to_string();
    opt.board_color = "#adb3bf".to_string();

    let params = use_query_map();
    let room = params.read_untracked().get("rid").unwrap_or_default();
    opt.server_url = Game::Chess.get_server_url(room);

    opt
}
