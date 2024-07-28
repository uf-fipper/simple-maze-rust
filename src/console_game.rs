use clap::Parser;
use rand::{thread_rng, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::{
    errors::{MazeError, MazeResult},
    game::{Game, GameField, GameValueMap, ToGameValue},
    maze_map::MazeMap,
    move_status::MoveStatus,
    player::Player,
    point::Point,
};

#[derive(Debug, Parser)]
#[command(
    version = "v0.1.0",
    // author = "author",
    about = "maze command",
    // long_about = "long about",
    multicall = true
)]
pub enum Cli {
    #[command(visible_aliases = ["w", "W"], about = "move up")]
    Up,
    #[command(visible_aliases = ["s", "S"], about = "move down")]
    Down,
    #[command(visible_aliases = ["a", "A"], about = "move left")]
    Left,
    #[command(visible_aliases = ["d", "D"], about = "move right")]
    Right,
    #[command(visible_aliases = ["r"], about = "restart the game")]
    Restart,
    #[command(about = "new game")]
    New(SubcommandNew),
    #[command(about = "solve the game")]
    Solve,
    #[command(visible_aliases = ["unsolve"], about = "unsolve the game")]
    UnSolve,
    #[command(about = "quit the game")]
    Quit,
    #[command(visible_aliases = ["disp"], about = "display")]
    Display,
}

#[derive(Debug, Parser)]
pub struct SubcommandNew {
    pub row: i32,
    pub column: i32,
    pub seed: Option<u64>,
}

