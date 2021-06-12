use std::ops::Mul;

use super::{equals_float, Float, Vec2};

#[derive(Debug, Clone, Copy)]
pub struct Mat22 {
    pub m00: Float,
    pub m01: Float,
    pub m10: Float,
    pub m11: Float,
}

impl Mat22 {
    pub fn new(m00: Float, m01: Float, m10: Float, m11: Float) -> Self {
        Self { m00, m01, m10, m11 }
    }

    pub fn rotation(radian: Float) -> Self {
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

impl Mul<Mat22> for Float {
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

impl Mul<Float> for Mat22 {
    type Output = Mat22;

    fn mul(self, rhs: Float) -> Self::Output {
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
