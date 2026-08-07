use crate::Vector;

use std::cmp::PartialEq;
use std::fmt;
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

// Structure
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<K> {
    data: Vec<K>,
    rows: usize,
    cols: usize,
}

// Constructors
impl<K> Matrix<K>
where
    K: Clone + Default,
{
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![K::default(); rows * cols],
            rows,
            cols,
        }
    }
}

impl<K> Matrix<K> {
    pub fn empty() -> Self {
        Self {
            data: Vec::new(),
            rows: 0,
            cols: 0,
        }
    }
}

// Accessors
impl<K> Matrix<K> {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }
}

// Predicates
impl<K> Matrix<K> {
    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    pub fn is_empty(&self) -> bool {
        self.rows == 0 && self.cols == 0
    }
}

// From
impl<K, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K> {
    fn from(data: [[K; C]; R]) -> Self {
        Self {
            rows: R,
            cols: C,
            data: data.into_iter().flatten().collect(),
        }
    }
}

// Display
impl<K: fmt::Display> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            write!(f, "[")?;

            for col in 0..self.cols {
                write!(f, "{}", self[(row, col)])?;

                if col + 1 < self.cols {
                    write!(f, ", ")?;
                }
            }

            writeln!(f, "]")?;
        }

        Ok(())
    }
}

// Index
impl<K> Index<(usize, usize)> for Matrix<K> {
    type Output = K;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[row * self.cols + col]
    }
}

impl<K> IndexMut<(usize, usize)> for Matrix<K> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[row * self.cols + col]
    }
}

// Addition
impl<K> Matrix<K>
where
    K: Copy + AddAssign,
{
    pub fn add(&mut self, v: &Matrix<K>) {
        assert_eq!(
            self.shape(),
            v.shape(),
            "Matrix::add: shape mismatch ({:?} vs {:?})",
            self.shape(),
            v.shape()
        );

        for (a, b) in self.data.iter_mut().zip(v.data.iter()) {
            *a += *b;
        }
    }
}

// Substraction
impl<K> Matrix<K>
where
    K: Copy + SubAssign,
{
    pub fn sub(&mut self, v: &Matrix<K>) {
        assert_eq!(
            self.shape(),
            v.shape(),
            "Matrix::sub: shape mismatch ({:?} vs {:?})",
            self.shape(),
            v.shape()
        );

        for (a, b) in self.data.iter_mut().zip(v.data.iter()) {
            *a -= *b;
        }
    }
}

// Scalar
impl<K> Matrix<K>
where
    K: Copy + MulAssign,
{
    pub fn scl(&mut self, a: K) {
        for x in self.data.iter_mut() {
            *x *= a;
        }
    }
}

// Multiplication
impl<K> Matrix<K>
where
    K: Copy + Default + AddAssign + Mul<Output = K>,
{
    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        //! Change error message
        assert_eq!(
            self.cols,
            vec.size(),
            "Matrix::mul_vec: shape mismatch ({} vs {})",
            self.cols(),
            vec.size()
        );

        let mut result = Vector::zeros(self.rows);

        for row in 0..self.rows {
            for col in 0..self.cols {
                result[row] += self[(row, col)] * vec[col];
            }
        }

        result
    }

    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        //! Change error message
        assert_eq!(
            self.cols,
            mat.rows,
            "Matrix::mul_mat: shape mismatch ({:?} vs {:?})",
            self.shape(),
            mat.shape()
        );

        let mut result = Matrix::zeros(self.rows, mat.cols);

        for row in 0..self.rows {
            for col in 0..mat.cols {
                for k in 0..self.cols {
                    result[(row, col)] += self[(row, k)] * mat[(k, col)];
                }
            }
        }

        result
    }
}

// Trace
impl<K> Matrix<K>
where
    K: Copy + Default + AddAssign,
{
    pub fn trace(&self) -> K {
        assert!(
            self.is_square(),
            "Matrix::trace: undefined for non-square matrix ({:?})",
            self.shape()
        );

        let mut sum = K::default();

        for row in 0..self.rows {
            sum += self[(row, row)];
        }

        sum
    }
}

// Transpose
impl<K> Matrix<K>
where
    K: Copy + Default,
{
    pub fn transpose(&self) -> Matrix<K> {
        let mut result = Matrix::zeros(self.cols, self.rows);

        for row in 0..self.rows {
            for col in 0..self.cols {
                result[(col, row)] = self[(row, col)];
            }
        }

        result
    }
}

