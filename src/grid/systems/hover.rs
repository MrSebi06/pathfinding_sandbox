use crate::grid::components::*;
use bevy::color::Mix;
use bevy::prelude::{Assets, Changed, Color, ColorMaterial, Entity, MeshMaterial2d, Query, ResMut};

pub fn cell_hovered(
    mut materials: ResMut<Assets<ColorMaterial>>,
    hovered_cells: Query<
        (
            Entity,
            &CellHovered,
            &MeshMaterial2d<ColorMaterial>,
            Option<&Wall>,
            Option<&Start>,
            Option<&End>,
        ),
        Changed<CellHovered>,
    >,
) {
    for (_, hovered, material_handle, wall, start, end) in hovered_cells.iter() {
        if let Some(material) = materials.get_mut(material_handle.0.id()) {
            // Determine the cell state from components
            let state = CellState::from_components(wall.is_some(), start.is_some(), end.is_some());

            if hovered.0 {
                material.color = state.to_color().mix(&Color::BLACK, 0.5);
            } else {
                material.color = state.to_color();
            }
        }
    }
}
