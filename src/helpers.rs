use bevy::prelude::*;

pub fn get_avoidance_force(position: Vec2, neighbor: Vec2) {
    let dxy = position - neighbor;
    let distance = position.distance_squared(neighbor);

    let force = dxy / distance;
}

pub fn point_in_arc(
    center: Vec2,
    radius: f32,
    // Arc definition
    start_angle: f32, // in radians
    arc_span: f32,    // how wide the arc is, in radians
    point: Vec2,
) -> bool {
    // 1. Check if point is within radius
    let dx = point.x - center.x;
    let dy = point.y - center.y;
    let distance = (dx * dx + dy * dy).sqrt();

    if distance > radius {
        return false;
    }

    // 2. Calculate angle of the point from center
    let point_angle = dy.atan2(dx);

    // 3. Normalize angles to [0, 2π]
    let mut normalized_point = point_angle;
    let mut normalized_start = start_angle;

    if normalized_point < 0.0 {
        normalized_point += 2.0 * std::f32::consts::PI;
    }
    if normalized_start < 0.0 {
        normalized_start += 2.0 * std::f32::consts::PI;
    }

    // 4. Check if point angle is within arc span
    let end_angle = normalized_start + arc_span;

    if end_angle <= 2.0 * std::f32::consts::PI {
        // Arc doesn't wrap around
        normalized_point >= normalized_start && normalized_point <= end_angle
    } else {
        // Arc wraps around 0/2π
        normalized_point >= normalized_start
            || normalized_point <= (end_angle % (2.0 * std::f32::consts::PI))
    }
}
