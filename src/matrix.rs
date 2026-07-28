use std::fmt;
use std::ops::{Index, IndexMut};

// Structure
#[derive(Debug, PartialEq)]
pub struct Matrix<K> {
    data: Vec<K>,
    rows: usize,
    cols: usize,
}

// Constructors
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

// // From 1D
// impl<K, const N: usize> From<[K; N]> for Matrix<K> {
//     fn from(data: [K; N]) -> Self {
//         Self {
//             rows: 1,
//             cols: N,
//             data: data.into_iter().collect(),
//         }
//     }
// }

// From 2D
impl<K, const R: usize, const C: usize> From<[[K; C]; R]> for Matrix<K> {
    fn from(data: [[K; C]; R]) -> Self {
        Self {
            rows: R,
            cols: C,
            data: data.into_iter().flatten().collect(),
        }
    }
}

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

impl<K> Index<(usize, usize)> for Matrix<K> {
    type Output = K;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        debug_assert!(row < self.rows && col < self.cols, "index out of bounds"); // FUCK
        &self.data[row * self.cols + col]
    }
}

impl<K> IndexMut<(usize, usize)> for Matrix<K> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        debug_assert!(row < self.rows && col < self.cols, "index out of bounds"); // FUCK
        &mut self.data[row * self.cols + col]
    }
}

impl<K> Matrix<K>
where
    K: Copy + std::ops::AddAssign,
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

impl<K> Matrix<K>
where
    K: Copy + std::ops::SubAssign,
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

impl<K> Matrix<K>
where
    K: Copy + std::ops::MulAssign,
{
    pub fn scl(&mut self, a: K) {
        for x in self.data.iter_mut() {
            *x *= a;
        }
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

    // Addition i32
    #[test]
    fn test_addition_i32_basic() {
        let mut u: Matrix<i32> = Matrix::from([[1, 2], [3, 4]]);
        let v: Matrix<i32> = Matrix::from([[7, 4], [-2, 2]]);
        u.add(&v);
        assert_eq!(u, Matrix::from([[8, 6], [1, 6]]));
    }

    #[test]
    fn test_addition_i32_zero() {
        let mut u: Matrix<i32> = Matrix::from([[0, 0], [0, 0]]);
        let v: Matrix<i32> = Matrix::from([[0, 0], [0, 0]]);
        u.add(&v);
        assert_eq!(u, Matrix::from([[0, 0], [0, 0]]));
    }

    #[test]
    fn test_addition_i32_empty() {
        let mut u: Matrix<i32> = Matrix::from([[]]);
        let v: Matrix<i32> = Matrix::from([[]]);
        u.add(&v);
        assert_eq!(u, Matrix::from([[]]));
    }

    #[test]
    #[should_panic(expected = "shape mismatch")]
    fn test_addition_i32_panic_shape_mismatch() {
        let mut u: Matrix<i32> = Matrix::from([[1, 2]]);
        let v: Matrix<i32> = Matrix::from([[1], [2]]);
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

    // Substraction i32
    #[test]
    fn test_substraction_i32_basic() {
        let mut u: Matrix<i32> = Matrix::from([[1, 2], [3, 4]]);
        let v: Matrix<i32> = Matrix::from([[7, 4], [-2, 2]]);
        u.sub(&v);
        assert_eq!(u, Matrix::from([[-6, -2], [5, 2]]));
    }

    #[test]
    fn test_substraction_i32_zero() {
        let mut u: Matrix<i32> = Matrix::from([[0, 0], [0, 0]]);
        let v: Matrix<i32> = Matrix::from([[0, 0], [0, 0]]);
        u.sub(&v);
        assert_eq!(u, Matrix::from([[0, 0], [0, 0]]));
    }

    #[test]
    fn test_substraction_i32_empty() {
        let mut u: Matrix<i32> = Matrix::from([[]]);
        let v: Matrix<i32> = Matrix::from([[]]);
        u.sub(&v);
        assert_eq!(u, Matrix::from([[]]));
    }

    #[test]
    #[should_panic(expected = "shape mismatch")]
    fn test_substraction_i32_panic_shape_mismatch() {
        let mut u: Matrix<i32> = Matrix::from([[1, 2]]);
        let v: Matrix<i32> = Matrix::from([[1], [2]]);
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

    // Scalar i32
    #[test]
    fn test_scalar_i32_basic() {
        let mut u: Matrix<i32> = Matrix::from([[1, 2], [3, 4]]);
        u.scl(2);
        assert_eq!(u, Matrix::from([[2, 4], [6, 8]]));
    }

    #[test]
    fn test_scalar_i32_zero() {
        let mut u: Matrix<i32> = Matrix::from([[1, 2], [3, 4]]);
        u.scl(0);
        assert_eq!(u, Matrix::from([[0, 0], [0, 0]]));

        let mut u: Matrix<i32> = Matrix::from([[0, 0], [0, 0]]);
        u.scl(0);
        assert_eq!(u, Matrix::from([[0, 0], [0, 0]]));
    }

    #[test]
    fn test_scalar_i32_empty() {
        let mut u: Matrix<i32> = Matrix::from([[]]);
        u.scl(1);
        assert_eq!(u, Matrix::from([[]]));
    }
}
