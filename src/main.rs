use components::navbar;
use yew::prelude::*;

use crate::{
    components::{
        board::{CurrentRoundBoard, PlayerBoard},
        pot::PotArea,
    },
    game::Game,
};

mod components;
mod game;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <navbar::NavBar/>
            <Game/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
