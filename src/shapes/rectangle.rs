use super::{Bounded, Circle, Material, MaterialLike, Particle, ParticleLike, RigidBody, Wall};
use crate::collide_calculation::{
    collide_circle_and_rectangle, collide_rectangle_and_rectange, collide_wall_and_rectangle,
};

pub struct Rectangle {
    material: Material,
    particle: Particle,
    pub width: f64,
    pub height: f64,
    pub angle: f64,
}

impl Rectangle {
    pub fn new(
        material: Material,
        particle: Particle,
        width: f64,
        height: f64,
        angle: f64,
    ) -> Self {
        Self {
            material,
            particle,
            width,
            height,
            angle,
        }
    }

    fn bound_width(&self) -> f64 {
        self.height * self.angle.sin().abs() + self.width * self.angle.cos().abs()
    }

    fn bound_height(&self) -> f64 {
        self.height * self.angle.cos().abs() + self.width * self.angle.sin().abs()
    }
}

impl Bounded for Rectangle {
    fn bound_left(&self) -> f64 {
        self.particle.position.x - self.bound_width() / 2.0
    }

    fn bound_top(&self) -> f64 {
        self.particle.position.y + self.bound_height() / 2.0
    }

    fn bound_right(&self) -> f64 {
        self.particle.position.x + self.bound_width() / 2.0
    }

    fn bound_bottom(&self) -> f64 {
        self.particle.position.y - self.bound_height() / 2.0
    }
}

impl RigidBody for Rectangle {
    fn collide_with(&mut self, other: &mut impl RigidBody) {
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

impl ParticleLike for Rectangle {
    fn particle_mut(&mut self) -> &mut Particle {
        &mut self.particle
    }

    fn particle(&self) -> &Particle {
        &self.particle
    }
}

impl MaterialLike for Rectangle {
    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn material(&self) -> &Material {
        &self.material
    }
}
