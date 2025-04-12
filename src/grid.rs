use bevy::asset::{Assets, Handle};
use bevy::color::Color;
use bevy::prelude::{
    Camera2d, Click, ColorMaterial, Commands, Component, Mesh, Mesh2d, MeshMaterial2d, Out, Over,
    Pointer, Query, Rectangle, ResMut, Transform, Trigger, Window, With,
};
use bevy::window::PrimaryWindow;
use std::fmt::Display;

#[derive(Component)]
struct Cell {
    x: usize,
    y: usize,
}

#[derive(Component)]
enum CellState {
    Empty,
    Wall,
    Start,
    End,
    Path,
    Visited,
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::Empty => write!(f, "Empty"),
            CellState::Wall => write!(f, "Wall"),
            CellState::Start => write!(f, "Start"),
            CellState::End => write!(f, "End"),
            CellState::Path => write!(f, "Path"),
            CellState::Visited => write!(f, "Visited"),
        }
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let num_cells = 20;
    let cell_size = 40.0;

    let window_size = cell_size * num_cells as f32;
    let mut window = q_windows.single_mut();
    window
        .resolution
        .set(window_size + cell_size * 2.0, window_size + cell_size * 2.0);

    let grid_start_x = -window_size / 2.0;
    let grid_start_y = -window_size / 2.0;

    commands.spawn(Camera2d);

    let shape = meshes.add(Rectangle::new(cell_size - 2.0, cell_size - 2.0));
    let default_material = materials.add(ColorMaterial::from(Color::WHITE));
    let hover_material = materials.add(ColorMaterial::from(Color::BLACK));

    for x in 0..num_cells {
        for y in 0..num_cells {
            commands
                .spawn((
                    Cell { x, y },
                    CellState::Empty,
                    Mesh2d(shape.clone()),
                    MeshMaterial2d(default_material.clone()),
                    Transform::from_xyz(
                        grid_start_x + x as f32 * cell_size + cell_size / 2.0,
                        grid_start_y + y as f32 * cell_size + cell_size / 2.0,
                        0.0,
                    ),
                ))
                .observe(on_mouse_over(hover_material.clone()))
                .observe(on_mouse_out(default_material.clone()))
                .observe(on_click);
        }
    }
}

fn on_mouse_over(
    hover_material: Handle<ColorMaterial>,
) -> impl Fn(Trigger<Pointer<Over>>, Query<(&Cell, &mut MeshMaterial2d<ColorMaterial>, &CellState)>)
{
    move |over, mut query| {
        if let Ok((cell, mut material, cell_state)) = query.get_mut(over.entity()) {
            {
                println!(
                    "Cell hovered: ({}, {}) - state : {}",
                    cell.x, cell.y, cell_state
                );
                material.0 = hover_material.clone();
            }
        }
    }
}

fn on_mouse_out(
    default_material: Handle<ColorMaterial>,
) -> impl Fn(Trigger<Pointer<Out>>, Query<&mut MeshMaterial2d<ColorMaterial>>) {
    move |over, mut query| {
        if let Ok(mut material) = query.get_mut(over.entity()) {
            material.0 = default_material.clone();
        }
    }
}

fn on_click(click: Trigger<Pointer<Click>>, mut query: Query<(&Cell, &mut CellState)>) {
    if let Ok((cell, mut cell_state)) = query.get_mut(click.entity()) {
        *cell_state = match *cell_state {
            CellState::Empty => CellState::Wall,
            CellState::Wall => CellState::Start,
            CellState::Start => CellState::End,
            CellState::End => CellState::Empty,
            _ => CellState::Empty,
        };
    }
}
