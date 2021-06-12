use super::{Bounded, Collider, Material, MaterialLike, Rectangle, RigidBody, RigidBodyLike, Wall};
use crate::{
    algebra::Float,
    collide_calculation::{
        collide_circle_and_circle, collide_circle_and_rectangle, collide_wall_and_circle,
    },
};

pub struct Circle {
    material: Material,
    rigid_body: RigidBody,
    pub radius: Float,
}

impl Circle {
    pub fn new(material: Material, rigid_body: RigidBody, radius: Float) -> Self {
        Self {
            material,
            rigid_body,
            radius,
        }
    }
}

impl Bounded for Circle {
    fn bound_left(&self) -> Float {
        self.rigid_body.position.x - self.radius
    }

    fn bound_top(&self) -> Float {
        self.rigid_body.position.y + self.radius
    }

    fn bound_right(&self) -> Float {
        self.rigid_body.position.x + self.radius
    }

    fn bound_bottom(&self) -> Float {
        self.rigid_body.position.y - self.radius
    }
}

impl Collider for Circle {
    fn collide_with(&mut self, other: &mut impl Collider) {
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

impl RigidBodyLike for Circle {
    fn rigid_body_mut(&mut self) -> &mut RigidBody {
        &mut self.rigid_body
    }

    fn rigid_body(&self) -> &RigidBody {
        &self.rigid_body
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
