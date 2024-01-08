use yew::{function_component, html, Html};

use crate::pages::background::Background;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Background>
            <div class="h-[calc(100vh-4rem)] flex flex-col content-center align-center items-center justify-center">
                // <DualView />
            </div>
        </Background>
    }
}
