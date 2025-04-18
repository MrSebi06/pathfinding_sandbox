use bevy::prelude::{Color, Component};

#[derive(Component)]
pub struct Cell {
    pub x: usize,
    pub y: usize,
}

#[derive(Component)]
pub struct Start;

#[derive(Component)]
pub struct End;

#[derive(Component)]
pub struct Wall;

#[derive(Component, Default)]
pub struct CellHovered(pub bool);

#[derive(Clone, PartialEq)]
pub enum CellState {
    Empty,
    Wall,
    Start,
    End,
}

impl CellState {
    pub(crate) fn to_color(&self) -> Color {
        match self {
            CellState::Empty => Color::WHITE,
            CellState::Wall => Color::srgb(0.5, 0.5, 0.5),
            CellState::Start => Color::srgb(0.0, 1.0, 0.0),
            CellState::End => Color::srgb(1.0, 0.0, 0.0),
        }
    }

    pub(crate) fn from_components(has_wall: bool, has_start: bool, has_end: bool) -> Self {
        if has_start {
            CellState::Start
        } else if has_end {
            CellState::End
        } else if has_wall {
            CellState::Wall
        } else {
            CellState::Empty
        }
    }
}
