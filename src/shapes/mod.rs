use crate::algebra::Vec2;

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
    fn bound_left(&self) -> f64;
    fn bound_top(&self) -> f64;
    fn bound_right(&self) -> f64;
    fn bound_bottom(&self) -> f64;
}

pub trait Collide {
    fn collide_with(&mut self, other: &mut impl Collide);
    fn collide_with_wall(&mut self, body: &Wall);
    fn collide_with_circle(&mut self, body: &mut Circle);
    fn collide_with_rectangle(&mut self, body: &mut Rectangle);
}

#[derive(Debug, Clone, Copy)]
pub struct RigidBody {
    pub mass: f64,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl RigidBody {
    pub fn inverse_mass(&self) -> f64 {
        if self.mass > 0.0 {
            1.0 / self.mass
        } else {
            f64::INFINITY
        }
    }
}

pub trait RigidBodyLike {
    fn rigid_body_mut(&mut self) -> &mut RigidBody;
    fn rigid_body(&self) -> &RigidBody;
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub restitution: f64,
}

pub trait MaterialLike {
    fn material_mut(&mut self) -> &mut Material;
    fn material(&self) -> &Material;
}
