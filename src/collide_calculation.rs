use crate::{
    algebra::Vec2,
    detection_narrow_phase::detect_collision_circle_and_circle,
    detection_narrow_phase::CollisionInfo,
    shape::{Circle, ParticleLike, Rectangle, Shape, Wall},
};

const POSITION_SLOT: f32 = 0.01;
const POSITION_CORRECTION_FACTOR: f32 = 0.2;

pub fn collide_circle_and_circle(circle1: &mut Circle, circle2: &mut Circle) {
    if let Some(CollisionInfo {
        penetration,
        normal,
    }) = detect_collision_circle_and_circle(circle1, circle2)
    {
        let p1 = circle1.particle_mut();
        let p2 = circle2.particle_mut();

        let rel_vel = p2.velocity - p1.velocity;
        let rel_vel_along_normal = rel_vel * normal;

        if rel_vel_along_normal > 0.0 {
            return;
        }

        let restitution = min_f32(p1.restitution, p2.restitution);
        let impulse_scalar =
            -(1.0 + restitution) * rel_vel_along_normal / (p1.inverse_mass() + p2.inverse_mass());
        let impulse = normal * impulse_scalar;

        p1.velocity = p1.velocity - impulse * p1.inverse_mass();
        p2.velocity = p2.velocity + impulse * p2.inverse_mass();

        if penetration > POSITION_SLOT {
            let correction = normal
                * (penetration / (p1.inverse_mass() + p2.inverse_mass()))
                * POSITION_CORRECTION_FACTOR;

            p1.position = p1.position - correction * p1.inverse_mass();
            p2.position = p2.position + correction * p2.inverse_mass();
        }
    }
}

pub fn collide_circle_and_rectangle(_circle: &mut Circle, _rect: &mut Rectangle) {
    todo!()
}

pub fn collide_rectangle_and_rectange(_rect1: &mut Rectangle, _rect2: &mut Rectangle) {
    todo!()
}

pub fn collide_wall_and_circle(wall: &Wall, circle: &mut Circle) {
    match wall.orientation {
        crate::shape::Orientation::Left => {
            if circle.bound_left() < wall.bound {
                let r = circle.radius;
                let mut p = circle.particle_mut();
                p.position = Vec2 {
                    x: wall.bound + r,
                    y: p.position.y,
                };
                p.velocity = Vec2 {
                    x: -p.restitution * p.velocity.x,
                    y: p.velocity.y,
                };
            }
        }
        crate::shape::Orientation::Top => {
            if circle.bound_top() > wall.bound {
                let r = circle.radius;
                let mut p = circle.particle_mut();
                p.position = Vec2 {
                    x: p.position.x,
                    y: wall.bound - r,
                };
                p.velocity = Vec2 {
                    x: p.velocity.x,
                    y: -p.restitution * p.velocity.y,
                };
            }
        }
        crate::shape::Orientation::Right => {
            if circle.bound_right() > wall.bound {
                let r = circle.radius;
                let mut p = circle.particle_mut();
                p.position = Vec2 {
                    x: wall.bound - r,
                    y: p.position.y,
                };
                p.velocity = Vec2 {
                    x: -p.restitution * p.velocity.x,
                    y: p.velocity.y,
                };
            }
        }
        crate::shape::Orientation::Bottom => {
            if circle.bound_bottom() < wall.bound {
                let r = circle.radius;
                let mut p = circle.particle_mut();
                p.position = Vec2 {
                    x: p.position.x,
                    y: wall.bound + r,
                };
                p.velocity = Vec2 {
                    x: p.velocity.x,
                    y: -p.restitution * p.velocity.y,
                };
            }
        }
    }
}

pub fn collide_wall_and_rectangle(_wall: &Wall, _rect: &mut Rectangle) {
    todo!()
}

fn min_f32(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}