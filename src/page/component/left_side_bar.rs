use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn LeftSideBar(#[prop(into)] class: String) -> impl IntoView {
    let active = RwSignal::new("/");
    view! {
        <div class=format!(
            "{} w-28 lg:w-56 shadow-md menu bg-base-200 rounded-box h-screen relative",
            class,
        )>
            <ul class="menu">
                <li class="menu-title">
                    <a href="/" rel="external">
                        第五塔灵
                    </a>
                </li>
                <li>
                    <MenuItem active label="日常" href="/" icon=icondata::AiCheckOutlined />
                </li>
                <li>
                    <MenuItem active label="秘籍" href="/script" icon=icondata::AiCheckOutlined />
                </li>
                <li>
                    <MenuItem active label="录制" href="/record" icon=icondata::AiCheckOutlined />
                </li>
                <li>
                    <MenuItem active label="广场" href="/square" icon=icondata::AiCheckOutlined />
                </li>
            </ul>
            <div class="flex absolute bottom-1 justify-start w-full">
                <div class="dropdown dropdown-hover dropdown-top">
                    <div tabindex="0" role="button" class="m-1 btn btn-ghost btn-circle">
                        <Icon icon=icondata::AiSettingOutlined />
                    </div>
                    <ul
                        tabindex="0"
                        class="p-2 w-32 shadow-sm dropdown-content menu bg-base-100 rounded-box z-1"
                    >
                        <li>
                            <a href="/about">关于我们</a>
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    }
}

#[component]
fn MenuItem(
    active: RwSignal<&'static str>,
    href: &'static str,
    icon: icondata_core::Icon,
    label: &'static str,
) -> impl IntoView {
    view! {
        <a
            href=href
            class=move || to_class_name(active, href)
            on:click=move |_| {
                active.set(href);
            }
        >
            <Icon icon />
            {label}
        </a>
    }
}

fn to_class_name(active: RwSignal<&'static str>, current: &str) -> &'static str {
    if active.get().eq(current) {
        "menu-active"
    } else {
        ""
    }
}
