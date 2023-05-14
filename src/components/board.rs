use std::collections::VecDeque;

use yew::{html, html_nested, Component, Context, Html, Properties};

use crate::components::tile::{Tile, TileColor};

const COLOR_SEQUENCE: [TileColor; 5] = [
    TileColor::Blue,
    TileColor::Green,
    TileColor::Purple,
    TileColor::Red,
    TileColor::Yellow,
];

pub enum PlayerBoardMsg {}

#[derive(PartialEq, Properties)]
pub struct PlayerBoardProps {}

pub struct PlayerBoard;

impl Component for PlayerBoard {
    type Message = PlayerBoardMsg;
    type Properties = PlayerBoardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <table>
                <tbody>
                    {
                        for COLOR_SEQUENCE.iter().map(
                            |c| html_nested!(<PlayerBoardRow start_color={(*c).clone()}/>)
                        )
                    }
                </tbody>
            </table>
        }
    }
}

#[derive(PartialEq, Properties)]
struct PlayerBoardRowProps {
    start_color: TileColor,
}

struct PlayerBoardRow;

impl Component for PlayerBoardRow {
    type Message = ();
    type Properties = PlayerBoardRowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut colors = VecDeque::from(COLOR_SEQUENCE);
        let pos = colors
            .iter()
            .position(|c| c == &ctx.props().start_color)
            .expect("Expected to find any color in the sequence.");
        colors.rotate_left(pos);

        html! {
        <tr>
            {
                for colors.iter().map(|c| html_nested!(
                    <td><Tile color={(*c).clone()} light=true/></td>
                ))
            }
        </tr>
        }
    }
}
