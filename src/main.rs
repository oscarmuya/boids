use bevy::prelude::*;
use bevy::window::WindowResolution;

mod components;
mod helpers;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fluid Simulation".to_string(),
                resolution: WindowResolution::new(800, 800),
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup,
            (
                setup_camera,
                systems::setup_boids,
                systems::setup_master_boid,
            )
                .chain(),
        )
        .add_systems(Update, (systems::move_boids, systems::move_master_boid))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
