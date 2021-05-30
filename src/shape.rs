use crate::{
    algebra::Vec2,
    collide_calculation::{
        collide_circle_and_circle, collide_circle_and_rectangle, collide_rectangle_and_rectange,
        collide_wall_and_circle, collide_wall_and_rectangle,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub mass: f32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub restitution: f32,
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

pub trait Bounded {
    fn bound_left(&self) -> f32;
    fn bound_top(&self) -> f32;
    fn bound_right(&self) -> f32;
    fn bound_bottom(&self) -> f32;
}

pub enum Orientation {
    Left,
    Top,
    Right,
    Bottom,
}

pub struct Wall {
    pub bound: f32,
    pub orientation: Orientation,
}

impl RigidBody for Wall {
    fn collide(&mut self, other: &mut impl RigidBody) {
        other.collide_with_wall(self);
    }

    fn collide_with_wall(&mut self, _body: &Wall) {
        panic!("Not Supported")
    }

    fn collide_with_circle(&mut self, body: &mut Circle) {
        collide_wall_and_circle(self, body);
    }

    fn collide_with_rectangle(&mut self, body: &mut Rectangle) {
        collide_wall_and_rectangle(self, body);
    }
}

pub struct Circle {
    particle: Particle,
    pub radius: f32,
}

impl Circle {
    pub fn new(particle: Particle, radius: f32) -> Self {
        Self { particle, radius }
    }
}

impl Bounded for Circle {
    fn bound_left(&self) -> f32 {
        self.particle.position.x - self.radius
    }

    fn bound_top(&self) -> f32 {
        self.particle.position.y + self.radius
    }

    fn bound_right(&self) -> f32 {
        self.particle.position.x + self.radius
    }

    fn bound_bottom(&self) -> f32 {
        self.particle.position.y - self.radius
    }
}

impl RigidBody for Circle {
    fn collide(&mut self, other: &mut impl RigidBody) {
        other.collide_with_circle(self);
    }

    fn collide_with_wall(&mut self, body: &Wall) {
        collide_wall_and_circle(body, self);
    }

    fn collide_with_circle(&mut self, body: &mut Circle) {
        collide_circle_and_circle(self, body);
    }

    fn collide_with_rectangle(&mut self, body: &mut Rectangle) {
        collide_circle_and_rectangle(self, body);
    }
}

impl ParticleLike for Circle {
    fn particle_mut(&mut self) -> &mut Particle {
        &mut self.particle
    }

    fn particle(&self) -> &Particle {
        &self.particle
    }
}

pub struct Rectangle {
    particle: Particle,
    pub width: f32,
    pub height: f32,
    pub angle: f32,
}

pub trait RigidBody {
    fn collide(&mut self, other: &mut impl RigidBody);
    fn collide_with_wall(&mut self, body: &Wall);
    fn collide_with_circle(&mut self, body: &mut Circle);
    fn collide_with_rectangle(&mut self, body: &mut Rectangle);
}

impl Rectangle {
    pub fn new(particle: Particle, width: f32, height: f32, angle: f32) -> Self {
        Self {
            particle,
            width,
            height,
            angle,
        }
    }

    fn bound_width(&self) -> f32 {
        self.height * self.angle.sin().abs() + self.width * self.angle.cos().abs()
    }

    fn bound_height(&self) -> f32 {
        self.height * self.angle.cos().abs() + self.width * self.angle.sin().abs()
    }
}

impl Bounded for Rectangle {
    fn bound_left(&self) -> f32 {
        self.particle.position.x - self.bound_width() / 2.0
    }

    fn bound_top(&self) -> f32 {
        self.particle.position.y + self.bound_height() / 2.0
    }

    fn bound_right(&self) -> f32 {
        self.particle.position.x + self.bound_width() / 2.0
    }

    fn bound_bottom(&self) -> f32 {
        self.particle.position.y - self.bound_height() / 2.0
    }
}

impl RigidBody for Rectangle {
    fn collide(&mut self, other: &mut impl RigidBody) {
        other.collide_with_rectangle(self);
    }

    fn collide_with_wall(&mut self, body: &Wall) {
        collide_wall_and_rectangle(body, self);
    }

    fn collide_with_circle(&mut self, body: &mut Circle) {
        collide_circle_and_rectangle(body, self);
    }

    fn collide_with_rectangle(&mut self, body: &mut Rectangle) {
        collide_rectangle_and_rectange(self, body);
    }
}

// impl<'a> ParticleLike<'a> for Rectangle {
//     fn particle(&'a mut self) -> &'a mut Particle {
//         &mut self.particle
//     }
// }
