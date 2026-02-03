use bevy::prelude::*;
use bevy::window::WindowResolution;

mod components;
mod helpers;
mod systems;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Fluid Simulation".to_string(),
                    resolution: WindowResolution::new(800, 800),
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, (setup_camera, systems::setup_boids).chain())
        .add_systems(Update, systems::move_boids)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
