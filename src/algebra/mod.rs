mod vec2;
pub use vec2::Vec2;

mod mat22;
pub use mat22::Mat22;

pub const FLOADT_TOLERANCE: f64 = 1e-6;

pub fn equals_float(x: f64, y: f64) -> bool {
    (x - y).abs() < FLOADT_TOLERANCE
}

pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}
