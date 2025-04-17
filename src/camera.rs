use bevy::input::common_conditions::input_pressed;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::{
    App, Camera2d, Commands, EventReader, IntoSystemConfigs, MouseButton, Query, Startup,
    Transform, Update, With,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, handle_pan.run_if(input_pressed(MouseButton::Left)));
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn handle_pan(
    mut evr_motion: EventReader<MouseMotion>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
) {
    let mut transform = camera_query.single_mut();
    for ev in evr_motion.read() {
        transform.translation.x -= ev.delta.x;
        transform.translation.y += ev.delta.y;
    }
}
