use crate::algebra::{Float, Vec2};

#[cfg(test)]
mod tests;

mod circle;
pub use self::circle::Circle;

mod rectangle;
pub use self::rectangle::Rectangle;

mod wall;
pub use self::wall::Wall;

#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Left,
    Top,
    Right,
    Bottom,
}

pub trait Bounded {
    fn bound_left(&self) -> Float;
    fn bound_top(&self) -> Float;
    fn bound_right(&self) -> Float;
    fn bound_bottom(&self) -> Float;
}

pub trait Collider {
    fn collide_with(&mut self, other: &mut impl Collider);
    fn collide_with_wall(&mut self, body: &Wall);
    fn collide_with_circle(&mut self, body: &mut Circle);
    fn collide_with_rectangle(&mut self, body: &mut Rectangle);
}

#[derive(Debug, Clone, Copy)]
pub struct RigidBody {
    pub mass: Float,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl RigidBody {
    pub fn particle(mass: Float, position: Vec2, velocity: Vec2, acceleration: Vec2) -> Self {
        Self {
            mass,
            position,
            velocity,
            acceleration,
        }
    }

    pub fn inverse_mass(&self) -> Float {
        if self.mass > 0.0 {
            1.0 / self.mass
        } else {
            Float::INFINITY
        }
    }
}

pub trait RigidBodyLike {
    fn rigid_body_mut(&mut self) -> &mut RigidBody;
    fn rigid_body(&self) -> &RigidBody;
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub restitution: Float,
}

pub trait MaterialLike {
    fn material_mut(&mut self) -> &mut Material;
    fn material(&self) -> &Material;
}
