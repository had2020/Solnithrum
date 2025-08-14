use gloo_storage::{LocalStorage, Storage};
use web_sys::*;
use yew::prelude::*;

pub fn generate_key_pair() -> ([u8; 24], Vec<u8>) {
    let window = web_sys::window().unwrap();
    let crypto = window.crypto().unwrap();
    let mut pass_phrase: [u8; 24]; // 24 words
    crypto.get_random_values_with_u8_array(&mut pass_phrase); // Pass-Phrase
}

pub fn save_key_pair() {
    LocalStorage::set("a1d2dr3", "value").unwrap();
    LocalStorage::set("p4as3e", "value").unwrap();
}

pub fn load_key_pair() {}

#[function_component(App)]
pub fn app() -> Html {
    let counter1: UseStateHandle<i32> = use_state(|| 0);
    let counter2: UseStateHandle<i32> = use_state(|| 0);

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
