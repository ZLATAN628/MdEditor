use yew::prelude::*;

use crate::pages::{home::Home, welcome::Welcome};
mod components;
mod contexts;
mod pages;

#[function_component(App)]
fn app() -> Html {
    html! {
        // html!(<Welcome />)
        html! (<Home />)
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
