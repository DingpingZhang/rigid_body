use crate::{
    algebra::Float,
    detection_broad_phase::{detect_by_broad_phase, ShapeIndexPair},
    shapes::{Bounded, Collider, RigidBody, RigidBodyLike, Wall},
};

pub struct Box<T>
where
    T: Collider + Bounded + RigidBodyLike,
{
    pub wall_left: Wall,
    pub wall_top: Wall,
    pub wall_right: Wall,
    pub wall_bottom: Wall,
    shapes: Vec<T>,
}

impl<T> Box<T>
where
    T: Collider + Bounded + RigidBodyLike,
{
    pub fn next_frame(&mut self, duration: Float) {
        for shape in self.shapes.iter_mut() {
            drive_particle(shape.rigid_body_mut(), duration);
        }

        for index_pair in detect_by_broad_phase(&self.shapes.iter().collect()) {
            let (shape1, shape2) = self.get_shape_pair_mut(index_pair);
            shape1.collide_with(shape2);
        }

        for shape in self.shapes.iter_mut() {
            shape.collide_with(&mut self.wall_left);
            shape.collide_with(&mut self.wall_top);
            shape.collide_with(&mut self.wall_right);
            shape.collide_with(&mut self.wall_bottom);
        }
    }

    fn get_shape_pair_mut(&mut self, index_pair: ShapeIndexPair) -> (&mut T, &mut T) {
        let mut shapes = self.shapes.iter_mut();
        let shape1 = shapes.nth(index_pair.0).unwrap();
        let shape2 = shapes.nth(index_pair.1).unwrap();
        return (shape1, shape2);
    }
}

fn drive_particle(particle: &mut RigidBody, duration: Float) {
    particle.position = particle.position
        + particle.velocity * duration
        + particle.acceleration * (duration * duration) / 2.0;
    particle.velocity = particle.velocity + particle.acceleration * duration;
}
