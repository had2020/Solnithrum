use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.svg" alt="Yew logo" />
            <h1>{ "Test!" }</h1>
            <div>
                <button {onclick}>{ "+1" }</button>
                <p>{ *counter }</p>
            </div>
        </main>
    }
}
