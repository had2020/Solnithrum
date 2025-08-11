use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.svg" alt="Yew logo" />
            <h1>{ "Test!" }</h1>
        </main>
    }
}
