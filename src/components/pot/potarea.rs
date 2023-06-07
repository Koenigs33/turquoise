use yew::{function_component, html, Callback, Html, Properties};

use crate::components::{
    pot::{pot::Pot, PotState, PotType},
    tile::tile::TileColor,
};

#[derive(PartialEq, Properties)]
pub struct PotAreaProps {
    pub update: Callback<PotAreaUpdate>,
    pub pots: Vec<PotState>,
}

#[function_component]
pub fn PotArea(props: &PotAreaProps) -> Html {
    let PotAreaProps { update, pots } = props;
    html! {
        <div class="p-big bg-info">
            <div class="row">
                {
                    for pots.iter()
                        .filter(|pot| matches!(pot.pot_type, PotType::Pot(_))).enumerate()
                        .map(|(id, pot)| html!(<Pot {id} tiles={pot.tiles.clone()} update={update}/>))
                }
            </div>
            // <div class="row">
            //     <CommonPot tiles={pots_state.common_pot.clone()}/>
            // </div>
        </div>
    }
}

#[derive(Debug)]
pub enum PotAreaUpdate {
    TileClicked {
        pot_id: usize,
        tile_id: usize,
        tile_color: TileColor,
    },
}
