use yew::prelude::*;

use crate::{
    components::{drawer::Drawer, welcome_hero::WelcomeHero},
    pages::background::Background,
};

#[function_component(Welcome)]
pub fn welcome() -> Html {
    html! {
        <Drawer>
            <Background>
                <WelcomeHero />
            </Background>
        </Drawer>
    }
}
