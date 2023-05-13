use gloo_console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[function_component]
pub fn NavBar() -> Html {
    html! {
        <nav aria-label="Main" class="navbar navbar--fixed-top">
            <div class="navbar__inner">
                <div class="navbar__items">
                    <HomeButton/>
                </div>
            </div>
        </nav>
    }
}

#[derive(PartialEq, Properties)]
pub struct HomeButtonProps {}

#[function_component]
pub fn HomeButton(props: &HomeButtonProps) -> Html {
    let HomeButtonProps {} = props;
    let onclick = Callback::from(|e: MouseEvent| {
        let msg = JsValue::from("click");
        let tgt = e.target();
        log!(msg);
        log!(tgt);
    });
    html! {
        <button type="button" {onclick}>
            <svg width="30" height="30" viewBox="0 0 30 30">
                <path stroke="currentColor" stroke-linecap="round" stroke-miterlimit="10" stroke-width="2" d="M4 7h22M4 15h22M4 23h22"></path>
            </svg>
        </button>
    }
}
