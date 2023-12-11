use std::ops::{Add, Deref, Sub};

use super::utils::read_file;

// relative coordinates = [x - 1, y - 1 ]
const X: Delta = [0, 0];
const N: Delta = [0, -1];
const S: Delta = [0, 1];
const E: Delta = [1, 0];
const W: Delta = [-1, 0];

type Point = [usize; 2];
type Delta = [isize; 2];

#[derive(Debug, Copy, Clone)]
struct Coord(Point);

impl Deref for Coord {
    type Target = Point;
    fn deref(&self) -> &Point {
        &self.0
    }
}
impl Eq for Coord {}
impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self[0] == other[0] && self[1] == other[1]
    }
}
impl Add<Delta> for Coord {
    type Output = Self;
    fn add(self, rhs: Delta) -> Self::Output {
        Self([
            self[0].checked_add_signed(rhs[0]).unwrap_or_default(),
            self[1].checked_add_signed(rhs[1]).unwrap_or_default(),
        ])
    }
}

#[allow(clippy::cast_possible_wrap)]
impl Sub for Coord {
    type Output = Delta;
    fn sub(self, rhs: Self) -> Self::Output {
        [
            self[0] as isize - rhs[0] as isize,
            self[1] as isize - rhs[1] as isize,
        ]
    }
}

struct Maze {
    map: Vec<Vec<u8>>,
    start: Coord,
}
impl Maze {
    fn new(map: Vec<Vec<u8>>) -> Self {
        // get the initial point
        let start = Coord(
            map.iter()
                .enumerate()
                .find_map(|(y, l)| l.iter().position(|&s| s == b'S').map(|x| [x, y]))
                .expect("No starting point found"),
        );
        Self { map, start }
    }
    fn symbol(&self, pos: Coord) -> u8 {
        self.map[pos[1]][pos[0]]
    }
    fn iter(&self, dir: Delta) -> PipeWalk {
        let symbol = self.symbol(self.start + dir);
        let dir = match (dir, symbol) {
            (N, b'|' | b'7' | b'F')
            | (S, b'|' | b'J' | b'L')
            | (W, b'-' | b'F' | b'L')
            | (E, b'-' | b'7' | b'J') => dir,
            _ => X,
        };
        PipeWalk {
            plane: self,
            next: self.start + dir,
            prev: self.start,
            len: 1,
        }
    }
}

struct PipeWalk<'p> {
    plane: &'p Maze,
    next: Coord,
    prev: Coord,
    len: u32,
}
impl<'m> Iterator for PipeWalk<'m> {
    type Item = (Coord, u32);
    fn next(&mut self) -> Option<Self::Item> {
        let dir = self.plane.symbol(self.next);
        // getting this right was the tricky and tedious part,
        // had to use the right types used and double check
        let next = match (self.next - self.prev, dir) {
            (N, b'|') | (W, b'L') | (E, b'J') => self.next + N,
            (S, b'|') | (W, b'F') | (E, b'7') => self.next + S,
            (N, b'7') | (S, b'J') | (W, b'-') => self.next + W,
            (N, b'F') | (S, b'L') | (E, b'-') => self.next + E,
            _ => return None,
        };
        self.prev = self.next;
        self.next = next;
        self.len += 1;
        Some((self.next, self.len))
    }
}

pub(crate) fn dayten1(data: &str) -> Result<u32, String> {
    let lines = read_file(data).map_err(|_| "reading filed")?;
    let map: Vec<Vec<u8>> = lines
        .into_iter()
        .map(|l| l.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let maze = Maze::new(map);

    let mut start_1 = None;
    let mut start_2 = None;

    // find the starting points right after S
    for i in [N, S, E, W] {
        let mut iter = maze.iter(i);
        if iter.next().is_some() {
            if start_1.is_some() {
                start_2 = Some(iter);
                break;
            }
            start_1 = Some(iter);
        }
    }

    if let (Some(s1), Some(s2)) = (start_1, start_2) {
        for ((c1, len1), (c2, len2)) in s1.zip(s2) {
            if c1 == c2 {
                return Ok(len1.max(len2));
            }
        }
        Err("No loop found".to_string())
    } else {
        Err("No starting positions found".to_string())
    }
}
pub(crate) fn dayten2(data: &str) -> Result<u32, String> {
    return Ok(1);
}