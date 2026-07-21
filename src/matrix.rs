use std::fmt;
use std::ops::{Index, IndexMut};

pub struct Matrix<K> {
    rows: usize,
    cols: usize,
    data: Vec<K>,
}

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

//TODO Improve display (see subject)
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
        debug_assert!(row < self.rows && col < self.cols, "index out of bounds");
        &self.data[row * self.cols + col]
    }
}

impl<K> IndexMut<(usize, usize)> for Matrix<K> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        debug_assert!(row < self.rows && col < self.cols, "index out of bounds");
        &mut self.data[row * self.cols + col]
    }
}

impl<K> Matrix<K>
where
    K: Copy + std::ops::AddAssign,
{
    pub fn add(&mut self, v: &Matrix<K>) {
        assert_eq!(self.shape(), v.shape(), "FUCK",);

        for row in 0..self.rows {
            for col in 0..self.cols {
                self[(row, col)] += v[(row, col)];
            }
        }
    }
}

impl<K> Matrix<K>
where
    K: Copy + std::ops::SubAssign,
{
    pub fn sub(&mut self, v: &Matrix<K>) {
        assert_eq!(self.shape(), v.shape(), "FUCK",);

        for row in 0..self.rows {
            for col in 0..self.cols {
                self[(row, col)] -= v[(row, col)];
            }
        }
    }
}

impl<K> Matrix<K>
where
    K: Copy + std::ops::MulAssign,
{
    pub fn scl(&mut self, a: K) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self[(row, col)] *= a;
            }
        }
    }
}
