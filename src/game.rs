use rand::Rng;

use crate::{
    errors::{MazeError, MazeResult},
    game_value::GameValue,
    map_value::MapValue,
    maze_map::MazeMap,
    move_status::MoveStatus,
    player::Player,
    point::Point,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct GameValueMap<T> {
    pub empty: T,
    pub r#move: T,
    pub solve: T,
    pub wall: T,
    pub road: T,
    pub border: T,
    pub player: T,
    pub st: T,
    pub ed: T,
}

pub trait ToGameValue {
    fn to<'a, U>(&self, map: &'a GameValueMap<U>) -> &'a U;
}

impl ToGameValue for MapValue {
    fn to<'a, U>(&self, map: &'a GameValueMap<U>) -> &'a U {
        match self {
            MapValue::Empty => &map.empty,
            MapValue::Wall => &map.wall,
            MapValue::Road => &map.road,
            MapValue::Border => &map.border,
            MapValue::St => &map.st,
            MapValue::Ed => &map.ed,
        }
    }
}

impl ToGameValue for GameValue {
    fn to<'a, U>(&self, map: &'a GameValueMap<U>) -> &'a U {
        match self {
            GameValue::Empty => &map.empty,
            GameValue::Move => &map.r#move,
            GameValue::Solve => &map.solve,
        }
    }
}

pub trait GameField<Random, T>
where
    Random: Rng,
{
    fn map(&self) -> &MazeMap<Random>;
    fn player(&self) -> &Player;

    fn row(&self) -> usize {
        self.map().row()
    }
    fn column(&self) -> usize {
        self.map().column()
    }
    fn random(&self) -> &Random {
        &self.map().random
    }
}

pub trait Game<Random, T>: GameField<Random, T>
where
    Random: Rng,
{
    fn move_find_road(&mut self, p: Point, lp: Point) -> MazeResult<Option<Point>> {
        let mut res = None;
        for p2 in p.get_range_vec() {
            if self.map().is_overrange(p2) {
                continue;
            }
            if p2 == lp {
                continue;
            }
            if self.map()[p2] == MapValue::Wall {
                continue;
            }
            if let Some(_) = res {
                return Ok(None);
            }
            res = Some(p2)
        }
        Ok(res)
    }

    fn move_to(&mut self, r#move: MoveStatus) -> MazeResult<Vec<Point>> {
        if self.is_win()? {
            return Err(MazeError::GameWin);
        }
        let mut lp = self.player().pos;
        let mut p = r#move.get_next(lp);

        if self.map().is_overrange(p) {
            return Err(MazeError::CanNotMove);
        }
        if self.map()[p] == MapValue::Wall {
            return Err(MazeError::CanNotMove);
        }
        let mut move_list = Vec::new();
        move_list.push(lp);
        move_list.push(p);
        let mut step: i32 = 1;
        let mut next_road_raw = self.move_find_road(p, lp)?;
        loop {
            let next_road = match next_road_raw {
                Some(p) => p,
                None => break,
            };
            if p == self.map().ed {
                break;
            }
            step += 1;
            lp = p;
            p = next_road;
            move_list.push(p);
            next_road_raw = self.move_find_road(p, lp)?;
        }
        self.after_move(r#move, move_list, step)
    }

    fn after_move(
        &mut self,
        r#_move: MoveStatus,
        move_list: Vec<Point>,
        _step: i32,
    ) -> MazeResult<Vec<Point>> {
        Ok(move_list)
    }

    fn after_move_player(&mut self, _pos: Point) -> MazeResult<()> {
        Ok(())
    }

    fn move_player(&mut self, pos: Point) -> MazeResult<()> {
        if self.map().is_overrange(pos) {
            return Err(MazeError::CanNotMove);
        }
        let value = self.map()[pos];
        if value != MapValue::Road && value != MapValue::St && value != MapValue::Ed {
            return Err(MazeError::CanNotMove);
        }
        self.after_move_player(pos)
    }

    fn solve(&self, pos: Point) -> MazeResult<Vec<Point>> {
        self.map().solve(pos)
    }

    fn is_win(&self) -> MazeResult<bool> {
        Ok(self.map().ed == self.player().pos)
    }

    fn display(&self) -> MazeResult<()> {
        Ok(())
    }

    fn display_mut(&mut self) -> MazeResult<()> {
        self.display()
    }

    fn run(&mut self) -> MazeResult<()>;
}
