use std::ops::{Add, Div, Mul, Sub};

use super::{equals_float, Float};

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: Float,
    pub y: Float,
}

impl Vec2 {
    pub fn new(x: Float, y: Float) -> Self {
        Self { x, y }
    }

    pub fn length_squared(self) -> Float {
        self.x * self.x + self.y * self.y
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        equals_float(self.x, other.x) && equals_float(self.y, other.y)
    }
}

impl Eq for Vec2 {}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = Float;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Mul<Float> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Float) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<Float> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: Float) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
