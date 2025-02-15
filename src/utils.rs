const EPSILON: f64 = 0.0005;

pub fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
