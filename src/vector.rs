pub struct Vector<K> {
    data: Vec<K>
}

impl<K> Vector<K> {
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl<K: fmt::Display> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for _ in data {
            
        }
        writeln!(f, "]")?;
        Ok(())
    }
}
