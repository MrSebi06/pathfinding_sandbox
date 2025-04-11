mod grid;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, grid::setup);
    app.run();
}
