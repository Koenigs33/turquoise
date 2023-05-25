use components::navbar;
use yew::prelude::*;

use crate::components::board::{CurrentRoundBoard, PlayerBoard};

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <navbar::NavBar/>
            <div class="container">
                <div class="row">
                    <CurrentRoundBoard/>
                    <PlayerBoard/>
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
