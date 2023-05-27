use yew::{function_component, html, Component, Context, Html, Properties};

use crate::components::tile::{Tile, TileColor};

#[derive(PartialEq, Properties)]
pub struct PotAreaProps {}

#[function_component]
pub fn PotArea(props: &PotAreaProps) -> Html {
    let PotAreaProps {} = props;
    html! {
        <div class="p-big bg-info">
            <div class="row">
                <Pot/>
                <Pot/>
                <Pot/>
                <Pot/>
                <Pot/>
            </div>
            <div class="row">
                <CommonPot/>
            </div>
        </div>
    }
}

pub struct CommonPot;

#[derive(Properties, PartialEq)]
pub struct CommonPotProps {}

pub enum CommonPotMsg {}

impl Component for CommonPot {
    type Message = CommonPotMsg;
    type Properties = CommonPotProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="col-sm container-small">
                <div class="container p-big">
                    <div class="row bg-gray-light border">
                        {
                            for (0..15).map(|_| html!{
                                <Tile color={TileColor::Blue} hatched=false filled=true/>
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
pub struct PotProps {}

pub enum PotMsg {}

impl Component for Pot {
    type Message = PotMsg;
    type Properties = PotProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="col-sm container-small">
                <div class="container p-big">
                    <div class="row bg-gray-light border">
                        <Tile color={TileColor::Blue} hatched=false filled=true/>
                        <Tile color={TileColor::Blue} hatched=false filled=true/>
                        <Tile color={TileColor::Blue} hatched=false filled=true/>
                        <Tile color={TileColor::Blue} hatched=false filled=true/>
                        <Tile color={TileColor::Blue} hatched=false filled=true/>
                    </div>
                </div>
            </div>
        }
    }
}
