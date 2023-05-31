use yew::{function_component, html, Component, Context, Html, Properties};

use crate::{
    components::tile::{Tile, TileColor, TilePosition},
    game::PotState,
};

#[derive(PartialEq, Properties)]
pub struct PotAreaProps {
    pub pots_state: PotState,
}

#[function_component]
pub fn PotArea(props: &PotAreaProps) -> Html {
    let PotAreaProps { pots_state } = props;
    html! {
        <div class="p-big bg-info">
            <div class="row">
                {
                    for pots_state.pots.values().map(|v| {
                        html!(<Pot tiles={(*v).clone()}/>)
                    })
                }
            </div>
            <div class="row">
                <CommonPot tiles={pots_state.common_pot.clone()}/>
            </div>
        </div>
    }
}

pub struct CommonPot;

#[derive(Properties, PartialEq)]
pub struct CommonPotProps {
    tiles: Vec<TileColor>,
}

pub enum CommonPotMsg {}

impl Component for CommonPot {
    type Message = CommonPotMsg;
    type Properties = CommonPotProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let CommonPotProps { tiles } = ctx.props();
        html! {
            <div class="col-sm container-small">
                <div class="container p-big">
                    <div class="row bg-gray-light border">
                        {
                            for tiles.iter().map(|c| html!{
                                <Tile color={(*c).clone()} hatched=false filled=true position={TilePosition::CommonPot}/>
                            })
                        }
                    </div>
                </div>
            </div>
        }
    }
}

pub struct Pot;

#[derive(Properties, PartialEq)]
pub struct PotProps {
    tiles: Vec<TileColor>,
}

pub enum PotMsg {}

impl Component for Pot {
    type Message = PotMsg;
    type Properties = PotProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let PotProps { tiles } = ctx.props();
        html! {
            <div class="col-sm container-small">
                <div class="container p-big">
                    <div class="row bg-gray-light border">
                        {
                            for tiles.iter().map(|c| html!{
                                <Tile position={TilePosition::Pot} color={(*c).clone()}/>
                            })
                        }
                    </div>
                </div>
            </div>
        }
    }
}
