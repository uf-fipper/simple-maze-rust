use clap::Parser;
use rand::{rngs::ThreadRng, Rng, SeedableRng};
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
    version = "v0.1.24",
    author = "author",
    about = "about",
    long_about = "long about",
    multicall = true // 加上这行可以不用带可执行文件名称参数
)]
enum Cli {
    #[command(aliases = ["w", "W"], help_template = "move up")]
    Up,
    #[command(aliases = ["s", "S"], help_template = "move down")]
    Down,
    #[command(aliases = ["a", "A"], help_template = "move left")]
    Left,
    #[command(aliases = ["d", "D"], help_template = "move right")]
    Right,
    #[command(aliases = ["r"], help_template = "restart the game")]
    Restart,
    #[command(help_template = "new game")]
    New(SubcommandNew),
    #[command(help_template = "solve the game")]
    Solve,
    #[command(help_template = "unsolve the game")]
    UnSolve,
    #[command(help_template = "quit the game")]
    Quit,
}

#[derive(Debug, Parser)]
struct SubcommandNew {
    row: i32,
    column: i32,
    seed: Option<String>,
}

enum RunOnceResult {
    Ok,
    InValid,
    CanNotMove,
    Quit,
    Error(String),
    CmdError(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConsoleGame<R = ThreadRng>
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

impl ConsoleGame<ThreadRng> {
    pub fn new(row: i32, column: i32) -> MazeResult<Self> {
        let map = MazeMap::new(row, column)?;
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
    R: Rng,
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
    R: Rng,
{
    fn display(&self) -> MazeResult<()> {
        let mut res_list: Vec<Vec<&str>> = self
            .map
            .map
            .iter()
            .map(|line| {
                line.iter()
                    .map(|value| value.to(&self.value_map).as_str())
                    .collect()
            })
            .collect();
        // solve
        if let Some(solve_list) = &self.solve_list {
            for p in solve_list.iter() {
                res_list[*p] = self.value_map.solve.as_str();
            }
        }
        // move
        if let Some(move_list) = &self.move_list {
            for p in move_list.iter() {
                res_list[*p] = self.value_map.r#move.as_str();
            }
        }
        // player
        res_list[self.player.pos] = self.value_map.player.as_str();
        let res_string = res_list
            .iter()
            .map(|line| line.join(""))
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
            std::io::stdin().read_line(&mut buf).unwrap();
            self.move_list = None;
            match self.run_once(buf.trim()) {
                RunOnceResult::Ok => {}
                RunOnceResult::InValid => println!("invalid input"),
                RunOnceResult::CanNotMove => println!("can not move"),
                RunOnceResult::Quit => break,
                RunOnceResult::Error(err) => println!("error: {}", err),
                RunOnceResult::CmdError(err) => println!("{}", err),
            }
            self.display().unwrap();
        }
        Ok(())
    }
}

// run
impl<R> ConsoleGame<R>
where
    R: Rng,
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

    fn inner_solve(&mut self) -> RunOnceResult {
        match self.solve(self.player.pos) {
            Ok(solve_list) => {
                self.solve_list = Some(solve_list);
                RunOnceResult::Ok
            }
            Err(err) => RunOnceResult::Error(err.to_string()),
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
                if let Some(str) = sub.seed {
                    let seed = &mut Self::to_seed(str.as_str());
                    self.map.random.fill(seed);
                }
                match self.new_game(sub.row, sub.column) {
                    Ok(_) => {}
                    Err(e) => return RunOnceResult::Error(e.to_string()),
                };
            }
            Cli::Solve => self.will_solve = true,
            Cli::UnSolve => self.will_solve = false,
            Cli::Quit => return RunOnceResult::Quit,
        };
        if self.will_solve {
            return self.inner_solve();
        } else {
            self.solve_list = None;
            RunOnceResult::Ok
        }
    }

    fn to_seed(raw: &str) -> <ChaCha8Rng as SeedableRng>::Seed {
        let u8array: &[u8] = raw.as_bytes();
        let mut seed: <ChaCha8Rng as SeedableRng>::Seed;
        if u8array.len() >= 32 {
            seed = u8array[0..32].try_into().unwrap();
        } else {
            seed = [0; 32];
            seed[..u8array.len()].clone_from_slice(u8array);
        }
        seed
    }
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    use super::*;

    #[test]
    fn test_run() {
        let u8array: &[u8] = "aosndfjewnjqkerkjwqnerijwerneafqwefqqweffe".as_bytes();
        let mut seed: <ChaCha8Rng as SeedableRng>::Seed;
        if u8array.len() >= 32 {
            seed = u8array[0..32].try_into().unwrap();
        } else {
            seed = [0; 32];
            seed[..u8array.len()].clone_from_slice(u8array);
        }
        println!("{:?}", seed);
        let mut random = thread_rng();
        random.fill(&mut seed);
        ConsoleGame::new_with_random(10, 20, random)
            .unwrap()
            .run()
            .unwrap();
    }
}
