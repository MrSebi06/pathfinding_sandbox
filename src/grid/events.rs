use crate::grid::components::CellState;
use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct CellClicked {
    pub entity: Entity,
    pub new_state: CellState,
}
