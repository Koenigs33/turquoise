use std::collections::VecDeque;

use yew::{function_component, html, html_nested, Component, Context, Html, Properties};

use crate::components::tile::{Tile, TileColor, TilePosition};

const COLOR_SEQUENCE: [TileColor; 5] = [
    TileColor::Blue,
    TileColor::Green,
    TileColor::Purple,
    TileColor::Red,
    TileColor::Yellow,
];

/// Represents the tiles already validated
pub struct PlayerBoard;

impl Component for PlayerBoard {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <table class="m-big">
                <tbody>
                    {
                        for COLOR_SEQUENCE.iter().map(
                            |c| html!(<PlayerBoardRow start_color={(*c).clone()}/>)
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
                colors.iter().map(|c| html!(
                    <td><Tile color={(*c).clone()} hatched=true filled=true position={TilePosition::PlayerBoard}/></td>
                )).collect::<Html>()
            }
        </tr>
        }
    }
}

/// Current selected tiles to be constructed
pub struct CurrentRoundBoard;

impl Component for CurrentRoundBoard {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <table class="m-big">
                <tbody>
                    {
                        (0..5).map(|i| html_nested!(<tr><CurrentRoundBoardRow size={i+1}/></tr>))
                            .collect::<Html>()
                    }
                </tbody>
            </table>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct CurrentRoundBoardRowProps {
    size: u8,
}

#[function_component]
pub fn CurrentRoundBoardRow(props: &CurrentRoundBoardRowProps) -> Html {
    let CurrentRoundBoardRowProps { size } = props;
    let empty_tiles = 0..(5 - size);
    let tiles = 0..*size;
    html! {
        <>
            { for empty_tiles.map(|_| html!{<td></td>}) }
            {
                for tiles.map(|_| html!{
                    <td>
                        <Tile color={TileColor::Blue} hatched=true filled=false position={TilePosition::CurrentBoard}/>
                    </td>
                })
            }
        </>
    }
}
