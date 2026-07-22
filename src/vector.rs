use std::fmt;

pub struct Vector<K> {
    data: Vec<K>,
}

impl<K> Vector<K> {
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl<K, const N: usize> From<[K; N]> for Vector<K> {
    fn from(data: [K; N]) -> Self {
        Self {
            data: data.into_iter().collect(),
        }
    }
}

impl<K: fmt::Display> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in self.data {
            writeln!(f, "[{}]", x)?;
        }

        Ok(())
    }
}

impl<K> Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl<K> IndexMut<usize> for Vector<K> {
    fn index(&mut self, i: usize) -> &mut Self::Output {
        &mut self.data[i];
    }
}

impl<K> Vector<K>
where
    K: Copy + std::ops::AddAssign,
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
    K: Copy + std::ops::SubAssign,
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
    K: Copy + std::ops::MulAssign,
{
    pub fn scl(&mut self, a: K) {
        for x in self.data.iter_mut() {
            *x *= a;
        }
    }
}

pub fn linear_combination(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
    K: Copy + Default + std::ops::AddAssign + Mul<Output = K>,
{
    // assert_eq!(u.size(), v.size(), "FUCK");
    assert!(!u.is_empty(), "linear_combination: empty input");

    let n = u[0].size();
    let mut result = vec![K::default(); n];

    for (vector, &coef) in u.iter().zip(coefs.iter()) {
        // assert_eq
        for (r, &x) in result.iter_mut().zip(vector.data.iter()) {
            *r += x * coef;
        }
    }

    Vector { data: result }
}

// #[cfg(test)]
// mod tests {

// }
