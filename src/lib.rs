use std::collections::HashMap;

pub enum DbError{
    DimensionMismatch{
        expected:usize,
        actual: usize,
    }
}

#[derive(Debug)]
pub struct VectorDb{
    dimension: usize,
    vectors: HashMap<u64, Vec<f32>>,
}

impl VectorDb{
    pub fn new(dimension: usize) -> Self {
        Self {
            dimension,
            vectors: HashMap::new(),
        }
    }
pub fn insert(&mut self, id: u64, vector: Vec<f32>) -> Result<(), DbError>{
        if vector.len() != self.dimension {
            return Err(DbError::DimensionMismatch {
                expected: self.dimension,
                actual: vector.len(),
            });
        }

        self.vectors.insert(id, vector);
        Ok(())
    }
}

impl std::fmt::Display for VectorDb{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, vector) in &self.vectors{
            writeln!(f,"id: {id}, vector: {vector:?}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests;
