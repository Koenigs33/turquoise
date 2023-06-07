use gloo_console::log;
use rand::{seq::SliceRandom, thread_rng};
use yew::{html, AttrValue, Component, Context, Html, Properties};

use crate::components::{
    board::{CurrentRoundBoard, PlayerBoard},
    pot::{
        potarea::{PotArea, PotAreaUpdate},
        PotState, PotType,
    },
    tile::tile::{TileColor, TilePosition, TileState},
};

const NUMBER_OF_TILES_PER_COLOR: usize = 20;

pub struct Game {
    pub n_pots: u8,
    pub tiles: Vec<TileState>,
    pub current_player: AttrValue,
}

#[derive(Properties, PartialEq)]
pub struct GameProps {}

pub enum GameMsg {
    PotAreaUpdate(PotAreaUpdate),
}

impl Component for Game {
    type Message = GameMsg;
    type Properties = GameProps;

    fn create(_ctx: &Context<Self>) -> Self {
        log!("Creating the game");

        // for now we start with 2 players
        let players = vec!["player1", "player2"];
        let n_pots: u8 = match players.len() {
            2 => 5,
            _ => panic!("not supported"),
        };
        let current_player = players[0].into();

        // create a vector of tile states and shuffle it
        let mut tiles = vec![TileColor::Blue; NUMBER_OF_TILES_PER_COLOR];
        tiles.extend(vec![TileColor::Red; NUMBER_OF_TILES_PER_COLOR]);
        tiles.extend(vec![TileColor::Yellow; NUMBER_OF_TILES_PER_COLOR]);
        tiles.extend(vec![TileColor::Green; NUMBER_OF_TILES_PER_COLOR]);
        tiles.extend(vec![TileColor::Purple; NUMBER_OF_TILES_PER_COLOR]);
        let mut tiles: Vec<TileState> = tiles
            .iter()
            .enumerate()
            .map(|(id, c)| TileState::new(id as u8, (*c).clone()))
            .collect();
        tiles.shuffle(&mut thread_rng());

        let mut tiles_iter = tiles.iter_mut();

        // make the pot from the shuffled vector
        for i in 0..n_pots {
            for _ in 0..4 {
                let mut tile = tiles_iter.next().expect("No more tiles");
                tile.position = TilePosition::Pot(i);
            }
        }

        Self {
            n_pots,
            tiles,
            current_player,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GameMsg::PotAreaUpdate(update) => {
                let PotAreaUpdate::TileClicked {
                    pot_id,
                    tile_id: _,
                    tile_color,
                } = update;
                self.tiles
                    .iter_mut()
                    .filter(|t| {
                        if let TilePosition::Pot(i) = t.position {
                            i as usize == pot_id && t.color == tile_color
                        } else {
                            false
                        }
                    })
                    .for_each(|t| t.selected = !t.selected);
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let update = ctx.link().callback(|update| GameMsg::PotAreaUpdate(update));

        let mut pots = Vec::new();
        for pot_id in 0..self.n_pots {
            let mut tiles = Vec::new();
            for tile in &self.tiles {
                if let TilePosition::Pot(id) = tile.position {
                    if id == pot_id {
                        tiles.push((*tile).clone())
                    }
                }
            }
            pots.push(PotState {
                pot_type: PotType::Pot(pot_id),
                tiles,
            })
        }

        let mut tiles = Vec::new();
        for tile in &self.tiles {
            if tile.position == TilePosition::CommonPot {
                tiles.push((*tile).clone());
            }
        }
        pots.push(PotState {
            pot_type: PotType::Common,
            tiles,
        });

        html! {
            <>
                <div>
                    <PotArea {update} {pots} />
                </div>
                <div class="container">
                    <div class="row">
                        <CurrentRoundBoard/>
                        <PlayerBoard/>
                    </div>
                </div>
            </>
        }
    }
}
