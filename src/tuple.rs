use std::ops::{Add, Div, Mul, Sub};

const EPSILON: f64 = 0.0005;

fn equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[derive(Debug, PartialEq)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    fn from(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn to_point(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    fn to_vector(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self.w = -self.w;
    }

    fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    fn normalize(&self) -> Self {
        Self {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
            z: self.z / self.magnitude(),
            w: self.w / self.magnitude(),
        }
    }

    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    fn cross(&self, other: &Self) -> Self {
        Self::to_vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut w = 0.0;

        if self.w == 1.0 || other.w == 1.0 {
            w = 1.0
        }

        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut w = 1.0;

        if (self.w == 1.0 && rhs.w == 1.0) || (self.w == 0.0 && rhs.w == 0.0) {
            w = 0.0;
        }

        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<i32> for Tuple {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x / rhs as f64,
            y: self.y / rhs as f64,
            z: self.z / rhs as f64,
            w: self.w / rhs as f64,
        }
    }
}

#[test]
fn should_be_point() {
    let a = Tuple::from(4.3, -4.2, 3.1, 1.0);

    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert!(a.is_point());
}

#[test]
fn should_be_vector() {
    let a = Tuple::from(4.3, -4.2, 3.1, 0.0);

    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert!(!a.is_point());
}

#[test]
fn to_point_create_tuple() {
    let a = Tuple::to_point(4.0, -4.0, 3.0);

    assert_eq!(a.x, 4.0);
    assert_eq!(a.y, -4.0);
    assert_eq!(a.z, 3.0);
    assert!(a.is_point());
}

#[test]
fn to_vector_create_tuple() {
    let a = Tuple::to_vector(4.0, -4.0, 3.0);

    assert_eq!(a.x, 4.0);
    assert_eq!(a.y, -4.0);
    assert_eq!(a.z, 3.0);
    assert_eq!(a.is_point(), false);
}

#[test]
fn adding_two_tuples() {
    let a = Tuple::to_point(3.0, -2.0, 5.0);
    let b = Tuple::to_vector(-2.0, 3.0, 1.0);

    assert_eq!(a + b, Tuple::to_point(1.0, 1.0, 6.0));
}

#[test]
fn adding_two_vectors() {
    let a = Tuple::to_vector(3.0, -2.0, 5.0);
    let b = Tuple::to_vector(-2.0, 3.0, 1.0);

    assert_eq!(a + b, Tuple::to_vector(1.0, 1.0, 6.0));
}

#[test]
fn subtracting_two_points() {
    let a = Tuple::to_point(3.0, 2.0, 1.0);
    let b = Tuple::to_point(5.0, 6.0, 7.0);

    assert_eq!(a - b, Tuple::to_vector(-2.0, -4.0, -6.0));
}

#[test]
fn subtracting_vector_and_point() {
    let a = Tuple::to_vector(3.0, 2.0, 1.0);
    let b = Tuple::to_point(5.0, 6.0, 7.0);

    assert_eq!(a - b, Tuple::to_point(-2.0, -4.0, -6.0));
}

#[test]
fn subtracting_two_vectors() {
    let a = Tuple::to_vector(3.0, 2.0, 1.0);
    let b = Tuple::to_vector(5.0, 6.0, 7.0);

    assert_eq!(a - b, Tuple::to_vector(-2.0, -4.0, -6.0));
}

#[test]
fn subtracting_a_vector_from_the_zero_vector() {
    let zero = Tuple::to_vector(0.0, 0.0, 0.0);
    let v = Tuple::to_vector(1.0, -2.0, 3.0);

    assert_eq!(zero - v, Tuple::to_vector(-1.0, 2.0, -3.0));
}

#[test]
fn negating_a_tuple() {
    let mut a = Tuple::from(1.0, -2.0, 3.0, -4.0);
    a.negate();

    assert_eq!(a.x, -1.0);
    assert_eq!(a.y, 2.0);
    assert_eq!(a.z, -3.0);
    assert_eq!(a.w, 4.0);
}

#[test]
fn multiplying_a_tuple_by_a_scalar() {
    let a = Tuple::from(1.0, -2.0, 3.0, -4.0);
    let scalar = 3.5;

    assert_eq!(a * scalar, Tuple::from(3.5, -7.0, 10.5, -14.0));
}

#[test]
fn multiplying_a_tuple_by_a_fraction() {
    let a = Tuple::from(1.0, -2.0, 3.0, -4.0);
    let scalar = 0.5;

    assert_eq!(a * scalar, Tuple::from(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn dividing_a_tuple_by_scalar() {
    let a = Tuple::from(1.0, -2.0, 3.0, -4.0);

    assert_eq!(a / 2, Tuple::from(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn compute_the_magnitude_vec() {
    let a = Tuple::to_vector(1.0, 0.0, 0.0);

    assert_eq!(a.magnitude(), 1.0);
}

#[test]
fn compute_the_magnitude_vec2() {
    let a = Tuple::to_vector(0.0, 1.0, 0.0);

    assert_eq!(a.magnitude(), 1.0);
}

#[test]
fn compute_the_magnitude_vec3() {
    let a = Tuple::to_vector(0.0, 0.0, 1.0);

    assert_eq!(a.magnitude(), 1.0);
}

#[test]
fn compute_the_magnitude_vec4() {
    let a = Tuple::to_vector(1.0, 2.0, 3.0);

    assert_eq!(a.magnitude(), 14.0_f64.sqrt());
}

#[test]
fn compute_the_magnitude_vec5() {
    let a = Tuple::to_vector(-1.0, -2.0, -3.0);

    assert_eq!(a.magnitude(), 14.0_f64.sqrt());
}

#[test]
fn normalize_vector() {
    let a = Tuple::to_vector(4.0, 0.0, 0.0);
    let normalized_a = a.normalize();

    assert_eq!(normalized_a, Tuple::to_vector(1.0, 0.0, 0.0));
}

#[test]
fn normalize_vector2() {
    let a = Tuple::to_vector(1.0, 2.0, 3.0);
    let normalized_a = a.normalize();
    let approx = Tuple::to_vector(0.2676, 0.53452, 0.80178);

    assert!(equal(normalized_a.x, approx.x));
    assert!(equal(normalized_a.y, approx.y));
    assert!(equal(normalized_a.z, approx.z));
}

#[test]
fn magnitude_of_normalized_vector() {
    let v = Tuple::to_vector(1.0, 2.0, 3.0);
    let norm = v.normalize();

    assert_eq!(norm.magnitude(), 1.0);
}

#[test]
fn dot_product_two_tuples() {
    let a = Tuple::to_vector(1.0, 2.0, 3.0);
    let b = Tuple::to_vector(2.0, 3.0, 4.0);

    assert_eq!(a.dot(&b), 20.0);
}

#[test]
fn cross_product_two_vectors() {
    let a = Tuple::to_vector(1.0, 2.0, 3.0);
    let b = Tuple::to_vector(2.0, 3.0, 4.0);

    assert_eq!(a.cross(&b), Tuple::to_vector(-1.0, 2.0, -1.0));
    assert_eq!(b.cross(&a), Tuple::to_vector(1.0, -2.0, 1.0));
}
