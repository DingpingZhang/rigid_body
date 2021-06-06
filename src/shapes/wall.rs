use super::{Circle, Material, MaterialLike, Orientation, Rectangle, Collider};
use crate::collide_calculation::{collide_wall_and_circle, collide_wall_and_rectangle};

pub struct Wall {
    material: Material,
    pub bound: f64,
    pub orientation: Orientation,
}

impl Wall {
    pub fn new(material: Material, bound: f64, orientation: Orientation) -> Self {
        Self {
            material,
            bound,
            orientation,
        }
    }
}

impl Collider for Wall {
    fn collide_with(&mut self, other: &mut impl Collider) {
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

impl MaterialLike for Wall {
    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn material(&self) -> &Material {
        &self.material
    }
}
