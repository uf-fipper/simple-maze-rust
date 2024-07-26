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
            Self::QueueEmpty => write!(f, "队列为空"),
            Self::SolveException => write!(f, "求解失败"),
            Self::GameWin => write!(f, "游戏已结束"),
            Self::CanNotMove => write!(f, "不能这么移动"),
        }
    }
}

impl std::error::Error for MazeError {}

pub type MazeResult<T> = Result<T, MazeError>;
