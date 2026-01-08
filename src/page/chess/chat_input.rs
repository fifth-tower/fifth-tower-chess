use chess_core::ChessChatData;
use leptos::{html::Textarea, prelude::*};
use leptos_hotkeys::*;

use crate::page::{ChessContext, ChessState};

#[component]
pub fn ChatInput(class: String) -> impl IntoView {
    let context = expect_context::<ChessContext>();
    let message = RwSignal::new("".to_string());
    let node_ref = NodeRef::<Textarea>::new();

    provide_hotkeys_context(node_ref, false, scopes!("*"));
    use_hotkeys_ref!((node_ref,"alt+keyS", "*") => move |_| {
        context.clone().send_chat(ChessChatData::Text(message.get_untracked()));
        message.set("".to_string());
    });

    let context = expect_context::<ChessContext>();
    let html = move |context: ChessContext| {
        if matches!(context.state.get(), ChessState::Lobby) {
            return view! {}.into_any();
        } else {
            view! {
                <div class=format!("flex flex-wrap gap-4 justify-start items-end {}", class)>
                    <MessageTextarea
                        node_ref
                        class="overscroll-auto flex-auto lg:flex-initial".to_string()
                        message
                    ></MessageTextarea>
                    <SendButton class="flex-initial".to_string() message />
                    <ClearButton class="flex-initial".to_string() />
                </div>
            }
            .into_any()
        }
    };
    view! { {move || html(context.clone())} }
}

#[component]
fn MessageTextarea(
    class: String,
    message: RwSignal<String>,
    node_ref: NodeRef<Textarea>,
) -> impl IntoView {
    view! {
        <textarea
            node_ref=node_ref
            placeholder="在这里可以发言噢~"
            class=format!("textarea textarea-xl textarea-accent lg:w-xl {}", class)
            rows="5"
            prop:value=move || message.get()
            on:input:target=move |ev| message.set(ev.target().value())
        ></textarea>
    }
}

#[component]
fn SendButton(class: String, message: RwSignal<String>) -> impl IntoView {
    let context = expect_context::<ChessContext>();
    let send = move |context: ChessContext| {
        context.send_chat(ChessChatData::Text(message.get()));
        message.set("".to_string());
    };
    view! {
        <button
            class=format!("btn btn-soft btn-accent {}", class)
            on:click=move |_| { send(context.clone()) }
        >
            "发送（ALT+S）"
        </button>
    }
}

#[component]
fn ClearButton(class: String) -> impl IntoView {
    let context = expect_context::<ChessContext>();
    let clear = |context: ChessContext| {
        context.chat.clear_message();
    };
    view! {
        <button
            class=format!("btn btn-soft btn-accent {}", class)
            on:click=move |_| { clear(context.clone()) }
        >
            清屏
        </button>
    }
}
