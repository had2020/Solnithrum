use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let counter1 = use_state(|| 0);
    let counter2 = use_state(|| 0);

    let onclick1 = {
        let counter1 = counter1.clone();
        Callback::from(move |_| counter1.set(*counter1 + 1))
    };

    let new_wallet = {
        let counter2 = counter2.clone();
        Callback::from(move |_| counter2.set(*counter2 + 1))
    };

    let load_wallet = {
        let counter2 = counter2.clone();
        Callback::from(move |_| counter2.set(*counter2 + 1))
    };

    html! {
        <main>
            <h1>{ "Test!" }</h1>

            <div>
                <button onclick={onclick1.clone()}>{ "+1" }</button>
                <p>{ *counter1 }</p>
            </div>

            <div>
                <button onclick={new_wallet.clone()}>{ "Create Wallet" }</button>
            </div>

            <div>
                <button onclick={load_wallet.clone()}>{ "Load Wallet" }</button>
            </div>

        </main>
    }
}
