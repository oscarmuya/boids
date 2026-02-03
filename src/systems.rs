use bevy::{platform::collections::HashMap, prelude::*};
use rand::Rng;

use crate::{
    components::{Boid, Velocity},
    helpers,
};

const BOID_SIZE: f32 = 7.0;
const VELOCITY: f32 = 300.0;
const NUMBER_OF_BOIDS: u32 = 200;

const PARTITION_SIZE: f32 = 70.0;
const SEPARATION_ANGLE: f32 = 90.0;
const MAX_STEERING_FORCE: f32 = 100.0;

const SEPARATION_RADIUS: f32 = 50.0;
const ALIGNMENT_RADIUS: f32 = 100.0;
const COHESION_RADIUS: f32 = 150.0;

const SEPARATION_STRENGTH: f32 = 2.5;
const ALIGNMENT_STRENGTH: f32 = 1.0;
const COHESION_STRENGTH: f32 = 2.0;

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

pub fn move_boids(
    mut query: Query<(&mut Transform, &mut Velocity), With<Boid>>,
    time: Res<Time>,
    window: Single<&Window>,
) {
    let dt = time.delta_secs();
    let h_width = window.width() / 2.0;
    let h_height = window.height() / 2.0;

    let mut list: Vec<_> = query.iter_mut().collect();
    let mut spatial_hash: HashMap<(i32, i32), Vec<usize>> = HashMap::new();

    for (i, (t, _)) in list.iter().enumerate() {
        let key_x = (t.translation.x / PARTITION_SIZE).floor() as i32;
        let key_y = (t.translation.y / PARTITION_SIZE).floor() as i32;
        let cell = (key_x, key_y);
        spatial_hash.entry(cell).or_default().push(i);
    }

    for i in 0..list.len() {
        // wrap boids that move out from screen
        // Rotate boid to face direction of movement we subtract pi/2
        // since our boid starts when pointing up.
        let angle = list[i].1.y.atan2(list[i].1.x) - std::f32::consts::FRAC_PI_2;
        let position_i = list[i].0.translation.truncate();

        let mut separation = Vec2::ZERO;
        let mut center_of_mass = Vec2::ZERO;
        let mut num_neighbors_alignment = 0.0;
        let mut num_neighbors_cohesion = 0.0;
        let mut velocity_neighbors = Vec2::ZERO;

        let grid_x = (position_i.x / PARTITION_SIZE).floor() as i32;
        let grid_y = (position_i.y / PARTITION_SIZE).floor() as i32;

        // find the neighbors
        for x_off in -1..=1 {
            for y_off in -1..=1 {
                let key = (grid_x + x_off, grid_y + y_off);
                if let Some(neighbors) = spatial_hash.get(&key) {
                    for &j in neighbors {
                        let position_j = list[j].0.translation.truncate();
                        let away_vector = position_i - position_j;
                        let dxy_2 = away_vector.length_squared();

                        // 1. alignment
                        if dxy_2 < ALIGNMENT_RADIUS * ALIGNMENT_RADIUS {
                            velocity_neighbors.x += list[j].1.x;
                            velocity_neighbors.y += list[j].1.y;
                            num_neighbors_alignment += 1.0;
                        }
                        // 2 cohesion
                        if dxy_2 < COHESION_RADIUS * COHESION_RADIUS {
                            center_of_mass += position_j;
                            num_neighbors_cohesion += 1.0;
                        }

                        // 3 separation
                        if helpers::point_in_arc(
                            position_i,
                            SEPARATION_RADIUS,
                            angle,
                            SEPARATION_ANGLE,
                            position_j,
                        ) && dxy_2 > 0.0001
                        {
                            let distance = dxy_2.sqrt();
                            // Weight by inverse distance (closer = stronger push)
                            let force = away_vector / (distance * distance);
                            separation += force;
                        }
                    }
                }
            }
        }

        // calculate cohesion steering
        let mut cohesion = Vec2::ZERO;
        if num_neighbors_cohesion > 0.0 {
            center_of_mass /= num_neighbors_cohesion;
            let desired_direction = center_of_mass - position_i;

            if desired_direction.length() > 0.0 {
                let desired_velocity = desired_direction.normalize() * VELOCITY;
                cohesion = desired_velocity
                    - Vec2 {
                        x: list[i].1.x,
                        y: list[i].1.y,
                    };

                if cohesion.length() > MAX_STEERING_FORCE {
                    cohesion = cohesion.normalize() * MAX_STEERING_FORCE;
                }
            }
        }

        // Calculate alignment steering
        let mut alignment = Vec2::ZERO;
        if num_neighbors_alignment > 0.0 {
            let mut avg_neighbor_velocity = velocity_neighbors / num_neighbors_alignment;
            if avg_neighbor_velocity.length() > 0.0 {
                avg_neighbor_velocity = avg_neighbor_velocity.normalize() * VELOCITY;
            }
            alignment = avg_neighbor_velocity
                - Vec2 {
                    x: list[i].1.x,
                    y: list[i].1.y,
                };

            // Limit steering force
            if alignment.length() > MAX_STEERING_FORCE {
                alignment = alignment.normalize() * MAX_STEERING_FORCE;
            }
        }

        // Normalize separation
        if separation.length() > 0.0 {
            separation = separation.normalize() * MAX_STEERING_FORCE;
        }

        // Combine all forces into acceleration
        let mut acceleration = Vec2::ZERO;
        acceleration += alignment * ALIGNMENT_STRENGTH;
        acceleration += separation * SEPARATION_STRENGTH;
        acceleration += cohesion * COHESION_STRENGTH;

        // Update velocity
        list[i].1.x += acceleration.x * dt;
        list[i].1.y += acceleration.y * dt;

        // Clamp velocity magnitude
        let speed_2 = list[i].1.x * list[i].1.x + list[i].1.y * list[i].1.y;
        if speed_2 > VELOCITY * VELOCITY {
            let speed = speed_2.sqrt();
            list[i].1.x = (list[i].1.x / speed) * VELOCITY;
            list[i].1.y = (list[i].1.y / speed) * VELOCITY;
        }

        // Wrap around screen edges
        let position = list[i].0.translation;
        if position.x > h_width {
            list[i].0.translation.x = -h_width;
        } else if position.x < -h_width {
            list[i].0.translation.x = h_width;
        }
        if position.y > h_height {
            list[i].0.translation.y = -h_height;
        } else if position.y < -h_height {
            list[i].0.translation.y = h_height;
        }

        // Update position
        list[i].0.translation.x += list[i].1.x * dt;
        list[i].0.translation.y += list[i].1.y * dt;

        // Update rotation to face movement direction
        let angle = list[i].1.y.atan2(list[i].1.x) - std::f32::consts::FRAC_PI_2;
        list[i].0.rotation = Quat::from_rotation_z(angle);
    }
}
