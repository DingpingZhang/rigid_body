use super::{Bounded, Circle, Collider, Material, MaterialLike, RigidBody, RigidBodyLike, Wall};
use crate::{
    algebra::Float,
    collide_calculation::{
        collide_circle_and_rectangle, collide_rectangle_and_rectange, collide_wall_and_rectangle,
    },
};

pub struct Rectangle {
    material: Material,
    rigid_body: RigidBody,
    pub width: Float,
    pub height: Float,
    pub angle: Float,
}

impl Rectangle {
    pub fn new(
        material: Material,
        rigid_body: RigidBody,
        width: Float,
        height: Float,
        angle: Float,
    ) -> Self {
        Self {
            material,
            rigid_body,
            width,
            height,
            angle,
        }
    }

    fn bound_width(&self) -> Float {
        self.height * self.angle.sin().abs() + self.width * self.angle.cos().abs()
    }

    fn bound_height(&self) -> Float {
        self.height * self.angle.cos().abs() + self.width * self.angle.sin().abs()
    }
}

impl Bounded for Rectangle {
    fn bound_left(&self) -> Float {
        self.rigid_body.position.x - self.bound_width() / 2.0
    }

    fn bound_top(&self) -> Float {
        self.rigid_body.position.y + self.bound_height() / 2.0
    }

    fn bound_right(&self) -> Float {
        self.rigid_body.position.x + self.bound_width() / 2.0
    }

    fn bound_bottom(&self) -> Float {
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
