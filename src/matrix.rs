use std::{
    ops::{AddAssign, Mul},
    process::Output,
};

#[derive(Debug, Clone, PartialEq)]
struct Matrix<T> {
    row: usize,
    col: usize,
    data: Vec<T>,
}

impl<T> Matrix<T>
where
    T: std::default::Default,
{
    fn new(row: usize, col: usize, mut data: Vec<T>) -> Self {
        data.resize_with(row * col, Default::default);
        Self { row, col, data }
    }

    // row: 0
    // col: 3
    // n: 4 (col)
    // location: (row * width) + col
    // TODO: Should this return a reference or value?
    fn at(&self, row: usize, col: usize) -> Option<&T> {
        if row > self.row || col > self.col {
            return None;
        }

        self.data.get((row * self.col) + col)
    }

    fn set(&mut self, row: usize, col: usize, val: T) {
        if row > self.row || col > self.col {
            panic!("Given row or col is incorrect.");
        }

        let location = (row * self.col) + col;
        self.data[location] = val;
    }
}

impl<T> Mul for Matrix<T>
where
    T: std::ops::Mul<T, Output = T> + std::ops::AddAssign + Copy + Default,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // if self.row != rhs.row && self.col != rhs.col {
        //     return None;
        // }

        let mut res = Matrix::new(self.row, self.col, vec![]);

        for r in 0..self.row {
            for c in 0..self.col {
                let mut sum_intersection: T = T::default();
                for i in 0..self.row {
                    let intersection_one = self.at(r, i).unwrap().clone(); // * rhs.at(i, c).unwrap();
                    let intersection_two = rhs.at(i, c).unwrap().clone();
                    let mul = intersection_one * intersection_two;
                    sum_intersection += mul;
                }

                res.set(r, c, sum_intersection);
            }
        }

        res
    }
}

// https://athemathmo.github.io/rulinalg/doc/src/rulinalg/macros/matrix.rs.html#45-66
macro_rules! matrix {
    () => {
        {
            // Handle the case when called with no arguments, i.e. matrix![]
            use $crate::matrix::Matrix;
            Matrix::new(0, 0, vec![])
        }
    };
    ($( $( $x: expr ),*);*) => {
        {
            use $crate::matrix::Matrix;
            let data_as_nested_array = [ $( [ $($x),* ] ),* ];
            let rows = data_as_nested_array.len();
            let cols = data_as_nested_array[0].len();
            let data_as_flat_array = data_as_nested_array.into_iter().flatten().collect();
            Matrix::new(rows, cols, data_as_flat_array)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn multiple_two_matrices() {
        let a = matrix!(1, 2, 3, 4; 5, 6, 7, 8; 9, 8, 7, 6; 5, 4, 3, 2);
        let b = matrix![-2, 1, 2, 3; 3, 2, 1, -1; 4, 3, 6, 5; 1, 2, 7, 8];

        let result = matrix![20, 22, 50, 48; 44, 54, 114, 108; 40, 58, 110, 102; 16, 26, 46, 42];

        assert_eq!(a * b, result);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = matrix![1, 2, 3, 4; 5, 6, 7, 8; 9, 8, 7, 6; 5, 4, 3, 2];
        let b = matrix![2, 3, 4, 5; 6, 7, 8, 9; 8, 7, 6, 5; 4, 3, 2, 1];

        assert_ne!(a, b);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let a = matrix![1, 2, 3, 4; 5, 6, 7, 8; 9, 8, 7, 6; 5, 4, 3, 2];
        let b = matrix![1, 2, 3, 4; 5, 6, 7, 8; 9, 8, 7, 6; 5, 4, 3, 2];

        assert_eq!(a, b);
    }

    #[test]
    fn construct_and_inspect() {
        let m = matrix![1.0, 2.0, 3.0, 4.0; 5.5, 6.5, 7.5, 8.5; 9.0, 10.0, 11.0, 12.0; 13.5, 14.5, 15.5, 16.5];

        assert_eq!(m.at(0, 0), Some(&1.0));
        assert_eq!(m.at(0, 3), Some(&4.0));
        assert_eq!(m.at(1, 0), Some(&5.5));
        assert_eq!(m.at(1, 2), Some(&7.5));
        assert_eq!(m.at(2, 2), Some(&11.0));
        assert_eq!(m.at(3, 0), Some(&13.5));
        assert_eq!(m.at(3, 2), Some(&15.5));
    }

    #[test]
    fn create_three_by_five_matrix() {
        let actual = matrix![9, 1, 2, 0, 3; 0, 0, 2, 3, 1; 8, 7, 5, 4, 6];
        let expected = Matrix::new(3, 5, vec![9, 1, 2, 0, 3, 0, 0, 2, 3, 1, 8, 7, 5, 4, 6]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn create_two_by_two_matrix() {
        let actual: Matrix<f64> = matrix![3.0, 1.0; 2.0, 7.0];
        let expected: Matrix<f64> = Matrix::new(2, 2, vec![3.0, 1.0, 2.0, 7.0]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn create_empty_matrix() {
        let actual: Matrix<usize> = matrix![];
        let expected: Matrix<usize> = Matrix::new(0, 0, vec![]);

        assert_eq!(actual.row, expected.row);
        assert_eq!(actual.col, expected.col);
        assert_eq!(actual.data, expected.data);
    }
}
