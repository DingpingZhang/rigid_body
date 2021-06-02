use super::{Bounded, Material, MaterialLike, Particle, ParticleLike, Rectangle, RigidBody, Wall};
use crate::collide_calculation::{
    collide_circle_and_circle, collide_circle_and_rectangle, collide_wall_and_circle,
};

pub struct Circle {
    material: Material,
    particle: Particle,
    pub radius: f64,
}

impl Circle {
    pub fn new(material: Material, particle: Particle, radius: f64) -> Self {
        Self {
            material,
            particle,
            radius,
        }
    }
}

impl Bounded for Circle {
    fn bound_left(&self) -> f64 {
        self.particle.position.x - self.radius
    }

    fn bound_top(&self) -> f64 {
        self.particle.position.y + self.radius
    }

    fn bound_right(&self) -> f64 {
        self.particle.position.x + self.radius
    }

    fn bound_bottom(&self) -> f64 {
        self.particle.position.y - self.radius
    }
}

impl RigidBody for Circle {
    fn collide_with(&mut self, other: &mut impl RigidBody) {
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

impl MaterialLike for Circle {
    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn material(&self) -> &Material {
        &self.material
    }
}
