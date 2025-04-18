use crate::grid::components::{CellState, End, Start, Wall};
use crate::grid::events::CellClicked;
use bevy::prelude::{
    Assets, ColorMaterial, Commands, Entity, EventReader, MeshMaterial2d, Query, ResMut, With,
};

pub fn process_empty_cell(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cell_clicked: EventReader<CellClicked>,
    query: Query<(Entity, &MeshMaterial2d<ColorMaterial>)>,
) {
    for ev in cell_clicked.read() {
        if ev.new_state != CellState::Empty {
            continue;
        }

        commands.entity(ev.entity).remove::<Wall>();
        commands.entity(ev.entity).remove::<Start>();
        commands.entity(ev.entity).remove::<End>();

        set_cell_color(ev.entity, &query, &mut materials, CellState::Empty);
    }
}

pub fn process_wall_cell(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cell_clicked: EventReader<CellClicked>,
    query: Query<(Entity, &MeshMaterial2d<ColorMaterial>)>,
) {
    for ev in cell_clicked.read() {
        if ev.new_state != CellState::Wall {
            continue;
        }

        commands.entity(ev.entity).remove::<Start>();
        commands.entity(ev.entity).remove::<End>();
        commands.entity(ev.entity).insert(Wall);

        set_cell_color(ev.entity, &query, &mut materials, CellState::Wall);
    }
}

pub fn process_start_cell(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cell_clicked: EventReader<CellClicked>,
    query: Query<(Entity, &MeshMaterial2d<ColorMaterial>)>,
    start_query: Query<Entity, With<Start>>,
) {
    for ev in cell_clicked.read() {
        if ev.new_state != CellState::Start {
            continue;
        }

        for old_start in start_query.iter() {
            commands.entity(old_start).remove::<Start>();

            if old_start != ev.entity {
                set_cell_color(old_start, &query, &mut materials, CellState::Empty);
            }
        }

        commands.entity(ev.entity).remove::<Wall>();
        commands.entity(ev.entity).remove::<End>();
        commands.entity(ev.entity).insert(Start);

        set_cell_color(ev.entity, &query, &mut materials, CellState::Start);
    }
}

pub fn process_end_cell(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cell_clicked: EventReader<CellClicked>,
    query: Query<(Entity, &MeshMaterial2d<ColorMaterial>)>,
    end_query: Query<Entity, With<End>>,
) {
    for ev in cell_clicked.read() {
        if ev.new_state != CellState::End {
            continue;
        }

        for old_end in end_query.iter() {
            commands.entity(old_end).remove::<End>();

            if old_end != ev.entity {
                set_cell_color(old_end, &query, &mut materials, CellState::Empty);
            }
        }

        commands.entity(ev.entity).remove::<Wall>();
        commands.entity(ev.entity).remove::<Start>();
        commands.entity(ev.entity).insert(End);

        set_cell_color(ev.entity, &query, &mut materials, CellState::End);
    }
}

fn set_cell_color(
    entity: Entity,
    query: &Query<(Entity, &MeshMaterial2d<ColorMaterial>)>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    cell_state: CellState,
) {
    if let Ok((_, material_handle)) = query.get(entity) {
        if let Some(material) = materials.get_mut(material_handle.0.id()) {
            material.color = cell_state.to_color();
        }
    }
}
