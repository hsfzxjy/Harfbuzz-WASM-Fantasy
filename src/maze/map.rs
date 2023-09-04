use std::collections::HashSet;
use std::fmt;

use tiny_rng::Rand;
use tiny_rng::Rng;

const UP: usize = 0b1000;
const LEFT: usize = 0b0100;
const RIGHT: usize = 0b0010;
const DOWN: usize = 0b0001;

#[derive(Copy, Clone)]
pub struct WallJunction(usize);

impl WallJunction {
    fn set(&mut self, bit: usize, activate: bool) {
        if activate {
            self.0 |= bit;
        } else {
            self.0 &= !bit;
        }
    }
    fn is(&self, bit: usize) -> bool {
        self.0 & bit != 0
    }

    pub fn set_up(&mut self, activate: bool) {
        self.set(UP, activate)
    }
    pub fn is_up(&self) -> bool {
        self.is(UP)
    }
    pub fn set_left(&mut self, activate: bool) {
        self.set(LEFT, activate)
    }
    pub fn is_left(&self) -> bool {
        self.is(LEFT)
    }
    pub fn set_right(&mut self, activate: bool) {
        self.set(RIGHT, activate)
    }
    pub fn is_right(&self) -> bool {
        self.is(RIGHT)
    }
    pub fn set_down(&mut self, activate: bool) {
        self.set(DOWN, activate)
    }
    pub fn is_down(&self) -> bool {
        self.is(DOWN)
    }
}

impl Default for WallJunction {
    fn default() -> Self {
        WallJunction(0)
    }
}

impl From<WallJunction> for char {
    fn from(wj: WallJunction) -> Self {
        match wj.0 {
            b if b == UP => '╵',
            b if b == UP | LEFT => '┘',
            b if b == UP | LEFT | RIGHT => '┴',
            b if b == UP | LEFT | RIGHT | DOWN => '┼',
            b if b == UP | LEFT | DOWN => '┤',
            b if b == UP | RIGHT => '└',
            b if b == UP | RIGHT | DOWN => '├',
            b if b == UP | DOWN => '│',
            b if b == LEFT => '╴',
            b if b == LEFT | RIGHT => '─',
            b if b == LEFT | RIGHT | DOWN => '┬',
            b if b == LEFT | DOWN => '┐',
            b if b == RIGHT => '╶',
            b if b == RIGHT | DOWN => '┌',
            b if b == DOWN => '╷',
            _ => ' ',
        }
    }
}

impl fmt::Display for WallJunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

const UPPER_LEFT: WallJunction = WallJunction(RIGHT | DOWN);
const UPPER_RIGHT: WallJunction = WallJunction(LEFT | DOWN);
const LOWER_LEFT: WallJunction = WallJunction(RIGHT | UP);
const LOWER_RIGHT: WallJunction = WallJunction(LEFT | UP);
const HORIZONTAL: WallJunction = WallJunction(LEFT | RIGHT);
const VERTICAL: WallJunction = WallJunction(UP | DOWN);

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Left,
    Direction::Right,
    Direction::Down,
];

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub struct Position(
    /// Row
    pub i32,
    /// Column
    pub i32,
);

pub struct Map {
    pub rows: i32,
    pub columns: i32,
    map: Box<[bool]>,
}

impl Map {
    fn new_with_value(rows: i32, columns: i32, value: bool) -> Map {
        Map {
            rows,
            columns,
            map: vec![value; (rows * 2 * columns - (rows + columns)) as usize].into_boxed_slice(),
        }
    }
    pub fn new(rows: i32, columns: i32) -> Map {
        Map::new_with_value(rows, columns, true)
    }

    pub fn generate_prim(rows: i32, columns: i32, start: Position, rng: &mut Rng) -> Map {
        let mut map = Map::new(rows, columns);

        let mut visited = HashSet::new();
        visited.insert(start);
        let mut walls = map.walls_around(&start);

        while !walls.is_empty() {
            let index = rng.rand_usize() % walls.len();
            let (from, dir) = walls.remove(index);
            if let Some(to) = map.move_in_direction(&from, &dir) {
                if !visited.contains(&to) {
                    map.set(&from, &dir, false);

                    visited.insert(to);
                    walls.append(&mut map.walls_around(&to));
                }
            }
        }

        map
    }

