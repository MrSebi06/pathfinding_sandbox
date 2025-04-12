use bevy::asset::Assets;
use bevy::color::Color;
use bevy::prelude::{
    Camera2d, Changed, Click, ColorMaterial, Commands, Component, Mesh, Mesh2d, MeshMaterial2d,
    Mix, Or, Out, Over, Pointer, Query, Rectangle, ResMut, Transform, Trigger, Window, With,
};
use bevy::window::PrimaryWindow;
use std::fmt::Display;

#[derive(Component)]
struct Cell {
    x: usize,
    y: usize,
}

#[derive(Component)]
pub enum CellState {
    Empty,
    Wall,
    Start,
    End,
}

#[derive(Component, Default)]
pub struct CellHovered(bool);

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CellState::Empty => write!(f, "Empty"),
            CellState::Wall => write!(f, "Wall"),
            CellState::Start => write!(f, "Start"),
            CellState::End => write!(f, "End"),
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

    for x in 0..num_cells {
        for y in 0..num_cells {
            let default_material = materials.add(ColorMaterial::from(Color::WHITE));
            commands
                .spawn((
                    Cell { x, y },
                    CellState::Empty,
                    CellHovered(false),
                    Mesh2d(shape.clone()),
                    MeshMaterial2d(default_material),
                    Transform::from_xyz(
                        grid_start_x + x as f32 * cell_size + cell_size / 2.0,
                        grid_start_y + y as f32 * cell_size + cell_size / 2.0,
                        0.0,
                    ),
                ))
                .observe(on_mouse_over)
                .observe(on_mouse_out)
                .observe(on_click);
        }
    }
}

fn on_mouse_over(over: Trigger<Pointer<Over>>, mut query: Query<&mut CellHovered>) {
    if let Ok(mut cell_hovered) = query.get_mut(over.entity()) {
        {
            cell_hovered.0 = true;
        }
    }
}

fn on_mouse_out(out: Trigger<Pointer<Out>>, mut query: Query<&mut CellHovered>) {
    if let Ok(mut cell_hovered) = query.get_mut(out.entity()) {
        cell_hovered.0 = false;
    }
}

fn on_click(click: Trigger<Pointer<Click>>, mut query: Query<&mut CellState>) {
    if let Ok(mut cell_state) = query.get_mut(click.entity()) {
        *cell_state = match *cell_state {
            CellState::Empty => CellState::Wall,
            CellState::Wall => CellState::Start,
            CellState::Start => CellState::End,
            CellState::End => CellState::Empty,
        };
    }
}

pub fn update_cell_colors(
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<
        (&CellState, &MeshMaterial2d<ColorMaterial>, &CellHovered),
        Or<(Changed<CellState>, Changed<CellHovered>)>,
    >,
) {
    for (state, material_handle, cell_hovered) in query.iter() {
        if let Some(material) = materials.get_mut(material_handle.0.id()) {
            let mut new_color = match state {
                CellState::Empty => Color::WHITE,
                CellState::Wall => Color::srgb(0.5, 0.5, 0.5),
                CellState::Start => Color::srgb(0.0, 1.0, 0.0),
                CellState::End => Color::srgb(1.0, 0.0, 0.0),
            };

            if cell_hovered.0 {
                new_color = new_color.mix(&Color::BLACK, 0.5);
            }

            material.color = new_color;
        }
    }
}
