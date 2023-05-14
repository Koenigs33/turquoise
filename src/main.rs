use components::navbar;
use yew::prelude::*;

use crate::components::{
    board::PlayerBoard,
    tile::{Tile, TileColor},
};

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <navbar::NavBar/>
            <div>
                <PlayerBoard/>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
