use components::navbar;
use yew::prelude::*;

use crate::components::{
    board::{CurrentRoundBoard, PlayerBoard},
    pot::PotArea,
};

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <navbar::NavBar/>
            <div>
                <PotArea/>
            </div>
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