// Reduced row echelon form
impl<K> Matrix<K>
where
    K: Copy + Default + PartialEq + Div<Output = K> + Mul<Output = K> + SubAssign,
{
    pub fn row_echelon(&self) -> Matrix<K> {
        let mut result = self.clone();
        let mut pivot_row: usize = 0;

        for col in 0..result.cols {
            if pivot_row >= result.rows {
                break;
            }

            let Some(found_row) = result.find_pivot(pivot_row, col) else {
                continue;
            };

            if found_row != pivot_row {
                result.swap_rows(found_row, pivot_row);
            }

            result.normalize_pivot_row(pivot_row, col);
            result.eliminate_column(pivot_row, col);

            pivot_row += 1;
        }

        result
    }

    fn swap_rows(&mut self, row1: usize, row2: usize) {
        for col in 0..self.cols {
            self.data
                .swap(row1 * self.cols + col, row2 * self.cols + col);
        }
    }

    fn find_pivot(&self, start_row: usize, col: usize) -> Option<usize> {
        (start_row..self.rows).find(|&row| self[(row, col)] != K::default())
    }

    fn normalize_pivot_row(&mut self, pivot_row: usize, start_col: usize) {
        let pivot = self[(pivot_row, start_col)];
        for col in start_col..self.cols {
            self[(pivot_row, col)] = self[(pivot_row, col)] / pivot;
        }
    }

    fn eliminate_column(&mut self, pivot_row: usize, start_col: usize) {
        for row in 0..self.rows {
            if row == pivot_row {
                continue;
            }

            let factor = self[(row, start_col)];
            if factor != K::default() {
                for col in start_col..self.cols {
                    let pivot = self[(pivot_row, col)];
                    self[(row, col)] -= factor * pivot;
                }
            }
        }
    }
}

