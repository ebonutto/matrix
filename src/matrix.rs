use crate::Vector;

use std::fmt;
use std::ops::{AddAssign, Index, IndexMut, Mul, MulAssign, SubAssign};

// Structure
#[derive(Debug, PartialEq)]
pub struct Matrix<K> {
    data: Vec<K>,
    rows: usize,
    cols: usize,
}

// Constructors
impl<K> Matrix<K>
where
    K: Copy + Default,
{
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![K::default(); rows * cols],
            rows,
            cols,
        }
    }
}

// Getters
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

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
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
                write!(f, "{}", self.data[row * self.cols + col])?;

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
        let i: usize = row * self.cols + col;

        debug_assert!(
            i < self.data.len(),
            "Matrix::index: index {} out of bounds (size {})",
            i,
            self.data.len()
        );

        &self.data[i]
    }
}

impl<K> IndexMut<(usize, usize)> for Matrix<K> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        let i: usize = row * self.cols + col;

        debug_assert!(
            i < self.data.len(),
            "Matrix::index_mut: index {} out of bounds (size {})",
            i,
            self.data.len()
        );

        &mut self.data[i]
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
        assert_eq!(
            self.cols,
            vec.size(),
            "Matrix::mul_vec: shape mismatch ({} vs {})",
            self.cols(),
            vec.size()
        );

        let mut result = Vector::zeros(self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                result[i] += self[(i, j)] * vec[j];
            }
        }

        result
    }

    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        assert_eq!(
            self.cols,
            mat.rows,
            "Matrix::mul_mat: shape mismatch ({:?} vs {:?})",
            self.shape(),
            mat.shape()
        );

        let mut result = Matrix::zeros(self.rows, mat.cols);

        for i in 0..self.rows {
            for j in 0..mat.cols {
                for k in 0..self.cols {
                    result[(i, j)] += self[(i, k)] * mat[(k, j)];
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

        for i in 0..self.rows {
            sum += self[(i, i)];
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
        let mut result = Matrix {
            data: vec![K::default(); self.cols * self.rows],
            rows: self.cols,
            cols: self.rows,
        };

        for i in 0..self.rows {
            for j in 0..self.cols {
                result[(j, i)] = self[(i, j)];
            }
        }

        result
    }
}

// impl<K> Matrix<K> {
//     fn row_echelon<K>(&self) -> Matrix<K> {
//         let mut result = vec![K::default(); self.rows];

//     }
// }

// impl<K> Matrix<K> {
//     pub fn determinant(&self) -> K {}
// }

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
}