enum RunOnceResult {
    Ok,
    InValid,
    CanNotMove,
    Quit,
    Error(String),
    CmdError(String),
    Display,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConsoleGame<R = ChaCha8Rng>
where
    R: Rng,
{
    map: MazeMap<R>,
    player: Player,
    is_move: bool,
    value_map: GameValueMap<String>,
    will_solve: bool,
    solve_list: Option<Vec<Point>>,
    move_list: Option<Vec<Point>>,
}

impl ConsoleGame<ChaCha8Rng> {
    pub fn new(row: i32, column: i32) -> MazeResult<Self> {
        let random = match ChaCha8Rng::from_rng(thread_rng()) {
            Ok(r) => r,
            Err(e) => return Err(MazeError::Init(e.to_string())),
        };
        let map = MazeMap::new_with_random(row, column, random)?;
        let player = Player::new(map.st, "player");
        Ok(Self {
            map,
            player,
            is_move: false,
            value_map: Self::new_value_map(),
            will_solve: false,
            solve_list: None,
            move_list: None,
        })
    }
}

impl<R> ConsoleGame<R>
where
    R: Rng + SeedableRng,
{
    fn new_value_map() -> GameValueMap<String> {
        GameValueMap {
            empty: "?".to_owned(),
            r#move: ".".to_owned(),
            solve: "#".to_owned(),
            wall: "O".to_owned(),
            road: " ".to_owned(),
            border: "O".to_owned(),
            player: "P".to_owned(),
            st: "S".to_owned(),
            ed: "E".to_owned(),
        }
    }

    pub fn new_with_random(row: i32, column: i32, random: R) -> MazeResult<Self> {
        let map = MazeMap::new_with_random(row, column, random)?;
        let player = Player::new(map.st, "player");
        Ok(Self {
            map,
            player,
            is_move: false,
            value_map: Self::new_value_map(),
            will_solve: false,
            solve_list: None,
            move_list: None,
        })
    }

    fn reset(&mut self) {
        self.is_move = false;
        self.move_list = None;
        self.solve_list = None;
    }

    pub fn restart(&mut self) {
        self.player.pos = self.map.st;
        self.move_list = None;
        self.solve_list = None;
    }

    pub fn new_game(&mut self, row: i32, column: i32) -> MazeResult<()> {
        self.map.generate(row, column)?;
        self.player = Player::new(self.map.st, "player");
        self.reset();
        Ok(())
    }

    pub fn new_game_with_random(&mut self, row: i32, column: i32, random: R) -> MazeResult<()> {
        self.map.generate_with_new_random(row, column, random)?;
        self.player = Player::new(self.map.st, "player");
        self.reset();
        Ok(())
    }
}

impl<R> GameField<R, String> for ConsoleGame<R>
where
    R: Rng,
{
    fn map(&self) -> &MazeMap<R> {
        &self.map
    }

    fn player(&self) -> &Player {
        &self.player
    }
}

impl<R> Game<R, String> for ConsoleGame<R>
where
    R: Rng + SeedableRng,
{
    fn display(&self) -> MazeResult<()> {
        let player_name_str = format!("player name: {}", self.player.name);
        let step_str = format!("step: {}", self.player.step);
        let move_times_str = format!("move times: {}", self.player.move_times);
        let mut res_list = vec![
            vec![&player_name_str],
            vec![&step_str],
            vec![&move_times_str],
        ];
        let mut map_list: Vec<Vec<&String>> = self
            .map
            .map
            .iter()
            .map(|line| line.iter().map(|value| value.to(&self.value_map)).collect())
            .collect();
        // solve
        let solve_list;
        if self.will_solve {
            solve_list = self.solve(self.player.pos)?;
        } else {
            solve_list = vec![];
        }
        for p in solve_list {
            map_list[p] = &self.value_map.solve;
        }
        // move
        if let Some(move_list) = &self.move_list {
            for p in move_list.iter() {
                map_list[*p] = &self.value_map.r#move;
            }
        }
        // player
        map_list[self.player.pos] = &self.value_map.player;

        res_list.extend(map_list);
        let res_string = res_list
            .iter()
            .map(|line| {
                line.iter()
                    .map(|&s| s.as_str())
                    .collect::<Vec<&str>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");
        println!("{}", res_string);
        Ok(())
    }

    fn after_move(
        &mut self,
        r#_move: crate::move_status::MoveStatus,
        move_list: Vec<crate::point::Point>,
        step: i32,
    ) -> MazeResult<Vec<crate::point::Point>> {
        self.player.pos = move_list[step as usize];
        self.player.step += step;
        self.player.move_times += 1;

        self.is_move = true;
        Ok(move_list)
    }

    fn after_move_player(&mut self, pos: crate::point::Point) -> MazeResult<()> {
        self.player.pos = pos;
        Ok(())
    }

    fn run(&mut self) -> MazeResult<()> {
        println!("game start!");
        self.display().unwrap();
        loop {
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap_or_else(|e| {
                println!("{}", e.to_string());
                0
            });
            self.move_list = None;
            match self.run_once(buf.trim()) {
                RunOnceResult::Ok => self
                    .display()
                    .unwrap_or_else(|e| println!("{}", e.to_string())),
                RunOnceResult::InValid => println!("invalid input"),
                RunOnceResult::CanNotMove => println!("can not move"),
                RunOnceResult::Quit => break,
                RunOnceResult::Error(err) => println!("error: {}", err),
                RunOnceResult::CmdError(err) => println!("{}", err),
                RunOnceResult::Display => self
                    .display()
                    .unwrap_or_else(|e| println!("{}", e.to_string())),
            }
        }
        Ok(())
    }
}

// run
impl<R> ConsoleGame<R>
where
    R: Rng + SeedableRng,
{
    fn inner_move(&mut self, status: MoveStatus) -> RunOnceResult {
        match self.move_to(status) {
            Ok(move_list) => {
                self.move_list = Some(move_list);
                RunOnceResult::Ok
            }
            Err(err) => match err {
                MazeError::CanNotMove => RunOnceResult::CanNotMove,
                other => RunOnceResult::Error(other.to_string()),
            },
        }
    }

    fn run_once(&mut self, cmd: &str) -> RunOnceResult {
        let itr = match shlex::split(cmd) {
            Some(itr) => itr,
            None => return RunOnceResult::InValid,
        };
        let cli = match Cli::try_parse_from(itr) {
            Ok(cli) => cli,
            Err(e) => return RunOnceResult::CmdError(e.to_string()),
        };
        match cli {
            Cli::Up => return self.inner_move(MoveStatus::Up),
            Cli::Down => return self.inner_move(MoveStatus::Down),
            Cli::Left => return self.inner_move(MoveStatus::Left),
            Cli::Right => return self.inner_move(MoveStatus::Right),
            Cli::Restart => {
                self.restart();
                return RunOnceResult::Ok;
            }
            Cli::New(sub) => {
                if let Some(state) = sub.seed {
                    match self.new_game_with_random(sub.row, sub.column, R::seed_from_u64(state)) {
                        Ok(_) => {}
                        Err(e) => return RunOnceResult::Error(e.to_string()),
                    };
                } else {
                    match self.new_game(sub.row, sub.column) {
                        Ok(_) => {}
                        Err(e) => return RunOnceResult::Error(e.to_string()),
                    }
                }
            }
            Cli::Solve => self.will_solve = true,
            Cli::UnSolve => self.will_solve = false,
            Cli::Quit => return RunOnceResult::Quit,
            Cli::Display => return RunOnceResult::Display,
        };
        RunOnceResult::Ok
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    #[test]
    fn test_run() {
        let seed = 123;
        println!("{:?}", seed);
        let random = ChaCha8Rng::seed_from_u64(seed);
        let mut game = ConsoleGame::new_with_random(10, 20, random).unwrap();
        assert!(!game.is_win().unwrap());
        game.move_to(MoveStatus::Right).unwrap();
        game.move_to(MoveStatus::Right).unwrap();
        game.move_to(MoveStatus::Right).unwrap();
        game.move_to(MoveStatus::Right).unwrap();
        game.move_to(MoveStatus::Down).unwrap();
        game.move_to(MoveStatus::Right).unwrap();
        game.move_to(MoveStatus::Right).unwrap();
        game.move_to(MoveStatus::Right).unwrap();
        game.move_to(MoveStatus::Down).unwrap();
        game.move_to(MoveStatus::Down).unwrap();
        assert!(game.is_win().unwrap());
    }
}
