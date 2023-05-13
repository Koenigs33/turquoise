use components::navbar;
use yew::prelude::*;

use crate::components::tile::{Tile, TileColor};

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <navbar::NavBar/>
            <div>
                <Tile color={TileColor::Red} light=true/>
                <Tile color={TileColor::Yellow} light=false/>
                <Tile color={TileColor::Blue} light=true/>
                <Tile color={TileColor::Purple} light=false/>
                <Tile color={TileColor::Green} light=true/>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
