use leptos::prelude::*;

#[component]
pub fn Blank(
    #[prop(into)] app_id: String,
    #[prop(into)] src: String,
    #[prop(into, optional)] width: MaybeProp<u32>,
    #[prop(into, optional)] height: MaybeProp<u32>,
    #[prop(default=Dir::Record)] dir: Dir,
) -> impl IntoView {
    view! {
        <div class="hidden text-base text-left alert alert-info alert-success alert-warning alert-error badge badge-xs badge-neutral badge-info badge-warning" />
    }
}
