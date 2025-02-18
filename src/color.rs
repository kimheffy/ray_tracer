use std::ops::{Add, Mul, Sub};

use crate::utils::equal;

#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Mul<i32> for Color {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            red: self.red * rhs as f64,
            green: self.green * rhs as f64,
            blue: self.blue * rhs as f64,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

fn hadamard_product(c1: &Color, c2: &Color) -> Color {
    let red = c1.red * c2.red;
    let green = c1.green * c2.green;
    let blue = c1.blue * c2.blue;

    Color { red, green, blue }
}

#[test]
fn should_test_for_color() {
    let color = Color::new(-0.5, 0.4, 1.7);

    assert_eq!(color.red, -0.5);
    assert_eq!(color.green, 0.4);
    assert_eq!(color.blue, 1.7);
}

#[test]
fn adding_two_colors() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);

    assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
}

#[test]
fn subtract_two_colors() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);
    let diff = c1 - c2;

    assert!(equal(diff.red, 0.2));
    assert!(equal(diff.green, 0.5));
    assert!(equal(diff.blue, 0.5));
}

#[test]
fn multiply_by_scalar() {
    let c1 = Color::new(0.2, 0.3, 0.4);

    assert_eq!(c1 * 2, Color::new(0.4, 0.6, 0.8));
}

#[test]
fn multiply_colors() {
    let c1 = Color::new(1.0, 0.2, 0.4);
    let c2 = Color::new(0.9, 1.0, 0.1);

    let multiply = c1 * c2;

    assert!(equal(multiply.red, 0.9));
    assert!(equal(multiply.green, 0.2));
    assert!(equal(multiply.blue, 0.04));
}
