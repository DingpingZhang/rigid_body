use crate::{
    algebra::Vec2,
    shape::{Circle, ParticleLike},
};

pub struct CollisionInfo {
    pub penetration: f32,
    pub normal: Vec2,
}

pub fn detect_collision_circle_and_circle(
    circle1: &Circle,
    circle2: &Circle,
) -> Option<CollisionInfo> {
    let min_distance = circle1.radius + circle2.radius;
    let normal = circle2.particle().position - circle1.particle().position;
    let actual_distance_squared = normal.length_squared();
    if actual_distance_squared < min_distance * min_distance {
        let actual_distance = actual_distance_squared.sqrt();
        return Some(CollisionInfo {
            penetration: min_distance - actual_distance,
            normal: normal / actual_distance,
        });
    }

    return None;
}
