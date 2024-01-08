use web_sys::window;
use yew::{classes, function_component, html, Callback, Html};

#[function_component(WelcomeHero)]
pub fn welcome_hero() -> Html {
    let open_home = Callback::from(move |_| {
        window().unwrap().alert_with_message("go home").unwrap();
    });
    let classes = classes!(
        "w-screen",
        "flex",
        "flex-col",
        "justify-center",
        "justify-center",
        "w-screen",
        "min-h-screen"
    );
    html! {
        <div class={classes}>
            <div class="flex flex-col items-center space-y-14">
                <h1
                    class="text-6xl md:text-7xl lg:text-8xl xl:text-8xl 2xl:text-8xl 3xl:text-8xl font-bold font-display tracking-wide">
                    {"Zlatan`s Editor"}
                </h1>
                <p class="text-2xl md:text-2xl lg:text-2xl xl:text-4xl 2xl:text-4xl 3xl:text-4xl font-mono">{"A markdown
                    editor."}
                </p>
                <button onclick={open_home} class="btn btn-lg btn-wide btn-primary btn-outline">{"Get Started"}</button>
            </div>
        </div>
    }
}
