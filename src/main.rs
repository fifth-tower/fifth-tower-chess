mod app;
mod common;
mod page;
// mod service;

use crate::app::App;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App /> }
    })
}
