mod grid;
mod ui;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_plugins(grid::plugin)
        .add_plugins(ui::plugin)
        .run();
}
