use super::location::Location;

#[derive(Debug, Clone)]
pub struct Span {
    start: Location,
    end: Location,
}

impl Span {
    pub fn new(start: Location, end: Location) -> Self {
        Span { start, end }
    }

    pub fn new_length(start: Location, length: usize) -> Self {
        Span {
            start: start.clone(),
            end: start.advanced_by(length),
        }
    }

    pub fn start(&self) -> Location {
        self.start.clone()
    }

    pub fn end(&self) -> Location {
        self.end.clone()
    }

    pub fn length(&self) -> usize {
        self.end.index() - self.start.index()
    }
}
