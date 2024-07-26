use crate::point::Point;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Player {
    pub pos: Point,
    pub name: String,
    pub step: i32,
    pub move_times: i32,
}

impl Player {
    pub fn new(pos: Point, name: impl Into<String>) -> Self {
        Self {
            pos,
            name: name.into(),
            step: 0,
            move_times: 0,
        }
    }
}
