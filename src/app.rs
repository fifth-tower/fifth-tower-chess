use crate::page::{Chess, Header, LeftSideBar, NotFound};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use tower::{
    common::COMMON_TITLE,
    web::prelude::OpTip,
    web_model::{OpTipData, Tipable},
};

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    let state = AppState::new();
    provide_context(state);

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />
        // sets the document title
        <Title text=COMMON_TITLE />
        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <div class="flex flex-col gap-2 px-4 w-full">
                <Header />
                <OpTip content=state.op_tip />
                <Routes fallback=|| view! { <NotFound /> }>
                    <Route path=path!("/") view=Chess />
                    <Route path=path!("/*any") view=NotFound />
                </Routes>
            </div>
        </Router>
    }
}

#[derive(Clone, Copy)]
pub struct AppState {
    pub op_tip: RwSignal<Option<OpTipData>>,
}

impl Tipable for AppState {
    fn tip<T>(&self, typ: tower::web_model::OpTipType, text: T)
    where
        T: Into<String>,
    {
        self.op_tip.set(Some(OpTipData {
            typ,
            class: "".into(),
            content: text.into(),
        }));
    }
}

impl AppState {
    pub fn new() -> Self {
        let op_tip = RwSignal::new(None);
        Self { op_tip }
    }
}
