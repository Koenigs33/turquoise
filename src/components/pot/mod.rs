use crate::components::tile::tile::TileState;

pub mod commonpot;
pub mod pot;
pub mod potarea;

#[derive(PartialEq, Debug)]
pub struct PotState {
    pub pot_type: PotType,
    pub tiles: Vec<TileState>,
}

#[derive(PartialEq, Debug)]
pub enum PotType {
    Pot(u8),
    Common,
}
