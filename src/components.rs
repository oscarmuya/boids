use bevy::prelude::*;

#[derive(Component)]
pub struct Boid;

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
