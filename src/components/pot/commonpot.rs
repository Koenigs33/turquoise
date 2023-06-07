use yew::{html, Component, Context, Html, Properties};

use crate::components::tile::tile::TileColor;

use super::pot::PotMsg;

pub struct CommonPot;

#[derive(Properties, PartialEq)]
pub struct CommonPotProps {
    tiles: Vec<TileColor>,
}

impl Component for CommonPot {
    type Message = PotMsg;
    type Properties = CommonPotProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PotMsg::TileClicked(id, color) => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let CommonPotProps { tiles } = ctx.props();

        // let tile_clicked = ctx.link().callback(|_| PotMsg::TileClicked);
        html! {
            <div class="col-sm container-small">
                <div class="container p-big">
                    <div class="row bg-gray-light border">
                        // {
                        //     for tiles.iter().map(|c| html!{
                        //         <Tile
                        //             click_handler={tile_clicked}
                        //             color={(*c).clone()}
                        //         />
                        //     })
                        // }
                    </div>
                </div>
            </div>
        }
    }
}
