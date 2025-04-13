use bevy::prelude::{App, Resource, Startup, Update};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EguiPlugin)
        .init_resource::<GridUiState>()
        .add_systems(Startup, setup_icons)
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

fn setup_icons(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    let mut fonts = egui::FontDefinitions::default();
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
    ctx.set_fonts(fonts);
}

fn grid_ui(mut contexts: EguiContexts, mut ui_state: bevy::prelude::ResMut<GridUiState>) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("grid_control").show_animated(ctx, ui_state.is_open, |ui| {
        ui.label("Grid Controls");
    });

    let frame = egui::Frame {
        fill: egui::Color32::from_black_alpha(0),
        stroke: egui::Stroke::NONE,
        ..Default::default()
    };
    egui::TopBottomPanel::top("top_panel")
        .frame(frame)
        .show_separator_line(false)
        .show(ctx, |ui| {
            if ui
                .button(format!(
                    "{}",
                    if ui_state.is_open {
                        egui_phosphor::regular::ARROW_FAT_LEFT
                    } else {
                        egui_phosphor::regular::ARROW_FAT_RIGHT
                    }
                ))
                .clicked()
            {
                ui_state.is_open = !ui_state.is_open;
                println!("Top panel button clicked!");
            }
        });
}
