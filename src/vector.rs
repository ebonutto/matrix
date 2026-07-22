pub struct Vector<K> {
    data: Vec<K>
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
    let Output = K;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl<K> IndexMut<usize> for Vector<K> {
    fn index(&mut self, i: usize) -> &mut Self::Output {
        &mut self.data[i];
    }
}
