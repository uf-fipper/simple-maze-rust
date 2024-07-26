#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GameValue {
    Empty,
    Move,
    Solve,
}

impl Default for GameValue {
    fn default() -> Self {
        Self::Empty
    }
}
