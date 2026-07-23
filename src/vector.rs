use std::fmt;
use std::ops::{AddAssign, Index, IndexMut, Mul, MulAssign, SubAssign};

// Structure
pub struct Vector<K> {
    data: Vec<K>,
}

// Constructors
impl<K> Vector<K> {
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

// Index
impl<K> Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl<K> IndexMut<usize> for Vector<K> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.data[i]
    }
}

// From
impl<K, const N: usize> From<[K; N]> for Vector<K> {
    fn from(data: [K; N]) -> Self {
        Self {
            data: data.into_iter().collect(),
        }
    }
}

// Display
impl<K: fmt::Display> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in &self.data {
            writeln!(f, "[{}]", x)?;
        }

        Ok(())
    }
}

// Operations
impl<K> Vector<K>
where
    K: Copy + AddAssign,
{
    pub fn add(&mut self, v: &Vector<K>) {
        assert_eq!(
            self.size(),
            v.size(),
            "Vector::add: size mismatch ({} vs {})",
            self.size(),
            v.size()
        );

        for (a, b) in self.data.iter_mut().zip(v.data.iter()) {
            *a += *b;
        }
    }
}

impl<K> Vector<K>
where
    K: Copy + SubAssign,
{
    pub fn sub(&mut self, v: &Vector<K>) {
        assert_eq!(
            self.size(),
            v.size(),
            "Vector::sub: size mismatch ({} vs {})",
            self.size(),
            v.size()
        );

        for (a, b) in self.data.iter_mut().zip(v.data.iter()) {
            *a -= *b;
        }
    }
}

impl<K> Vector<K>
where
    K: Copy + MulAssign,
{
    pub fn scl(&mut self, a: K) {
        for x in self.data.iter_mut() {
            *x *= a;
        }
    }
}

impl<K> Vector<K>
where
    K: Copy + std::iter::Sum + Mul<Output = K>,
{
    pub fn dot(&self, v: &Vector<K>) -> K {
        assert_eq!(
            self.size(),
            v.size(),
            "Vector::dot: size mismatch ({} vs {})",
            self.size(),
            v.size()
        );

        self.data
            .iter()
            .zip(v.data.iter())
            .map(|(&x, &y)| x * y)
            .sum()
    }
}

// Norms
impl<K> Vector<K>
where
    K: Copy + Into<f32>,
{
    pub fn norm_1(&self) -> f32 {
        self.data.iter().map(|&x| x.into().abs()).sum()
    }

    pub fn norm(&self) -> f32 {
        self.data
            .iter()
            .map(|&x| {
                let v: f32 = x.into();
                v * v
            })
            .sum::<f32>()
            .sqrt()
    }

    pub fn norm_inf(&self) -> f32 {
        self.data
            .iter()
            .map(|&x| x.into().abs())
            .fold(0.0f32, f32::max)
    }
}

// Cosine
pub fn angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> f32
where
    K: Copy + std::iter::Sum + Mul<Output = K> + Into<f32>,
{
    // Zero
    // Size
    u.dot(v).into() / (u.norm() * v.norm())
}

// pub fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
// where
//     K: Copy + Default + AddAssign + Mul<Output = K>,
// {
//     // assert_eq!(u.size(), v.size(), "FUCK");
//     assert!(!u.is_empty(), "linear_combination: empty input");

//     let n = u[0].size();
//     let mut result = vec![K::default(); n];

//     for (vector, &coef) in u.iter().zip(coefs.iter()) {
//         // assert_eq
//         for (r, &x) in result.iter_mut().zip(vector.data.iter()) {
//             *r += x * coef;
//         }
//     }

//     Vector { data: result }
// }
