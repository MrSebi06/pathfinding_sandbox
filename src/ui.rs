use bevy::prelude::{App, Resource, Update};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EguiPlugin)
        .init_resource::<GridUiState>()
        .add_systems(Update, grid_ui);
}

#[derive(Resource)]
struct GridUiState {
    is_open: bool,
}
impl Default for GridUiState {
    fn default() -> Self {
        Self { is_open: true }
    }
}

fn grid_ui(mut contexts: EguiContexts, mut ui_state: bevy::prelude::ResMut<GridUiState>) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("grid_control").show_animated(ctx, ui_state.is_open, |ui| {
        ui.label("Grid Controls");
        if ui.button("Click me!").clicked() {
            ui_state.is_open = !ui_state.is_open;
            println!("Button clicked!");
        }
    });
}
