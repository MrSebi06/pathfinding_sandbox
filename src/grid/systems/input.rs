use crate::grid::components::{Cell, CellHovered};
use crate::grid::events::CellClicked;
use crate::grid::resources::CellEditMode;
use bevy::prelude::{Click, Entity, EventWriter, Out, Over, Pointer, Query, Res, ResMut, Trigger};

pub fn on_mouse_over(
    over: Trigger<Pointer<Over>>,
    mut query: Query<&mut CellHovered>,
    cell_edit_mode: Res<CellEditMode>,
) {
    if cell_edit_mode.mode.is_none() {
        return;
    }

    if let Ok(mut cell_hovered) = query.get_mut(over.entity()) {
        cell_hovered.0 = true;
    }
}

pub fn on_mouse_out(out: Trigger<Pointer<Out>>, mut query: Query<&mut CellHovered>) {
    if let Ok(mut cell_hovered) = query.get_mut(out.entity()) {
        cell_hovered.0 = false;
    }
}

pub fn on_click(
    click: Trigger<Pointer<Click>>,
    mut cell_edit_mode: ResMut<CellEditMode>,
    query: Query<(Entity, &Cell)>,
    mut cell_clicked: EventWriter<CellClicked>,
) {
    if let Some(edit_mode) = cell_edit_mode.mode.clone() {
        if let Ok((entity, _)) = query.get(click.entity()) {
            cell_clicked.send(CellClicked {
                entity,
                new_state: edit_mode,
            });
            cell_edit_mode.mode = None;
        }
    }
}
