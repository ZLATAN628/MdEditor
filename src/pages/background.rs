use yew::prelude::*;

use crate::contexts::config::use_config;

#[derive(Properties, PartialEq)]
pub struct BackgroundProps {
    pub children: Children,
}

#[function_component(Background)]
pub fn background(props: &BackgroundProps) -> Html {
    let theme = use_config();
    let classes = classes!(
        "flex",
        "flex-col",
        "justify-between",
        "max-w-[calc(100svw)]",
        "print:hidden",
        "min-h-screen"
    );
    html! {
        <div data-theme={theme} class={classes}>
            { props.children.clone() }
        </div>
    }
}
