use std::ops::Mul;

use crate::tuple;

macro_rules! matrix {
    ($mat:literal) => {{
        let mut _matr = Matrix::new();

        for (row_index, line) in $mat.lines().enumerate() {
            for (col_index, c) in line.trim().split(' ').enumerate() {
                println!("at ({},{}): {}", row_index, col_index, c);
                _matr.0[row_index][col_index] = c.parse::<f64>().unwrap();
            }
        }

        (_matr)
    }};
}

#[derive(PartialEq, Debug, Clone)]
struct Matrix(Vec<Vec<f64>>);

impl Matrix {
    fn new() -> Self {
        Self(vec![vec![0.0; 4]; 4])
    }
}

impl Mul<Self> for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut c = Matrix::new();

        for (row_index, row) in c.0.iter_mut().enumerate() {
            for (col_index, ele) in row.iter_mut().enumerate() {
                *ele = self.0[row_index][0] * rhs.0[0][col_index]
                    + self.0[row_index][1] * rhs.0[1][col_index]
                    + self.0[row_index][2] * rhs.0[2][col_index]
                    + self.0[row_index][3] * rhs.0[3][col_index];
            }
        }

        c
    }
}

impl Mul<tuple::Tuple> for Matrix {
    type Output = tuple::Tuple;

    fn mul(self, rhs: tuple::Tuple) -> Self::Output {
        let mut t = tuple::Tuple::new();

        for i in 0..3 {
            let val = self.0[i][0] * rhs.x
                + self.0[i][1] * rhs.y
                + self.0[i][2] * rhs.z
                + self.0[i][3] * rhs.w;

            match i {
                0 => t.x = val,
                1 => t.y = val,
                2 => t.z = val,
                _ => eprintln!("Can't match with i"),
            }
        }

        t.w = rhs.w;

        t
    }
}

#[test]
fn multiplying_matrix_by_identity_matrix() {
    let a = matrix!(
        "0 1 2 4
1 2 4 8
2 4 8 10
4 8 16 32"
    );

    let i = matrix!(
        "1 0 0 0
0 1 0 0
0 0 1 0
0 0 0 1"
    );

    let origin_a = a.clone();

    assert_eq!(a * i, origin_a);
}

#[test]
fn multiplying_identity_matrix_by_tuple() {
    let i = matrix!(
        "1 0 0 0
0 1 0 0
0 0 1 0
0 0 0 1"
    );

    let t = tuple::Tuple::from(1.0, 2.0, 3.0, 4.0);

    assert_eq!(i * t, t);
}

#[test]
fn multiplying_two_matrices() {
    let a = matrix!(
        "1 2 3 4
5 6 7 8
9 8 7 6
5 4 3 2
"
    );

    let b = matrix!(
        "-2 1 2 3
3 2 1 -1
4 3 6 5
1 2 7 8"
    );

    let result = a * b;

    assert_eq!(
        result,
        matrix!(
            "20 22 50 48
44 54 114 108
40 58 110 102
16 26 46 42"
        )
    );
}

#[test]
fn multiply_matrix_by_tuple() {
    let a = matrix!(
        "1 2 3 4
2 4 4 2
8 6 4 1
0 0 0 1
"
    );

    let b = tuple::Tuple::from(1.0, 2.0, 3.0, 1.0);

    assert_eq!(a * b, tuple::Tuple::from(18.0, 24.0, 33.0, 1.0));
}

#[test]
fn constructing_inspecting_a_4x4_matrix() {
    let mat = matrix!(
        "1 2 3 4
    5.5 6.5 7.5 8.5
    9 10 11 12
    13.5 14.5 15.5 16.5"
    );

    assert_eq!(mat.0[0][0], 1.0);
    assert_eq!(mat.0[0][1], 2.0);
    assert_eq!(mat.0[0][2], 3.0);
    assert_eq!(mat.0[0][3], 4.0);
    assert_eq!(mat.0[1][0], 5.5);
    assert_eq!(mat.0[1][1], 6.5);
    assert_eq!(mat.0[1][2], 7.5);
    assert_eq!(mat.0[1][3], 8.5);
    assert_eq!(mat.0[2][0], 9.0);
    assert_eq!(mat.0[2][1], 10.0);
    assert_eq!(mat.0[2][2], 11.0);
    assert_eq!(mat.0[2][3], 12.0);
    assert_eq!(mat.0[3][0], 13.5);
    assert_eq!(mat.0[3][1], 14.5);
    assert_eq!(mat.0[3][2], 15.5);
    assert_eq!(mat.0[3][3], 16.5);
}

#[test]
fn constructing_2x2_matrix() {
    let mat = matrix!(
        "-3 5
        1 -2"
    );

    assert_eq!(mat.0[0][0], -3.0);
    assert_eq!(mat.0[0][1], 5.0);
    assert_eq!(mat.0[1][0], 1.0);
    assert_eq!(mat.0[1][1], -2.0);
}

#[test]
fn constructing_3x3_matrix() {
    let mat = matrix!(
        "-3 5 0
        1 -2 -7
        0 1.0 1.0"
    );

    assert_eq!(mat.0[0][0], -3.0);
    assert_eq!(mat.0[0][1], 5.0);
    assert_eq!(mat.0[0][2], 0.0);
    assert_eq!(mat.0[1][0], 1.0);
    assert_eq!(mat.0[1][1], -2.0);
    assert_eq!(mat.0[1][2], -7.0);
    assert_eq!(mat.0[2][0], 0.0);
    assert_eq!(mat.0[2][1], 1.0);
    assert_eq!(mat.0[2][2], 1.0);
}

#[test]
fn matrix_equality_identical() {
    let a = matrix!(
        "1 2 3 4
    5 6 7 8
    9 8 7 6
    5 4 3 2"
    );
    let b = matrix!(
        "1 2 3 4
    5 6 7 8
    9 8 7 6
    5 4 3 2"
    );

    assert!(a == b);
}

#[test]
fn matrix_equality_different() {
    let a = matrix!(
        "1 2 3 4
    5 6 7 8
    9 8 7 6
    5 4 3 2"
    );
    let b = matrix!(
        "2 3 4 5
    6 7 8 9
    8 7 6 5
    4 3 2 1"
    );

    assert!(a != b);
}
