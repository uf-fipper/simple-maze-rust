use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum MazeError {
    Init(String),
    QueueEmpty,
    SolveException,
    GameWin,
    CanNotMove,
}

impl Display for MazeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Init(value) => write!(f, "{}", value),
            Self::QueueEmpty => write!(f, "queue empty"),
            Self::SolveException => write!(f, "solve failed"),
            Self::GameWin => write!(f, "game is over"),
            Self::CanNotMove => write!(f, "can not move"),
        }
    }
}

impl std::error::Error for MazeError {}

pub type MazeResult<T> = Result<T, MazeError>;
