use gloo_console::log;
use yew::{html, Callback, Component, Context, Html, Properties};

use crate::components::tile::tile::{Tile, TileColor, TileState};

use super::potarea::PotAreaUpdate;

pub struct Pot;

#[derive(Properties, PartialEq)]
pub struct PotProps {
    pub id: usize,
    pub tiles: Vec<TileState>,
    pub update: Callback<PotAreaUpdate>,
}

pub enum PotMsg {
    TileClicked(u8, TileColor),
}

impl Component for Pot {
    type Message = PotMsg;
    type Properties = PotProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let click_handler = &ctx.props().update;
        let pot_id = &ctx.props().id;
        match msg {
            PotMsg::TileClicked(id, color) => {
                click_handler.emit(PotAreaUpdate::TileClicked {
                    pot_id: *pot_id,
                    tile_id: id as usize,
                    tile_color: color,
                });
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let tiles = &ctx.props().tiles;

        html! {
            <div class="col-sm container-small">
                <div class="container p-big">
                    <div class="row bg-gray-light border">
                        {
                            for tiles.iter().map(|s| {
                                let tile_clicked = ctx.link().callback(|(id, color)| PotMsg::TileClicked(id, color));

                                html!{
                                    <Tile
                                    click_handler={tile_clicked}
                                    color={s.color.clone()}
                                    id={s.id}
                                    selected={s.selected}
                                    />
                                }
                            })
                        }
                    </div>
                </div>
            </div>
        }
    }
}
