#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MapValue {
    Empty,
    Wall,
    Road,
    Border,
    St,
    Ed,
}

impl Default for MapValue {
    fn default() -> Self {
        Self::Empty
    }
}