    pub fn set_below(&mut self, pos: &Position, closed: bool) {
        self.set_above(&Position(pos.0 - 1, pos.1), closed);
    }
    pub fn is_below(&self, pos: &Position) -> bool {
        self.is_above(&Position(pos.0 - 1, pos.1))
    }
    pub fn set_left(&mut self, pos: &Position, closed: bool) {
        self.set_right(&Position(pos.0, pos.1 - 1), closed);
    }
    pub fn is_left(&self, pos: &Position) -> bool {
        self.is_right(&Position(pos.0, pos.1 - 1))
    }
    pub fn set_right(&mut self, pos: &Position, closed: bool) {
        assert!(pos.0 < self.rows && pos.1 < self.columns - 1);

        self.map[((self.rows - 1) * self.columns + pos.0 * (self.columns - 1) + pos.1) as usize] =
            closed;
    }
    pub fn is_right(&self, pos: &Position) -> bool {
        assert!(pos.0 < self.rows && pos.1 < self.columns - 1);

        self.map[((self.rows - 1) * self.columns + pos.0 * (self.columns - 1) + pos.1) as usize]
    }
    pub fn set_above(&mut self, pos: &Position, closed: bool) {
        assert!(pos.0 < self.rows - 1 && pos.1 < self.columns);

        self.map[(pos.0 * self.columns + pos.1) as usize] = closed;
    }
    pub fn is_above(&self, pos: &Position) -> bool {
        assert!(pos.0 < self.rows - 1 && pos.1 < self.columns);

        self.map[(pos.0 * self.columns + pos.1) as usize]
    }

    pub fn set(&mut self, pos: &Position, dir: &Direction, closed: bool) {
        match dir {
            Direction::Up => self.set_below(pos, closed),
            Direction::Left => self.set_left(pos, closed),
            Direction::Right => self.set_right(pos, closed),
            Direction::Down => self.set_above(pos, closed),
        };
    }
    pub fn is(&self, pos: &Position, dir: &Direction) -> Option<bool> {
        match dir {
            Direction::Up if 0 < pos.0 && pos.0 < self.rows && pos.1 < self.columns => {
                Some(self.is_below(pos))
            }
            Direction::Left if pos.0 < self.rows && 0 < pos.1 && pos.1 < self.columns => {
                Some(self.is_left(pos))
            }
            Direction::Right if pos.0 < self.rows && pos.1 < self.columns - 1 => {
                Some(self.is_right(pos))
            }
            Direction::Down if pos.0 < self.rows - 1 && pos.1 < self.columns => {
                Some(self.is_above(pos))
            }
            _ => None,
        }
    }

    fn move_in_direction(&self, current: &Position, dir: &Direction) -> Option<Position> {
        match dir {
            Direction::Up if current.0 > 0 => Some(Position(current.0 - 1, current.1)),
            Direction::Left if current.1 > 0 => Some(Position(current.0, current.1 - 1)),
            Direction::Right if current.1 < self.columns - 1 => {
                Some(Position(current.0, current.1 + 1))
            }
            Direction::Down if current.0 < self.rows - 1 => {
                Some(Position(current.0 + 1, current.1))
            }
            _ => None,
        }
    }

    fn walls_around(&self, pos: &Position) -> Vec<(Position, Direction)> {
        DIRECTIONS
            .iter()
            .filter_map(|dir| {
                if self.is(pos, dir) == Some(true) {
                    return Some((*pos, *dir));
                }
                None
            })
            .collect()
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut above;
        let mut below = Vec::with_capacity(self.columns as usize + 1);

        below.push(UPPER_LEFT);
        below.append(&mut vec![HORIZONTAL; self.columns as usize - 1]);
        below.push(UPPER_RIGHT);

        for r in 0..(self.rows - 1) {
            above = below;
            below = vec![WallJunction::default(); self.columns as usize + 1];
            below[0] = VERTICAL;
            below[self.columns as usize] = VERTICAL;

            for c in 0..(self.columns - 1) {
                above[c as usize + 1].set_down(self.is_right(&Position(r, c)));
                below[c as usize + 1].set_up(self.is_right(&Position(r, c)));
            }

            for c in 0..self.columns {
                below[c as usize].set_right(self.is_above(&Position(r, c)));
                below[c as usize + 1].set_left(self.is_above(&Position(r, c)));
            }

            writeln!(
                f,
                "{}",
                above
                    .into_iter()
                    .map(|j| format!("{}", j))
                    .collect::<String>()
            )?;
        }

        above = below;
        below = vec![HORIZONTAL; self.columns as usize + 1];
        below[0] = LOWER_LEFT;
        below[self.columns as usize] = LOWER_RIGHT;
        for c in 0..(self.columns - 1) {
            above[c as usize + 1].set_down(self.is_right(&Position(self.rows - 1, c)));
            below[c as usize + 1].set_up(self.is_right(&Position(self.rows - 1, c)));
        }

        writeln!(
            f,
            "{}",
            above
                .into_iter()
                .map(|j| format!("{}", j))
                .collect::<String>()
        )?;
        write!(
            f,
            "{}",
            below
                .into_iter()
                .map(|j| format!("{}", j))
                .collect::<String>()
        )
    }
}
