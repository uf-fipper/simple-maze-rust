use std::{
    collections::VecDeque,
    ops::{Index, IndexMut},
};

use rand::{rngs::ThreadRng, Rng};

use crate::{
    errors::{MazeError, MazeResult},
    map_value::MapValue,
    point::{CanPointIndex, Point},
    random::randarray,
};

type TMap = Vec<Vec<MapValue>>;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct MazeMap<Random = ThreadRng>
where
    Random: Rng,
{
    pub random: Random,
    pub map: TMap,
    pub inst_st: Point,
    pub st: Point,
    pub ed: Point,
}

impl<Random> MazeMap<Random>
where
    Random: Rng,
{
    pub fn row(&self) -> usize {
        self.map.len()
    }

    pub fn column(&self) -> usize {
        self.map[0].len()
    }

    pub fn is_overrange(&self, p: Point) -> bool {
        let (i, j) = (p.0, p.1);
        if i < 0 || j < 0 {
            return true;
        }
        if i as usize >= self.row() || j as usize >= self.column() {
            return true;
        }
        return false;
    }

    fn init_get_walls(&self, p: Point, lp: Point) -> Vec<Point> {
        let mut result = vec![];
        for point in p.get_range_vec() {
            if self.is_overrange(point) {
                continue;
            }
            if point == lp {
                continue;
            }
            result.push(point);
        }
        result
    }

    fn init_check_walls(&self, p: Point, lp: Point) -> bool {
        let temp = self.init_get_walls(p, lp);
        if temp.is_empty() {
            return false;
        }
        for t in temp {
            if self.map[t] != MapValue::Wall {
                return false;
            }
        }
        true
    }

    fn init_map(&mut self) -> MazeResult<()> {
        let mut stack = VecDeque::new();
        let mut p = self.inst_st;
        let mut lp = Point(-1, -1);
        let mut step = 0;
        stack.push_back((p, lp, step));
        while !stack.is_empty() {
            (p, lp, step) = stack.pop_back().unwrap();
            if !self.init_check_walls(p, lp) {
                continue;
            }
            self.map[p] = MapValue::Road;
            let mut around_walls = self.init_get_walls(p, lp);
            if around_walls.is_empty() {
                continue;
            }
            around_walls = randarray(&mut self.random, &around_walls);
            for wall in around_walls {
                stack.push_back((wall, p, step + 1));
            }
        }

        // 是否获取了 st 和 ed
        let mut st_get = false;
        let mut ed_get = false;
        for i in 0..self.row() {
            for j in 0..self.column() {
                if st_get && ed_get {
                    return Ok(());
                }
                if !st_get && self.map[Point(i, j)] == MapValue::Road {
                    self.st = Point(i as i32, j as i32);
                    self.map[self.st] = MapValue::St;
                    st_get = true;
                }
                let ed_idx = Point(self.row() - 1 - i, self.column() - 1 - j);
                if !ed_get && self.map[ed_idx] == MapValue::Road {
                    self.ed = Point(ed_idx.0 as i32, ed_idx.1 as i32);
                    self.map[self.ed] = MapValue::Ed;
                    ed_get = true;
                }
            }
        }
        Ok(())
    }

    pub fn new_with_random(row: i32, column: i32, random: Random) -> MazeResult<Self> {
        if row < 2 || column < 2 {
            return Err(MazeError::Init(String::from("行和列不能小于2")));
        }
        let mut self_ = Self {
            random,
            map: vec![vec![MapValue::Wall; column as usize]; row as usize],
            inst_st: Default::default(),
            st: Default::default(),
            ed: Default::default(),
        };
        self_.init_map()?;
        Ok(self_)
    }

    pub fn generate_with_new_random(
        &mut self,
        row: i32,
        column: i32,
        random: Random,
    ) -> MazeResult<()> {
        self.random = random;
        self.map = vec![vec![MapValue::Wall; column as usize]; row as usize];
        self.inst_st = Default::default();
        self.st = Default::default();
        self.ed = Default::default();
        self.init_map()
    }

    pub fn generate(&mut self, row: i32, column: i32) -> MazeResult<()> {
        self.map = vec![vec![MapValue::Wall; column as usize]; row as usize];
        self.inst_st = Default::default();
        self.st = Default::default();
        self.ed = Default::default();
        self.init_map()
    }

    pub fn re_generate(&mut self) -> MazeResult<()> {
        self.generate(self.row() as i32, self.column() as i32)
    }

    fn solve_get_roads(&self, map_temp: &Vec<Vec<Option<Point>>>, p: Point) -> Vec<Point> {
        p.get_range_vec()
            .iter()
            .filter(|&p2| {
                if self.is_overrange(*p2) {
                    return false;
                }
                if let Some(_) = map_temp[*p2] {
                    return false;
                }
                if self.map[*p2] == MapValue::Wall {
                    return false;
                }
                true
            })
            .map(|p2| *p2)
            .collect()
    }

    pub fn solve(&self, pos: Point) -> MazeResult<Vec<Point>> {
        if pos == self.ed {
            return Ok(vec![pos]);
        }
        let mut queue = VecDeque::new();
        let mut map_temp = vec![vec![None; self.column()]; self.row()];
        let mut p = pos;
        let mut lp = None;
        let mut step = 0;
        queue.push_back((p, lp, step));
        while p != self.ed {
            if queue.is_empty() {
                return Err(MazeError::QueueEmpty);
            }
            (p, lp, step) = queue.pop_front().unwrap();
            map_temp[p] = lp;
            let roads = self.solve_get_roads(&map_temp, p);
            for road in roads {
                queue.push_back((road, Some(p), step + 1));
            }
        }
        let mut rp = map_temp[self.ed].unwrap();
        let mut res = vec![Point::default(); step + 1];
        res[step] = self.ed;
        for i in (0..step).rev() {
            res[i] = rp;
            rp = map_temp[rp].unwrap();
        }
        if res[0] != pos {
            return Err(MazeError::SolveException);
        }
        Ok(res)
    }
}

impl<Random> MazeMap<Random>
where
    Random: Rng + Default,
{
    pub fn new(row: i32, column: i32) -> MazeResult<Self> {
        if row < 2 || column < 2 {
            return Err(MazeError::Init(String::from("行和列不能小于2")));
        }
        let mut self_ = Self {
            map: vec![vec![MapValue::Wall; column as usize]; row as usize],
            ..Default::default()
        };
        self_.init_map()?;
        Ok(self_)
    }
}

impl<Random, T> Index<Point<T>> for MazeMap<Random>
where
    Random: Rng,
    T: CanPointIndex,
{
    type Output = MapValue;

    fn index(&self, index: Point<T>) -> &Self::Output {
        &self.map[index]
    }
}

impl<Random, T> IndexMut<Point<T>> for MazeMap<Random>
where
    Random: Rng,
    T: CanPointIndex,
{
    fn index_mut(&mut self, index: Point<T>) -> &mut Self::Output {
        &mut self.map[index]
    }
}
