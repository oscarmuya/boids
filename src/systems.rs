use bevy::prelude::*;
use rand::Rng;

use crate::{
    components::{Boid, BoidFOV, MasterBoid, Velocity},
    helpers,
};

const BOID_SIZE: f32 = 8.0;
const VELOCITY: f32 = 150.0;
const NUMBER_OF_BOIDS: u32 = 80;
const FOV_RADIUS: f32 = 100.0;
const FOV_ANGLE: f32 = 90.0;

const AVOIDANCE_STRENGTH: f32 = 150.0;

pub fn setup_master_boid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    for _ in 0..4 {
        // spawn the master boid
        let width = window.width() / 2.0;
        let height = window.height() / 2.0;
        let mut rng = rand::rng();
        let x = rng.random_range(-width..width);
        let y = rng.random_range(-height..height);

        let vx = if rng.random_bool(0.5) {
            VELOCITY
        } else {
            -VELOCITY
        };

        let vy = if rng.random_bool(0.5) {
            VELOCITY
        } else {
            -VELOCITY
        };

        let triangle = meshes.add(Triangle2d::new(
            Vec2::Y * (BOID_SIZE + 4.0),
            Vec2::new(-BOID_SIZE, -BOID_SIZE),
            Vec2::new(BOID_SIZE, -BOID_SIZE),
        ));

        // boid
        let circle = meshes.add(Circle::new(FOV_RADIUS));
        let fov_material = materials.add(Color::srgba(0.8, 0.8, 0.8, 0.3));
        let material = materials.add(Color::srgb(1.0, 0.0, 0.0));

        commands
            .spawn((
                Mesh2d(triangle),
                MeshMaterial2d(material),
                Transform::from_xyz(x, y, 0.0),
                Velocity { x: vx, y: vy },
                MasterBoid,
            ))
            .with_children(|parent| {
                parent.spawn((
                    BoidFOV,
                    Mesh2d(circle),
                    MeshMaterial2d(fov_material),
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ));
            });
    }
}

pub fn setup_boids(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    for _ in 0..NUMBER_OF_BOIDS {
        // spawn boids randomly in the window
        let width = window.width() / 2.0;
        let height = window.height() / 2.0;
        let mut rng = rand::rng();
        let x = rng.random_range(-width..width);
        let y = rng.random_range(-height..height);

        let vx = if rng.random_bool(0.5) {
            VELOCITY
        } else {
            -VELOCITY
        };

        let vy = if rng.random_bool(0.5) {
            VELOCITY
        } else {
            -VELOCITY
        };

        let triangle = meshes.add(Triangle2d::new(
            Vec2::Y * (BOID_SIZE + 4.0),
            Vec2::new(-BOID_SIZE, -BOID_SIZE),
            Vec2::new(BOID_SIZE, -BOID_SIZE),
        ));
        let material = materials.add(Color::srgb(0.0, 0.45, 0.7));

        commands.spawn((
            Mesh2d(triangle),
            MeshMaterial2d(material.clone()),
            Transform::from_xyz(x, y, 0.0),
            Velocity { x: vx, y: vy },
            Boid,
        ));
    }
}

pub fn move_master_boid(
    mut query: Query<(&mut Transform, &mut Velocity), With<MasterBoid>>,
    query_2: Query<&Transform, (With<Boid>, Without<MasterBoid>)>,
    time: Res<Time>,
    window: Single<&Window>,
) {
    let dt = time.delta_secs();
    let h_width = window.width() / 2.0;
    let h_height = window.height() / 2.0;

    for (mut transform, mut velocity) in &mut query {
        // wrap boids that move out from screen
        // Rotate boid to face direction of movement we subtract pi/2
        // since our boid starts when pointing up.
        let angle = velocity.y.atan2(velocity.x) - std::f32::consts::FRAC_PI_2;
        let position_i = transform.translation.truncate();

        // 1. avoid other boids
        let mut avoidance = Vec2::ZERO;
        for transform_j in &query_2 {
            let position_j = transform_j.translation.truncate();
            if helpers::point_in_arc(position_i, FOV_RADIUS, angle, FOV_ANGLE, position_j) {
                let away_vector = position_i - position_j;
                let distance = position_i.distance(position_j);
                // Weight by inverse distance (closer = stronger push)
                let force = away_vector / (distance * distance);
                avoidance += force;
            }
        }

        if avoidance.length() > 0.0 {
            avoidance = avoidance.normalize() * AVOIDANCE_STRENGTH;
        }

        velocity.x = (velocity.x + avoidance.x * dt).clamp(-VELOCITY, VELOCITY);
        velocity.y = (velocity.y + avoidance.y * dt).clamp(-VELOCITY, VELOCITY);

        let position = transform.translation;

        if position.x > h_width {
            transform.translation.x = -h_width;
        } else if position.x < -h_width {
            transform.translation.x = h_width;
        }

        if position.y > h_height {
            transform.translation.y = -h_height;
        } else if position.y < -h_height {
            transform.translation.y = h_height;
        }

        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;

        let angle = velocity.y.atan2(velocity.x) - std::f32::consts::FRAC_PI_2;
        // Set rotation around z-axis (2D rotation)
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

pub fn move_boids(
    mut query: Query<(&mut Transform, &mut Velocity), With<Boid>>,
    time: Res<Time>,
    window: Single<&Window>,
) {
    let dt = time.delta_secs();
    let h_width = window.width() / 2.0;
    let h_height = window.height() / 2.0;

    let mut list: Vec<_> = query.iter_mut().collect();

    for i in 0..list.len() {
        // wrap boids that move out from screen
        // Rotate boid to face direction of movement we subtract pi/2
        // since our boid starts when pointing up.
        let angle = list[i].1.y.atan2(list[i].1.x) - std::f32::consts::FRAC_PI_2;
        let position_i = list[i].0.translation.truncate();

        // 1. avoid other boids
        let mut avoidance = Vec2::ZERO;
        for (transform_j, _) in list.iter() {
            let position_j = transform_j.translation.truncate();
            if helpers::point_in_arc(position_i, FOV_RADIUS, angle, FOV_ANGLE, position_j) {
                let away_vector = position_i - position_j;
                let distance = position_i.distance(position_j);

                if distance > 0.00001 {
                    // Weight by inverse distance (closer = stronger push)
                    let force = away_vector / (distance * distance);
                    avoidance += force;
                }
            }
        }

        if avoidance.length() > 0.0 {
            avoidance = avoidance.normalize() * AVOIDANCE_STRENGTH;
        }

        list[i].1.x = (list[i].1.x + avoidance.x * dt).clamp(-VELOCITY, VELOCITY);
        list[i].1.y = (list[i].1.y + avoidance.y * dt).clamp(-VELOCITY, VELOCITY);

        if position_i.x > h_width {
            list[i].0.translation.x = -h_width;
        } else if position_i.x < -h_width {
            list[i].0.translation.x = h_width;
        }

        if position_i.y > h_height {
            list[i].0.translation.y = -h_height;
        } else if position_i.y < -h_height {
            list[i].0.translation.y = h_height;
        }

        list[i].0.translation.x += list[i].1.x * dt;
        list[i].0.translation.y += list[i].1.y * dt;

        let angle = list[i].1.y.atan2(list[i].1.x) - std::f32::consts::FRAC_PI_2;
        // Set rotation around z-axis (2D rotation)
        list[i].0.rotation = Quat::from_rotation_z(angle);
    }
}
