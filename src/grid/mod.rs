use crate::grid::events::CellClicked;
use crate::grid::resources::CellEditMode;
use crate::grid::systems::*;
use bevy::prelude::{App, Startup, Update};

pub mod components;
mod events;
pub mod resources;
mod systems;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CellEditMode>()
        .add_event::<CellClicked>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                process_empty_cell,
                process_wall_cell,
                process_start_cell,
                process_end_cell,
            ),
        )
        .add_systems(Update, cell_hovered);
}
