use crate::common::to_turn_label;
use crate::page::*;
use chess_core::*;
use leptos::prelude::*;
use leptos_use::use_interval_fn;
use leptos_use::utils::Pausable;

#[component]
pub(crate) fn TurnCountDown(class: String) -> impl IntoView {
    let context = expect_context::<ChessContext>();
    view! {
        <div class=format!(
            "text-center {}",
            class,
        )>
            {move || {
                let state = context.state.get();
                if !matches!(state, ChessState::InGame) {
                    return view! {}.into_any();
                }
                let (cur_turn, turn_second, _) = context.turn.get();
                let color = match cur_turn {
                    PieceColor::UnSet => "",
                    PieceColor::Red => "color:red;",
                    PieceColor::Black => "color:black",
                }
                    .to_string();
                view! { {(turn_second > 0).then(|| view! { <CountDown turn_second color /> })} }
                    .into_any()
            }}
        </div>
    }
    .into_any()
}

#[component]
fn CountDown(turn_second: u16, color: String) -> impl IntoView {
    let context = expect_context::<ChessContext>();
    if turn_second == 0 {
        return view! {}.into_any();
    }
    let remain_sec = RwSignal::new(turn_second);
    let Pausable {
        pause,
        resume,
        is_active,
    } = use_interval_fn(
        move || {
            let remain = remain_sec.get_untracked();
            if remain > 0 {
                remain_sec.set(remain - 1);
            }
        },
        1000,
    );
    pause();
    let turn = context.turn.clone();
    Effect::new(move |_| {
        if !is_active.get() {
            return;
        }
        let remain = remain_sec.get();
        if remain <= 0 {
            context.send_chat(ChessChatData::Lost(ChessLostType::Timeout));
            pause();
        }
    });

    Effect::new(move |_| {
        if is_active.get() {
            return;
        }
        let (cur_turn, turn_second, _) = turn.get();
        if !matches!(cur_turn, PieceColor::UnSet) {
            remain_sec.set(turn_second);
            resume();
        }
    });
    view! {
        <span class="font-mono text-6xl text-center countdown">
            {move || {
                let second = remain_sec.get();
                view! { <span style=format!("--value:{};{}", second, color)>{second}</span> }
            }}

        </span>
    }
    .into_any()
}

#[component]
pub(crate) fn TurnCountDownMenu() -> impl IntoView {
    let context = expect_context::<ChessContext>();
    let second_text = move || {
        let second = context.turn.get().1;
        to_turn_label(second)
    };
    view! {
        {move || {
            if matches!(context.turn.get().2, ChessSetTurnStatus::NoSet) {
                return view! { <span class="self-center mr-4">"步时：" {second_text}</span> }
                    .into_any();
            }
            view! {
                <div>
                    "设置步时：" <div class="dropdown dropdown-start">
                        <div tabindex="0" role="button" class="m-1 btn btn-sm btn-ghost">
                            {second_text}
                        </div>
                        <ul
                            tabindex="0"
                            class="p-2 w-52 shadow-sm dropdown-content menu bg-base-100 rounded-box z-1"
                        >
                            <TurnCountDownMenuItem second=30 label="30s".to_string() />
                            <TurnCountDownMenuItem second=60 label="60s".to_string() />
                            <TurnCountDownMenuItem second=90 label="90s".to_string() />
                            <TurnCountDownMenuItem second=0 label="不限".to_string() />
                        </ul>
                    </div>
                </div>
            }
                .into_any()
        }}
    }
}

#[component]
fn TurnCountDownMenuItem(second: u16, label: String) -> impl IntoView {
    let context = expect_context::<ChessContext>();
    view! {
        <li on:click=move |_| {
            context
                .turn
                .update(|t| {
                    t.1 = second;
                    t.2 = ChessSetTurnStatus::Setted;
                });
            context.send_chat(ChessChatData::SetTurn(second));
        }>
            <a>{label}</a>
        </li>
    }
}
