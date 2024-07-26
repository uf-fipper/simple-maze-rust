use crate::point::Point;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum MoveStatus {
    Up,
    Down,
    Left,
    Right,
}

impl MoveStatus {
    pub fn get_next(&self, p: Point) -> Point {
        match self {
            Self::Up => p - (1, 0),
            Self::Down => p + (1, 0),
            Self::Left => p - (0, 1),
            Self::Right => p + (0, 1),
        }
    }
}
