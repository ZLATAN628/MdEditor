use yew::{function_component, html, Children, Html, Properties};

use crate::contexts::config::use_config;

#[derive(Debug, PartialEq, Properties)]
pub struct DrawerProps {
    pub children: Children,
}

#[function_component(Drawer)]
pub fn drawer(props: &DrawerProps) -> Html {
    let theme = use_config();

    // let drawer_classes: yew::prelude::Classes = classes!("flex", "flex-col",);

    html! {
        <div data-theme={theme} class="drawer print:hidden">
            <div class="drawer-content">
                { props.children.clone() }
            </div>
        </div>

    }
}
