use crate::algebra::Vec2;

mod circle;
pub use self::circle::Circle;

mod rectangle;
pub use self::rectangle::Rectangle;

mod wall;
pub use self::wall::Wall;

pub enum Orientation {
    Left,
    Top,
    Right,
    Bottom,
}

pub trait Bounded {
    fn bound_left(&self) -> f32;
    fn bound_top(&self) -> f32;
    fn bound_right(&self) -> f32;
    fn bound_bottom(&self) -> f32;
}

pub trait RigidBody {
    fn collide_with(&mut self, other: &mut impl RigidBody);
    fn collide_with_wall(&mut self, body: &Wall);
    fn collide_with_circle(&mut self, body: &mut Circle);
    fn collide_with_rectangle(&mut self, body: &mut Rectangle);
}

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub mass: f32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl Particle {
    pub fn inverse_mass(&self) -> f32 {
        if self.mass > 0.0 {
            1.0 / self.mass
        } else {
            f32::INFINITY
        }
    }
}

pub trait ParticleLike {
    fn particle_mut(&mut self) -> &mut Particle;
    fn particle(&self) -> &Particle;
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub restitution: f32,
}

pub trait MaterialLike {
    fn material_mut(&mut self) -> &mut Material;
    fn material(&self) -> &Material;
}
