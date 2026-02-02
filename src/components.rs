use bevy::prelude::*;

#[derive(Component)]
pub struct Boid;

#[derive(Component)]
pub struct MasterBoid;

#[derive(Component)]
pub struct BoidFOV;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