// Determinant
impl<K> Matrix<K>
where
    K: Copy + Default + Sub<Output = K> + Mul<Output = K> + Add<Output = K>,
{
    pub fn determinant(&self) -> K {
        assert!(
            self.is_square(),
            "Matrix::determinant: undefined for non-square matrix ({:?})",
            self.shape()
        );

        match self.rows {
            0 => K::default(),
            1 => self[(0, 0)],
            2 => self.determinant_2x2(),
            3 => self.determinant_3x3(),
            // 4 => self.determinant_4x4(),
            _ => unreachable!(),
        }
    }

    fn determinant_2x2(&self) -> K {
        self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
    }

    fn determinant_3x3(&self) -> K {
        self[(0, 0)] * (self[(1, 1)] * self[(2, 2)] - self[(1, 2)] * self[(2, 1)])
            - self[(0, 1)] * (self[(1, 0)] * self[(2, 2)] - self[(1, 2)] * self[(2, 0)])
            + self[(0, 2)] * (self[(1, 0)] * self[(2, 1)] - self[(1, 1)] * self[(2, 0)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Addition f32
    #[test]
    fn test_addition_f32_basic() {
        let mut u: Matrix<f32> = Matrix::from([[1., 2.], [3., 4.]]);
        let v: Matrix<f32> = Matrix::from([[7., 4.], [-2., 2.]]);
        u.add(&v);
        assert_eq!(u, Matrix::from([[8., 6.], [1., 6.]]));
    }

    #[test]
    fn test_addition_f32_zero() {
        let mut u: Matrix<f32> = Matrix::from([[1., 2.], [3., 4.]]);
        let v: Matrix<f32> = Matrix::from([[0., 0.], [0., 0.]]);
        u.add(&v);
        assert_eq!(u, Matrix::from([[1., 2.], [3., 4.]]));

        let mut u: Matrix<f32> = Matrix::from([[0., 0.], [0., 0.]]);
        let v: Matrix<f32> = Matrix::from([[0., 0.], [0., 0.]]);
        u.add(&v);
        assert_eq!(u, Matrix::from([[0., 0.], [0., 0.]]));
    }

    #[test]
    fn test_addition_f32_empty() {
        let mut u: Matrix<f32> = Matrix::from([[]]);
        let v: Matrix<f32> = Matrix::from([[]]);
        u.add(&v);
        assert_eq!(u, Matrix::from([[]]));
    }

    #[test]
    #[should_panic(expected = "shape mismatch")]
    fn test_addition_f32_panic_shape_mismatch() {
        let mut u: Matrix<f32> = Matrix::from([[1., 2.]]);
        let v: Matrix<f32> = Matrix::from([[1.], [2.]]);
        u.add(&v);
    }

    // Substraction f32
    #[test]
    fn test_substraction_f32_basic() {
        let mut u: Matrix<f32> = Matrix::from([[1., 2.], [3., 4.]]);
        let v: Matrix<f32> = Matrix::from([[7., 4.], [-2., 2.]]);
        u.sub(&v);
        assert_eq!(u, Matrix::from([[-6., -2.], [5., 2.]]));
    }

    #[test]
    fn test_substraction_f32_zero() {
        let mut u: Matrix<f32> = Matrix::from([[1., 2.], [3., 4.]]);
        let v: Matrix<f32> = Matrix::from([[0., 0.], [0., 0.]]);
        u.sub(&v);
        assert_eq!(u, Matrix::from([[1., 2.], [3., 4.]]));

        let mut u: Matrix<f32> = Matrix::from([[0., 0.], [0., 0.]]);
        let v: Matrix<f32> = Matrix::from([[0., 0.], [0., 0.]]);
        u.sub(&v);
        assert_eq!(u, Matrix::from([[0., 0.], [0., 0.]]));
    }

    #[test]
    fn test_substraction_f32_empty() {
        let mut u: Matrix<f32> = Matrix::from([[]]);
        let v: Matrix<f32> = Matrix::from([[]]);
        u.sub(&v);
        assert_eq!(u, Matrix::from([[]]));
    }

    #[test]
    #[should_panic(expected = "shape mismatch")]
    fn test_substraction_f32_panic_shape_mismatch() {
        let mut u: Matrix<f32> = Matrix::from([[1., 2.]]);
        let v: Matrix<f32> = Matrix::from([[1.], [2.]]);
        u.sub(&v);
    }

    // Scalar f32
    #[test]
    fn test_scalar_f32_basic() {
        let mut u: Matrix<f32> = Matrix::from([[1., 2.], [3., 4.]]);
        u.scl(2.);
        assert_eq!(u, Matrix::from([[2., 4.], [6., 8.]]));
    }

    #[test]
    fn test_scalar_f32_zero() {
        let mut u: Matrix<f32> = Matrix::from([[1., 2.], [3., 4.]]);
        u.scl(0.);
        assert_eq!(u, Matrix::from([[0., 0.], [0., 0.]]));

        let mut u: Matrix<f32> = Matrix::from([[0., 0.], [0., 0.]]);
        u.scl(0.);
        assert_eq!(u, Matrix::from([[0., 0.], [0., 0.]]));
    }

    #[test]
    fn test_scalar_f32_empty() {
        let mut u: Matrix<f32> = Matrix::from([[]]);
        u.scl(1.);
        assert_eq!(u, Matrix::from([[]]));
    }

    // Multiplication vector f32
    #[test]
    fn test_multiplication_vector_f32_basic() {
        let u: Matrix<f32> = Matrix::from([[1., 0.], [0., 1.]]);
        let v: Vector<f32> = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v), Vector::from([4., 2.]));

        let u: Matrix<f32> = Matrix::from([[2., 0.], [0., 2.]]);
        let v: Vector<f32> = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v), Vector::from([8., 4.]));

        let u: Matrix<f32> = Matrix::from([[2., -2.], [-2., 2.]]);
        let v: Vector<f32> = Vector::from([4., 2.]);
        assert_eq!(u.mul_vec(&v), Vector::from([4., -4.]));
    }

    #[test]
    fn test_multiplication_vector_f32_zero() {
        let u: Matrix<f32> = Matrix::from([[1., 2.], [3., 14.]]);
        let v: Vector<f32> = Vector::from([0., 0.]);
        assert_eq!(u.mul_vec(&v), Vector::from([0., 0.]));

        let u: Matrix<f32> = Matrix::from([[0., 0.], [0., 0.]]);
        let v: Vector<f32> = Vector::from([0., 0.]);
        assert_eq!(u.mul_vec(&v), Vector::from([0., 0.]));
    }

    #[test]
    fn test_multiplication_vector_f32_empty() {
        let u: Matrix<f32> = Matrix::from([] as [[f32; 0]; 0]);
        let v: Vector<f32> = Vector::from([]);
        assert_eq!(u.mul_vec(&v), Vector::from([]));
    }

    // Multiplication matrix f32
    #[test]
    fn test_multiplication_matrix_f32_basic() {
        let u: Matrix<f32> = Matrix::from([[1., 0.], [0., 1.]]);
        let v: Matrix<f32> = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(u.mul_mat(&v), Matrix::from([[1., 0.], [0., 1.]]));

        let u: Matrix<f32> = Matrix::from([[1., 0.], [0., 1.]]);
        let v: Matrix<f32> = Matrix::from([[2., 1.], [4., 2.]]);
        assert_eq!(u.mul_mat(&v), Matrix::from([[2., 1.], [4., 2.]]));

        let u: Matrix<f32> = Matrix::from([[3., -5.], [6., 8.]]);
        let v: Matrix<f32> = Matrix::from([[2., 1.], [4., 2.]]);
        assert_eq!(u.mul_mat(&v), Matrix::from([[-14., -7.], [44., 22.]]));
    }

    #[test]
    fn test_multiplication_matrix_f32_zero() {
        let u: Matrix<f32> = Matrix::from([[1., 2.], [3., 14.]]);
        let v: Matrix<f32> = Matrix::from([[0., 0.], [0., 0.]]);
        assert_eq!(u.mul_mat(&v), Matrix::from([[0., 0.], [0., 0.]]));

        let u: Matrix<f32> = Matrix::from([[0., 0.], [0., 0.]]);
        let v: Matrix<f32> = Matrix::from([[0., 0.], [0., 0.]]);
        assert_eq!(u.mul_mat(&v), Matrix::from([[0., 0.], [0., 0.]]));
    }

    #[test]
    fn test_multiplication_matrix_f32_empty() {
        let u: Matrix<f32> = Matrix::from([] as [[f32; 0]; 0]);
        let v: Matrix<f32> = Matrix::from([] as [[f32; 0]; 0]);
        assert_eq!(u.mul_mat(&v), Matrix::from([] as [[f32; 0]; 0]));
    }

    // Trace f32
    #[test]
    fn test_trace_f32_basic() {
        let u: Matrix<f32> = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(u.trace(), 2.);

        let u: Matrix<f32> = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
        assert_eq!(u.trace(), 9.);

        let u: Matrix<f32> = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
        assert_eq!(u.trace(), -21.);

        let u: Matrix<f32> = Matrix::from([[12.]]);
        assert_eq!(u.trace(), 12.);
    }

    #[test]
    fn test_trace_f32_zero() {
        let u: Matrix<f32> = Matrix::from([[0., 1., 1.], [1., 0., 1.], [1., 1., 0.]]);
        assert_eq!(u.trace(), 0.);

        let u: Matrix<f32> = Matrix::from([[0., 0.], [0., 0.]]);
        assert_eq!(u.trace(), 0.);
    }

    #[test]
    fn test_trace_f32_empty() {
        let u: Matrix<f32> = Matrix::from([] as [[f32; 0]; 0]);
        assert_eq!(u.trace(), 0.);
    }

    #[test]
    #[should_panic(expected = "undefined for non-square matrix")]
    fn test_trace_f32_panic_non_squared() {
        let u: Matrix<f32> = Matrix::from([[1., 2.]]);
        assert_eq!(u.trace(), 0.);
    }

    // Transpose f32
    #[test]
    fn test_transpose_f32_basic() {
        let u: Matrix<f32> = Matrix::from([[1., 0.], [0., 1.]]);
        assert_eq!(u.transpose(), Matrix::from([[1., 0.], [0., 1.]]));

        let u = Matrix::from([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]);
        assert_eq!(
            u.transpose(),
            Matrix::from([[2., 4., -2.], [-5., 3., 3.], [0., 7., 4.]])
        );

        let u = Matrix::from([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]);
        assert_eq!(
            u.transpose(),
            Matrix::from([[-2., 1., 0.], [-8., -23., 6.], [4., 4., 4.]])
        );
    }

    #[test]
    fn test_transpose_f32_zero() {
        let u: Matrix<f32> = Matrix::from([[0., 0.], [0., 0.]]);
        assert_eq!(u.transpose(), Matrix::from([[0., 0.], [0., 0.]]));
    }

    #[test]
    fn test_transpose_f32_empty() {
        let u: Matrix<f32> = Matrix::from([] as [[f32; 0]; 0]);
        assert_eq!(u.transpose(), Matrix::from([] as [[f32; 0]; 0]));
    }

    #[test]
    fn test_reduced_row_echelon_form_f32_basic() {
        let u: Matrix<f32> = Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]);
        assert_eq!(
            u.row_echelon(),
            Matrix::from([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
        );

        let u = Matrix::from([[1., 2.], [3., 4.]]);
        assert_eq!(u.row_echelon(), Matrix::from([[1., 0.], [0., 1.]]));

        let u = Matrix::from([[1., 2.], [2., 4.]]);
        assert_eq!(u.row_echelon(), Matrix::from([[1., 2.], [0., 0.]]));

        let u = Matrix::from([
            [8., 5., -2., 4., 28.],
            [4., 2.5, 20., 4., -4.],
            [8., 5., 1., 4., 17.],
        ]);
        println!("{}", u.row_echelon());
        // [1.0, 0.625, 0.0, 0.0, -12.1666667]
        // [0.0, 0.0, 1.0, 0.0, -3.6666667]
        // [0.0, 0.0, 0.0, 1.0, 29.5 ]
    }
}
