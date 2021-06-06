use std::ops::Mul;

use super::{equals_float, Vec2};

#[derive(Debug, Clone, Copy)]
pub struct Mat22 {
    pub m00: f64,
    pub m01: f64,
    pub m10: f64,
    pub m11: f64,
}

impl Mat22 {
    pub fn new(m00: f64, m01: f64, m10: f64, m11: f64) -> Self {
        Self { m00, m01, m10, m11 }
    }

    pub fn rotation(radian: f64) -> Self {
        let cos = radian.cos();
        let sin = radian.sin();
        Mat22::new(cos, -sin, sin, cos)
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.m00 * self.m11 - self.m01 * self.m10;
        if equals_float(det, 0.0) {
            None
        } else {
            Some(Mat22::new(
                self.m11 / det,
                -self.m01 / det,
                -self.m10 / det,
                self.m00 / det,
            ))
        }
    }
}

impl PartialEq for Mat22 {
    fn eq(&self, other: &Self) -> bool {
        self.m00 == other.m00
            && self.m01 == other.m01
            && self.m10 == other.m10
            && self.m11 == other.m11
    }
}

impl Eq for Mat22 {}

impl Mul for Mat22 {
    type Output = Mat22;

    fn mul(self, rhs: Self) -> Self::Output {
        Mat22::new(
            self.m00 * rhs.m00 + self.m01 * rhs.m10,
            self.m00 * rhs.m01 + self.m01 * rhs.m11,
            self.m10 * rhs.m00 + self.m11 * rhs.m10,
            self.m10 * rhs.m01 + self.m11 * rhs.m11,
        )
    }
}

impl Mul<Mat22> for f64 {
    type Output = Mat22;

    fn mul(self, rhs: Mat22) -> Self::Output {
        Mat22::new(
            self * rhs.m00,
            self * rhs.m01,
            self * rhs.m10,
            self * rhs.m11,
        )
    }
}

impl Mul<f64> for Mat22 {
    type Output = Mat22;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl Mul<Vec2> for Mat22 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::new(
            self.m00 * rhs.x + self.m01 * rhs.y,
            self.m10 * rhs.x + self.m11 * rhs.y,
        )
    }
}
