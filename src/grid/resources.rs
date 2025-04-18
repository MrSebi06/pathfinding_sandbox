use crate::grid::components::CellState;
use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct CellEditMode {
    pub mode: Option<CellState>,
}
