use super::{Bounded, Circle, Material, MaterialLike, RigidBody, Collider, RigidBodyLike, Wall};
use crate::collide_calculation::{
    collide_circle_and_rectangle, collide_rectangle_and_rectange, collide_wall_and_rectangle,
};

pub struct Rectangle {
    material: Material,
    rigid_body: RigidBody,
    pub width: f64,
    pub height: f64,
    pub angle: f64,
}

impl Rectangle {
    pub fn new(
        material: Material,
        rigid_body: RigidBody,
        width: f64,
        height: f64,
        angle: f64,
    ) -> Self {
        Self {
            material,
            rigid_body,
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
        self.rigid_body.position.x - self.bound_width() / 2.0
    }

    fn bound_top(&self) -> f64 {
        self.rigid_body.position.y + self.bound_height() / 2.0
    }

    fn bound_right(&self) -> f64 {
        self.rigid_body.position.x + self.bound_width() / 2.0
    }

    fn bound_bottom(&self) -> f64 {
        self.rigid_body.position.y - self.bound_height() / 2.0
    }
}

impl Collider for Rectangle {
    fn collide_with(&mut self, other: &mut impl Collider) {
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

impl RigidBodyLike for Rectangle {
    fn rigid_body_mut(&mut self) -> &mut RigidBody {
        &mut self.rigid_body
    }

    fn rigid_body(&self) -> &RigidBody {
        &self.rigid_body
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
