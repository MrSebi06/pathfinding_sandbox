use crate::grid::components::*;
use crate::grid::systems::input::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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

    let shape = meshes.add(Rectangle::new(cell_size - 2.0, cell_size - 2.0));

    for x in 0..num_cells {
        for y in 0..num_cells {
            let default_material = materials.add(ColorMaterial::from(Color::WHITE));
            commands
                .spawn((
                    Cell { x, y },
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
