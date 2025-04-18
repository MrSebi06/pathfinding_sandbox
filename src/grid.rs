use bevy::app::{Startup, Update};
use bevy::color::Mix;
use bevy::prelude::{
    App, Assets, Changed, Click, Color, ColorMaterial, Commands, Component, Entity, Event,
    EventReader, EventWriter, Mesh, Mesh2d, MeshMaterial2d, Out, Over, Pointer, Query, Rectangle,
    Res, ResMut, Resource, Transform, Trigger, Window, With,
};
use bevy::window::PrimaryWindow;
use std::fmt::Display;

/// This module implements a simple grid-based game where each cell can be in one of four states:
/// `Empty`, `Wall`, `Start`, or `End`.
///
/// The grid is displayed in a 2D window, and the user can interact with
/// the cells by clicking on them. The cells change their state and color based on user interaction.
///
pub(super) fn plugin(app: &mut App) {
    app.init_resource::<CellEditMode>()
        .add_event::<CellClicked>()
        .add_systems(Startup, setup)
        .add_systems(Update, update_cell)
        .add_systems(Update, cell_hovered);
}

#[derive(Component)]
struct Cell {
    x: usize,
    y: usize,
}

#[derive(Component, Clone)]
pub enum CellState {
    Empty,
    Wall,
    Start,
    End,
}
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
impl CellState {
    fn to_color(&self) -> Color {
        match self {
            CellState::Empty => Color::WHITE,
            CellState::Wall => Color::srgb(0.5, 0.5, 0.5),
            CellState::Start => Color::srgb(0.0, 1.0, 0.0),
            CellState::End => Color::srgb(1.0, 0.0, 0.0),
        }
    }
}

#[derive(Component, Default)]
struct CellHovered(bool);

#[derive(Resource, Default)]
pub struct CellEditMode {
    pub mode: Option<CellState>,
}

#[derive(Event)]
struct CellClicked {
    pub entity: Entity,
    pub new_state: CellState,
}

fn setup(
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

fn on_mouse_over(
    over: Trigger<Pointer<Over>>,
    mut query: Query<&mut CellHovered>,
    cell_edit_mode: Res<CellEditMode>,
) {
    if !cell_edit_mode.mode.is_some() {
        return;
    }
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

fn on_click(
    click: Trigger<Pointer<Click>>,
    mut cell_edit_mode: ResMut<CellEditMode>,
    mut query: Query<Entity, With<Cell>>,
    mut cell_clicked: EventWriter<CellClicked>,
) {
    if let Some(edit_mode) = cell_edit_mode.mode.clone() {
        if let Ok(entity) = query.get_mut(click.entity()) {
            cell_clicked.send(CellClicked {
                entity,
                new_state: edit_mode,
            });
            cell_edit_mode.mode = None;
        }
    }
}

fn update_cell(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &mut CellState, &MeshMaterial2d<ColorMaterial>)>,
    mut cell_clicked: EventReader<CellClicked>,
) {
    for ev in cell_clicked.read() {
        if let Ok((_, mut state, material_handle)) = query.get_mut(ev.entity) {
            if let Some(material) = materials.get_mut(material_handle.0.id()) {
                *state = ev.new_state.clone();
                let new_color = state.to_color();
                material.color = new_color;
            }
        }
    }
}

fn cell_hovered(
    mut query: Query<
        (&CellHovered, &MeshMaterial2d<ColorMaterial>, &CellState),
        Changed<CellHovered>,
    >,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (hovered, material_handle, state) in query.iter_mut() {
        if let Some(material) = materials.get_mut(material_handle.0.id()) {
            if hovered.0 {
                material.color = state.to_color().mix(&Color::BLACK, 0.5)
            } else {
                material.color = state.to_color()
            }
        }
    }
}
