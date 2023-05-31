use std::collections::HashMap;

use gloo_console::log;
use rand::{seq::SliceRandom, thread_rng};
use yew::{html, Component, Context, Html, Properties};

use crate::components::{
    board::{CurrentRoundBoard, PlayerBoard},
    pot::PotArea,
    tile::TileColor,
};

const NUMBER_OF_TILES_PER_COLOR: usize = 20;

#[derive(PartialEq, Clone)]
pub struct PotState {
    pub pots: HashMap<String, Vec<TileColor>>,
    pub common_pot: Vec<TileColor>,
}

impl PotState {
    pub fn new(pots: HashMap<String, Vec<TileColor>>) -> Self {
        Self {
            pots,
            common_pot: Vec::new(),
        }
    }
}

#[derive(Default)]
pub struct PlayerState {
    current_round: Vec<Vec<TileColor>>,
    player_board: Vec<Vec<Option<TileColor>>>,
}

pub struct Game {
    pub pot_state: PotState,
    pub players_state: HashMap<String, PlayerState>,
    pub current_player: String,
    pub tiles_bin: Vec<TileColor>,
    pub tiles_reserve: Vec<TileColor>,
}

#[derive(Properties, PartialEq)]
pub struct GameProps {}

pub enum GameMsg {}

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
        let mut players_state = HashMap::new();
        for player in players {
            players_state.insert(player.into(), PlayerState::default());
        }

        // create a vector of tiles and shuffle it
        let mut tiles = vec![TileColor::Blue; NUMBER_OF_TILES_PER_COLOR];
        tiles.extend(vec![TileColor::Red; NUMBER_OF_TILES_PER_COLOR]);
        tiles.extend(vec![TileColor::Yellow; NUMBER_OF_TILES_PER_COLOR]);
        tiles.extend(vec![TileColor::Green; NUMBER_OF_TILES_PER_COLOR]);
        tiles.extend(vec![TileColor::Purple; NUMBER_OF_TILES_PER_COLOR]);
        tiles.shuffle(&mut thread_rng());

        // make the pot from the shuffled vector
        let mut pots = HashMap::new();
        for i in 1..=n_pots {
            let pot_name = format!("pot_{}", i);
            pots.insert(
                pot_name,
                (0..4)
                    .map(|_| tiles.pop().expect("No more tiles"))
                    .collect(),
            );
        }
        let pot_state = PotState::new(pots);

        Self {
            pot_state,
            players_state,
            current_player,
            tiles_bin: Vec::new(),
            tiles_reserve: tiles,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div>
                    <PotArea pots_state={self.pot_state.clone()}/>
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
