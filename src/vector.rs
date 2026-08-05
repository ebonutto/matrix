use std::fmt;
use std::ops::{AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

// Structure
#[derive(Debug, PartialEq)]
pub struct Vector<K> {
    data: Vec<K>,
}

// Constructors
impl<K> Vector<K>
where
    K: Clone + Default,
{
    pub fn zeros(size: usize) -> Self {
        Self {
            data: vec![K::default(); size],
        }
    }
}

impl<K> Vector<K> {
    pub fn empty() -> Self {
        Self { data: Vec::new() }
    }
}

// Getters
impl<K> Vector<K> {
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
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

// Index
impl<K> Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, i: usize) -> &Self::Output {
        debug_assert!(
            i < self.data.len(),
            "Vector::index: index {} out of bounds (size {})",
            i,
            self.data.len()
        );

        &self.data[i]
    }
}

impl<K> IndexMut<usize> for Vector<K> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        debug_assert!(
            i < self.data.len(),
            "Vector::index_mut: index {} out of bounds (size {})",
            i,
            self.data.len()
        );

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

// Addition
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

// Substraction
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

// Scalar
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

    if u.is_empty() {
        return Vector::from(Vec::new());
    }

    let n = u[0].size();
    let mut result = vec![K::default(); n];

    for (i, (vector, &coef)) in u.iter().zip(coefs.iter()).enumerate() {
        assert_eq!(
            vector.size(),
            n,
            "linear_combination: vector at index {} has size {} but expected {} (based on vector 0)",
            i,
            vector.size(),
            n
        );

        for (r, &x) in result.iter_mut().zip(vector.data.iter()) {
            *r += x * coef;
        }
    }

    Vector::from(result)
}

