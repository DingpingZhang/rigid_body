use crate::shape::Shape;
use std::{collections::HashSet, usize};

#[derive(Clone, Copy)]
pub struct ShapeIndexPair(pub usize, pub usize);

impl PartialEq for ShapeIndexPair {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0 && self.1 == rhs.1 || self.0 == rhs.1 && self.1 == rhs.0
    }
}

impl Eq for ShapeIndexPair {}

struct ShapeBoundPoint {
    index: usize,
    bound: f32,
    is_start: bool,
}

pub fn detect_by_broad_phase(shapes: &Vec<&impl Shape>) -> Vec<ShapeIndexPair> {
    let x_axis_results = detect_axis_by_broad_phase(
        shapes,
        |shape| shape.bound_left(),
        |shape| shape.bound_right(),
    );
    let y_axis_results = detect_axis_by_broad_phase(
        shapes,
        |shape| shape.bound_bottom(),
        |shape| shape.bound_top(),
    );

    let mut results: Vec<ShapeIndexPair> = Vec::new();
    for x_axis_result in x_axis_results.iter() {
        for y_axis_result in y_axis_results.iter() {
            if x_axis_result == y_axis_result {
                results.push(*x_axis_result)
            }
        }
    }

    return results;
}

fn detect_axis_by_broad_phase(
    shapes: &Vec<&impl Shape>,
    get_min: impl Fn(&dyn Shape) -> f32,
    get_max: impl Fn(&dyn Shape) -> f32,
) -> Vec<ShapeIndexPair> {
    let mut points: Vec<ShapeBoundPoint> = Vec::new();
    for i in 0..shapes.len() {
        let shape = shapes[i];
        points.push(ShapeBoundPoint {
            index: i,
            bound: get_min(shape),
            is_start: true,
        });
        points.push(ShapeBoundPoint {
            index: i,
            bound: get_max(shape),
            is_start: false,
        });
    }

    return detect_by_broad_phase_core(&mut points);
}

fn detect_by_broad_phase_core(shapes: &mut Vec<ShapeBoundPoint>) -> Vec<ShapeIndexPair> {
    shapes.sort_by(|x, y| x.bound.partial_cmp(&y.bound).unwrap());

    let mut activated_ids: HashSet<usize> = HashSet::new();
    let mut result: Vec<ShapeIndexPair> = Vec::new();
    for ShapeBoundPoint {
        index,
        bound: _,
        is_start,
    } in shapes
    {
        if *is_start {
            for activated_id in &activated_ids {
                result.push(ShapeIndexPair(*index, *activated_id));
            }

            activated_ids.insert(*index);
        } else {
            activated_ids.remove(index);
        }
    }

    return result;
}
