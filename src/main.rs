use components::navbar;
use yew::prelude::*;

use crate::{ game::Game};

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
