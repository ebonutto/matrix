use std::fmt;
use std::ops::{AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

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

// From
impl<K, const N: usize> From<[K; N]> for Vector<K> {
    fn from(data: [K; N]) -> Self {
        Self {
            data: data.into_iter().collect(),
        }
    }
}

impl<K> From<Vec<K>> for Vector<K> {
    fn from(data: Vec<K>) -> Self {
        Self { data }
    }
}

// Index //! Index out of bounds
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

// Display
impl<K: fmt::Display> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in &self.data {
            writeln!(f, "[{}]", x)?;
        }

        Ok(())
    }
}

// Arithmetic
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

// Linear combination
pub fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
    K: Copy + Default + AddAssign + Mul<Output = K>,
{
    assert_eq!(
        u.len(),
        coefs.len(),
        "linear_combination: {} vectors but {} coefficients",
        u.len(),
        coefs.len()
    );
    assert!(!u.is_empty(), "linear_combination: empty input");

    let n = u[0].size();
    let mut result = vec![K::default(); n];

    for (vector, &coef) in u.iter().zip(coefs.iter()) {
        assert_eq!(
            vector.size(),
            n,
            "linear_combination: inconsistent vector sizes"
        );

        for (r, &x) in result.iter_mut().zip(vector.data.iter()) {
            *r += x * coef;
        }
    }

    Vector::from(result)
}

// Dot
impl<K> Vector<K>
where
    K: Copy + Default + AddAssign + Mul<Output = K>,
{
    pub fn dot(&self, v: &Vector<K>) -> K {
        assert_eq!(
            self.size(),
            v.size(),
            "Vector::dot: size mismatch ({} vs {})",
            self.size(),
            v.size()
        );

        let mut sum = K::default();

        for (&a, &b) in self.data.iter().zip(v.data.iter()) {
            sum += a * b;
        }

        sum
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
    K: Copy + Default + AddAssign + Mul<Output = K> + Into<f32>,
{
    assert_eq!(
        u.size(),
        v.size(),
        "angle_cos: size mismatch ({} vs {})",
        u.size(),
        v.size()
    );
    assert!(u.size() > 0, "angle_cos: vectors must be non-empty");

    let norm_u = u.norm();
    let norm_v = v.norm();
    assert!(
        norm_u != 0.0 && norm_v != 0.0,
        "angle_cos: undefined for 0 vectors"
    );

    u.dot(v).into() / (norm_u * norm_v)
}

// Cross product
pub fn cross_product<K>(u: &Vector<K>, v: &Vector<K>) -> Vector<K>
where
    K: Copy + Sub<Output = K> + Mul<Output = K>,
{
    assert_eq!(
        u.size(),
        3,
        "cross_product: u is not 3-dimensional (size {})",
        u.size()
    );
    assert_eq!(
        v.size(),
        3,
        "cross_product: v is not 3-dimensional (size {})",
        v.size()
    );

    let s1 = u[1] * v[2] - u[2] * v[1];
    let s2 = u[2] * v[0] - u[0] * v[2];
    let s3 = u[0] * v[1] - u[1] * v[0];

    Vector::from([s1, s2, s3])
}

#[cfg(test)]
mod tests {

}
