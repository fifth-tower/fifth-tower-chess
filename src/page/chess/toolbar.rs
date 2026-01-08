use chess_core::ChessChatData;
use leptos::{logging::log, prelude::*};
use leptos_router::hooks::use_query_map;
use leptos_use::{use_clipboard, use_window, UseClipboardReturn};

use super::ChessContext;

#[component]
pub(crate) fn CopyInviteUrlButton(class: String) -> impl IntoView {
    let context = expect_context::<ChessContext>();
    let params = use_query_map();
    let room = params.read_untracked().get("rid").unwrap_or_default();
    let UseClipboardReturn { copy, copied, .. } = use_clipboard();
    Effect::new(move |_| {
        copied.get().then(|| {
            log!("copied");
            context.send_local_chat(ChessChatData::Text("邀请好友链接已复制".to_string()))
        });
    });
    if room.len() > 0 {
        view! {
            <button
                class=format!("btn btn-ghost {}", class)
                on:click=move |_| {
                    let loc = use_window()
                        .as_ref()
                        .map(|w| w.location().href().unwrap_or_default());
                    if let Some(loc) = loc {
                        copy(loc.as_str());
                    }
                }
            >
                复制邀请好友链接
            </button>
        }
        .into_any()
    } else {
        return view! {}.into_any();
    }
}
