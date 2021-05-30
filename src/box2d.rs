use crate::{
    detection_broad_phase::{detect_by_broad_phase, ShapeIndexPair},
    shape::{RigidBody, Shape, Wall},
};

pub struct Box<T>
where
    T: RigidBody + Shape,
{
    pub wall_left: Wall,
    pub wall_top: Wall,
    pub wall_right: Wall,
    pub wall_bottom: Wall,
    shapes: Vec<T>,
}

impl<T> Box<T>
where
    T: RigidBody + Shape,
{
    pub fn next_frame(&mut self, duration: f32) {
        for ShapeIndexPair(index1, index2) in detect_by_broad_phase(&self.shapes()) {
            let shapes = &mut self.shapes;
            let shape1 = &mut shapes[index1];
            let shape2 = &mut shapes[index2];
            shape1.collide(shape2);
        }
    }

    pub fn shapes(&self) -> Vec<&T> {
        let mut results: Vec<&T> = Vec::new();
        for shape in &self.shapes {
            results.push(shape)
        }

        return results;
    }

    pub fn shapes_mut(&mut self) -> Vec<&mut T> {
        let mut results: Vec<&mut T> = Vec::new();
        for shape in &mut self.shapes {
            results.push(shape)
        }

        return results;
    }
}
