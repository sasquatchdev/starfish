use super::Source;

#[derive(Debug, Clone)]
pub struct Location {
    source: Source,
    index: usize,
}

impl Location {
    pub fn new(source: Source, index: usize) -> Self {
        Self { source, index }
    }

    pub fn new_start(source: Source) -> Self {
        Self::new(source, 0)
    }
    
    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn advance_by(&mut self, n: usize) {
        self.index += n;
    }
    
    pub fn advanced(&self) -> Self {
        Self::new(self.source.clone(), self.index + 1)
    }

    pub fn advanced_by(&self, n: usize) -> Self {
        Self::new(self.source.clone(), self.index + n)
    }

    pub fn is_eof(&self) -> bool {
        self.index >= self.source.1.len()
    }
}
