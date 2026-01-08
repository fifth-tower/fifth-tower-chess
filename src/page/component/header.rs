use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <div class="navbar">
            <div class="navbar-start"></div>
            <div class="navbar-end">
                <a class="text-white bg-red-500 rounded-xl btn" href="#">
                    <Icon icon=icondata::AiMailOutlined />
                    登陆
                </a>

            </div>
        </div>
    }
}
