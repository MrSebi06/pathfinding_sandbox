use crate::grid::components::Cell;
use bevy::prelude::Resource;

struct Path {
    cells: Vec<Cell>,
}

#[derive(Resource)]
struct VisitedPaths {
    paths: Vec<Path>,
}