// Dot product
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
    assert!(u.size() > 0, "angle_cos: undefined for empty vectors");

    let norm_u = u.norm();
    let norm_v = v.norm();
    assert!(
        norm_u != 0.0 && norm_v != 0.0,
        "angle_cos: undefined for zero vectors"
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
        v.size(),
        "cross_product: size mismatch ({} vs {})",
        u.size(),
        v.size()
    );
    assert_eq!(
        u.size(),
        3,
        "cross_product: undefined for non-three-dimensional vectors ({})",
        u.size()
    );

    let s1 = u[1] * v[2] - u[2] * v[1];
    let s2 = u[2] * v[0] - u[0] * v[2];
    let s3 = u[0] * v[1] - u[1] * v[0];

    Vector::from([s1, s2, s3])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-5
    }

    // Addition f32
    #[test]
    fn test_addition_f32_basic() {
        let mut u: Vector<f32> = Vector::from([2., 3.]);
        let v: Vector<f32> = Vector::from([5., 7.]);
        u.add(&v);
        assert_eq!(u, Vector::from([7., 10.]));

        let mut u: Vector<f32> = Vector::from([-3., 5., -1.]);
        let v: Vector<f32> = Vector::from([1., 0., 1.]);
        u.add(&v);
        assert_eq!(u, Vector::from([-2., 5., 0.]));

        let mut u: Vector<f32> = Vector::from([1.]);
        let v: Vector<f32> = Vector::from([3.]);
        u.add(&v);
        assert_eq!(u, Vector::from([4.]));
    }

    #[test]
    fn test_addition_f32_zero() {
        let mut u: Vector<f32> = Vector::from([1., 2.]);
        let v: Vector<f32> = Vector::from([0., 0.]);
        u.add(&v);
        assert_eq!(u, Vector::from([1., 2.]));

        let mut u: Vector<f32> = Vector::from([0., 0.]);
        let v: Vector<f32> = Vector::from([0., 0.]);
        u.add(&v);
        assert_eq!(u, Vector::from([0., 0.]));
    }

    #[test]
    fn test_addition_f32_empty() {
        let mut u: Vector<f32> = Vector::from([]);
        let v: Vector<f32> = Vector::from([]);
        u.add(&v);
        assert_eq!(u, Vector::from([]));
    }

    #[test]
    #[should_panic(expected = "size mismatch")]
    fn test_addition_f32_panic_size_mismatch() {
        let mut u: Vector<f32> = Vector::from([1., 2.]);
        let v: Vector<f32> = Vector::from([1., 2., 3.]);
        u.add(&v);
    }

    // Substraction f32
    #[test]
    fn test_substraction_f32_basic() {
        let mut u: Vector<f32> = Vector::from([2., 3.]);
        let v: Vector<f32> = Vector::from([5., 7.]);
        u.sub(&v);
        assert_eq!(u, Vector::from([-3., -4.]));

        let mut u: Vector<f32> = Vector::from([-3., 5., -1.]);
        let v: Vector<f32> = Vector::from([1., 0., 1.]);
        u.sub(&v);
        assert_eq!(u, Vector::from([-4., 5., -2.]));

        let mut u: Vector<f32> = Vector::from([1.]);
        let v: Vector<f32> = Vector::from([3.]);
        u.sub(&v);
        assert_eq!(u, Vector::from([-2.]));
    }

    #[test]
    fn test_substraction_f32_zero() {
        let mut u: Vector<f32> = Vector::from([1., 2.]);
        let v: Vector<f32> = Vector::from([0., 0.]);
        u.sub(&v);
        assert_eq!(u, Vector::from([1., 2.]));

        let mut u: Vector<f32> = Vector::from([0., 0.]);
        let v: Vector<f32> = Vector::from([0., 0.]);
        u.sub(&v);
        assert_eq!(u, Vector::from([0., 0.]));
    }

    #[test]
    fn test_substraction_f32_empty() {
        let mut u: Vector<f32> = Vector::from([]);
        let v: Vector<f32> = Vector::from([]);
        u.sub(&v);
        assert_eq!(u, Vector::from([]));
    }

    #[test]
    #[should_panic(expected = "size mismatch")]
    fn test_substraction_f32_panic_size_mismatch() {
        let mut u: Vector<f32> = Vector::from([1., 2.]);
        let v: Vector<f32> = Vector::from([1., 2., 3.]);
        u.sub(&v);
    }

    // Scalar f32
    #[test]
    fn test_scalar_f32_basic() {
        let mut u: Vector<f32> = Vector::from([2., 3.]);
        u.scl(2.);
        assert_eq!(u, Vector::from([4., 6.]));

        let mut u: Vector<f32> = Vector::from([-3., 5., -1.]);
        u.scl(3.);
        assert_eq!(u, Vector::from([-9., 15., -3.]));

        let mut u: Vector<f32> = Vector::from([1.]);
        u.scl(4.);
        assert_eq!(u, Vector::from([4.]));
    }

    #[test]
    fn test_scalar_f32_zero() {
        let mut u: Vector<f32> = Vector::from([1., 2.]);
        u.scl(0.);
        assert_eq!(u, Vector::from([0., 0.]));

        let mut u: Vector<f32> = Vector::from([0., 0.]);
        u.scl(0.);
        assert_eq!(u, Vector::from([0., 0.]));
    }

    #[test]
    fn test_scalar_f32_empty() {
        let mut u: Vector<f32> = Vector::from([]);
        u.scl(1.);
        assert_eq!(u, Vector::from([]));
    }

    // Linear combination f32
    #[test]
    fn test_linear_combination_f32_basic() {
        let e1: Vector<f32> = Vector::from([1., 0., 0.]);
        let e2: Vector<f32> = Vector::from([0., 1., 0.]);
        let e3: Vector<f32> = Vector::from([0., 0., 1.]);
        assert_eq!(
            linear_combination(&[e1, e2, e3], &[10., -2., 0.5]),
            Vector::from([10., -2., 0.5])
        );

        let v1: Vector<f32> = Vector::from([1., 2., 3.]);
        let v2: Vector<f32> = Vector::from([0., 10., -100.]);
        assert_eq!(
            linear_combination(&[v1, v2], &[10., -2.]),
            Vector::from([10., 0., 230.])
        );
    }

    #[test]
    fn test_linear_combination_f32_zero() {
        let e1: Vector<f32> = Vector::from([1., 2., 3.]);
        let e2: Vector<f32> = Vector::from([4., 5., 6.]);
        let e3: Vector<f32> = Vector::from([7., 8., 9.]);
        assert_eq!(
            linear_combination(&[e1, e2, e3], &[0., 0., 0.]),
            Vector::from([0., 0., 0.])
        );

        let e1: Vector<f32> = Vector::from([0., 0., 0.]);
        let e2: Vector<f32> = Vector::from([0., 0., 0.]);
        let e3: Vector<f32> = Vector::from([0., 0., 0.]);
        assert_eq!(
            linear_combination(&[e1, e2, e3], &[0., 0., 0.]),
            Vector::from([0., 0., 0.])
        );
    }

    #[test]
    fn test_linear_combination_f32_empty() {
        let e1: Vector<f32> = Vector::from([]);
        let e2: Vector<f32> = Vector::from([]);
        let e3: Vector<f32> = Vector::from([]);
        assert_eq!(
            linear_combination(&[e1, e2, e3], &[1., 2., 3.]),
            Vector::from([])
        );

        let empty_vectors: [Vector<f32>; 0] = [];
        let empty_coefs: [f32; 0] = [];
        assert_eq!(
            linear_combination(&empty_vectors, &empty_coefs),
            Vector::from([])
        );
    }

    #[test]
    #[should_panic(expected = "vectors but")]
    fn test_linear_combination_f32_panic_length_mismatch() {
        let e1: Vector<f32> = Vector::from([1., 2., 3.]);
        let e2: Vector<f32> = Vector::from([4., 5., 6.]);
        let e3: Vector<f32> = Vector::from([7., 8., 9.]);
        linear_combination(&[e1, e2, e3], &[1., 2.]);
    }

    #[test]
    #[should_panic(expected = "but expected")]
    fn test_linear_combination_f32_panic_inconsistent_size() {
        let e1: Vector<f32> = Vector::from([1., 2., 3.]);
        let e2: Vector<f32> = Vector::from([4., 5., 6.]);
        let e3: Vector<f32> = Vector::from([7., 8.]);
        linear_combination(&[e1, e2, e3], &[1., 2., 3.]);
    }

    // Linear interpolation

    // Dot product f32
    #[test]
    fn test_dot_product_f32_basic() {
        let u: Vector<f32> = Vector::from([1., 1.]);
        let v: Vector<f32> = Vector::from([1., 1.]);
        assert_eq!(u.dot(&v), 2.);

        let u: Vector<f32> = Vector::from([-1., 6.]);
        let v: Vector<f32> = Vector::from([3., 2.]);
        assert_eq!(u.dot(&v), 9.);

        let u: Vector<f32> = Vector::from([1., -1.]);
        let v: Vector<f32> = Vector::from([2., 2.]);
        assert_eq!(u.dot(&v), 0.);
    }

    #[test]
    fn test_dot_product_f32_zero() {
        let u: Vector<f32> = Vector::from([0., 0.]);
        let v: Vector<f32> = Vector::from([0., 0.]);
        assert_eq!(u.dot(&v), 0.);
    }

    #[test]
    fn test_dot_product_f32_empty() {
        let u: Vector<f32> = Vector::from([]);
        let v: Vector<f32> = Vector::from([]);
        assert_eq!(u.dot(&v), 0.);
    }

    #[test]
    #[should_panic(expected = "size mismatch")]
    fn test_dot_product_f32_panic_size_mismatch() {
        let u: Vector<f32> = Vector::from([1., 2.]);
        let v: Vector<f32> = Vector::from([1., 2., 3.]);
        u.dot(&v);
    }

    // Norms f32
    #[test]
    fn test_norms_f32_basic() {
        let u: Vector<f32> = Vector::from([1., 2., 3.]);
        assert_eq!(u.norm_1(), 6.);
        assert_eq!(u.norm(), 3.74165738);
        assert_eq!(u.norm_inf(), 3.);

        let u: Vector<f32> = Vector::from([-1., -2.]);
        assert_eq!(u.norm_1(), 3.);
        assert_eq!(u.norm(), 2.236067977);
        assert_eq!(u.norm_inf(), 2.);
    }

    #[test]
    fn test_norms_f32_zero() {
        let u: Vector<f32> = Vector::from([0., 0., 0.]);
        assert_eq!(u.norm_1(), 0.);
        assert_eq!(u.norm(), 0.);
        assert_eq!(u.norm_inf(), 0.);
    }

    #[test]
    fn test_norms_f32_empty() {
        let u: Vector<f32> = Vector::from([]);
        assert_eq!(u.norm_1(), 0.);
        assert_eq!(u.norm(), 0.);
        assert_eq!(u.norm_inf(), 0.);
    }

    // Cosine f32
    #[test]
    fn test_angle_cos_f32_basic() {
        let u: Vector<f32> = Vector::from([1., 0.]);
        let v: Vector<f32> = Vector::from([1., 0.]);
        assert_eq!(angle_cos(&u, &v), 1.);

        let u: Vector<f32> = Vector::from([1., 0.]);
        let v: Vector<f32> = Vector::from([0., 1.]);
        assert_eq!(angle_cos(&u, &v), 0.);

        let u: Vector<f32> = Vector::from([-1., 1.]);
        let v: Vector<f32> = Vector::from([1., -1.]);
        assert!(approx_eq(angle_cos(&u, &v), -1.));

        let u: Vector<f32> = Vector::from([2., 1.]);
        let v: Vector<f32> = Vector::from([4., 2.]);
        assert_eq!(angle_cos(&u, &v), 1.);

        let u: Vector<f32> = Vector::from([1., 2., 3.]);
        let v: Vector<f32> = Vector::from([4., 5., 6.]);
        assert!(approx_eq(angle_cos(&u, &v), 0.974631846));

        let u: Vector<f32> = Vector::from([1.]);
        let v: Vector<f32> = Vector::from([3.]);
        assert_eq!(angle_cos(&u, &v), 1.);
    }

    #[test]
    #[should_panic(expected = "undefined for empty vectors")]
    fn test_angle_cos_f32_panic_empty() {
        let u: Vector<f32> = Vector::from([]);
        let v: Vector<f32> = Vector::from([]);
        angle_cos(&u, &v);
    }

    #[test]
    #[should_panic(expected = "undefined for zero vectors")]
    fn test_angle_cos_f32_panic_zero() {
        let u: Vector<f32> = Vector::from([1., 2.]);
        let v: Vector<f32> = Vector::from([0., 0.]);
        angle_cos(&u, &v);
    }

    // Cross product f32
    #[test]
    fn test_cross_product_f32_basic() {
        let u: Vector<f32> = Vector::from([0., 0., 1.]);
        let v: Vector<f32> = Vector::from([1., 0., 0.]);
        assert_eq!(cross_product(&u, &v), Vector::from([0., 1., 0.]));

        let u: Vector<f32> = Vector::from([1., 2., 3.]);
        let v: Vector<f32> = Vector::from([4., 5., 6.]);
        assert_eq!(cross_product(&u, &v), Vector::from([-3., 6., -3.]));

        let u: Vector<f32> = Vector::from([4., 2., -3.]);
        let v: Vector<f32> = Vector::from([-2., -5., 16.]);
        assert_eq!(cross_product(&u, &v), Vector::from([17., -58., -16.]));
    }

    #[test]
    fn test_cross_product_f32_zero() {
        let u: Vector<f32> = Vector::from([1., 2., 3.]);
        let v: Vector<f32> = Vector::from([0., 0., 0.]);
        assert_eq!(cross_product(&u, &v), Vector::from([0., 0., 0.]));

        let u: Vector<f32> = Vector::from([0., 0., 0.]);
        let v: Vector<f32> = Vector::from([0., 0., 0.]);
        assert_eq!(cross_product(&u, &v), Vector::from([0., 0., 0.]));
    }

    #[test]
    #[should_panic(expected = "undefined for non-three-dimensional vectors")]
    fn test_cross_product_f32_panic_empty() {
        let u: Vector<f32> = Vector::from([]);
        let v: Vector<f32> = Vector::from([]);
        cross_product(&u, &v);
    }

    #[test]
    #[should_panic(expected = "size mismatch")]
    fn test_cross_product_f32_panic_size_mismatch() {
        let u: Vector<f32> = Vector::from([1., 2.]);
        let v: Vector<f32> = Vector::from([1., 2., 3.]);
        cross_product(&u, &v);
    }

    #[test]
    #[should_panic(expected = "undefined for non-three-dimensional vectors")]
    fn test_cross_product_f32_panic_non_three_dimensional() {
        let u: Vector<f32> = Vector::from([1., 2.]);
        let v: Vector<f32> = Vector::from([1., 2.]);
        cross_product(&u, &v);
    }
}
