type Matrix = Vec<Vec<f64>>;

trait NewTrait {
    fn new() -> Self;
}

impl NewTrait for Matrix {
    fn new() -> Self {
        vec![vec![0.0; 4]; 4]
    }
}

macro_rules! matrix {
    ($mat:literal) => {{
        let mut _matr = Matrix::new();

        for (row_index, line) in $mat.lines().enumerate() {
            for (col_index, c) in line.trim().split(' ').enumerate() {
                _matr[row_index][col_index] = c.parse::<f64>().unwrap();
            }
        }

        (_matr)
    }};
}

#[test]
fn constructing_inspecting_a_4x4_matrix() {
    let mat = matrix!(
        "1 2 3 4
    5.5 6.5 7.5 8.5
    9 10 11 12
    13.5 14.5 15.5 16.5"
    );

    assert_eq!(mat[0][0], 1.0);
    assert_eq!(mat[0][1], 2.0);
    assert_eq!(mat[0][2], 3.0);
    assert_eq!(mat[0][3], 4.0);
    assert_eq!(mat[1][0], 5.5);
    assert_eq!(mat[1][1], 6.5);
    assert_eq!(mat[1][2], 7.5);
    assert_eq!(mat[1][3], 8.5);
    assert_eq!(mat[2][0], 9.0);
    assert_eq!(mat[2][1], 10.0);
    assert_eq!(mat[2][2], 11.0);
    assert_eq!(mat[2][3], 12.0);
    assert_eq!(mat[3][0], 13.5);
    assert_eq!(mat[3][1], 14.5);
    assert_eq!(mat[3][2], 15.5);
    assert_eq!(mat[3][3], 16.5);
}

#[test]
fn constructing_2x2_matrix() {
    let mat = matrix!(
        "-3 5
        1 -2"
    );

    assert_eq!(mat[0][0], -3.0);
    assert_eq!(mat[0][1], 5.0);
    assert_eq!(mat[1][0], 1.0);
    assert_eq!(mat[1][1], -2.0);
}

#[test]
fn constructing_3x3_matrix() {
    let mat = matrix!(
        "-3 5 0
        1 -2 -7
        0 1.0 1.0"
    );

    assert_eq!(mat[0][0], -3.0);
    assert_eq!(mat[0][1], 5.0);
    assert_eq!(mat[0][2], 0.0);
    assert_eq!(mat[1][0], 1.0);
    assert_eq!(mat[1][1], -2.0);
    assert_eq!(mat[1][2], -7.0);
    assert_eq!(mat[2][0], 0.0);
    assert_eq!(mat[2][1], 1.0);
    assert_eq!(mat[2][2], 1.0);
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
