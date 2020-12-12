extern crate regex;
#[allow(unused_imports)]
use regex::Regex;
use std::io::{BufRead, Cursor};
use std::ops::{AddAssign, Mul};
#[allow(unused_imports)]
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Pos {
    x: i32,
    y: i32,
}
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn right(&self) -> Dir {
        match &self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }
    fn left(&self) -> Dir {
        match &self {
            Dir::North => Dir::West,
            Dir::East => Dir::North,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
        }
    }
    fn forward(&self) -> Pos {
        match &self {
            Dir::North => Pos { x: 0, y: 1 },
            Dir::East => Pos { x: 1, y: 0 },
            Dir::South => Pos { x: 0, y: -1 },
            Dir::West => Pos { x: -1, y: 0 },
        }
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Mul<i32> for Pos {
    type Output = Self;

    fn mul(self, other: i32) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Pos {
    fn manhattan_distance(self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn rotate_right(self) -> Self {
        Pos {
            x: self.y,
            y: -self.x,
        }
    }

    fn rotate_left(self) -> Self {
        Pos {
            x: -self.y,
            y: self.x,
        }
    }
}

fn main() {
    let cursor = Cursor::new(include_str!("inputs/puzzle12.txt"));
    let instructions = cursor.lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let mut pos = Pos { x: 0, y: 0 };
    let mut dir = Dir::East;
    for line in instructions.iter() {
        let instruction = line.chars().nth(0).unwrap();
        let n = i32::from_str(&line[1..]).unwrap();
        pos += match line.chars().nth(0).unwrap() {
            'N' => Dir::North.forward() * n,
            'E' => Dir::East.forward() * n,
            'S' => Dir::South.forward() * n,
            'W' => Dir::West.forward() * n,
            'F' => dir.forward() * n,
            'L' => {
                for _ in 0..n / 90 {
                    dir = dir.left();
                }
                Pos { x: 0, y: 0 }
            }
            'R' => {
                for _ in 0..n / 90 {
                    dir = dir.right();
                }
                Pos { x: 0, y: 0 }
            }
            _ => unreachable!(),
        }
    }

    println!("{:?} -> {}", pos, pos.manhattan_distance());

    let mut waypoint = Pos { x: 10, y: 1 };
    let mut pos = Pos { x: 0, y: 0 };
    for line in instructions.iter() {
        let instruction = line.chars().nth(0).unwrap();
        let n = i32::from_str(&line[1..]).unwrap();
        match line.chars().nth(0).unwrap() {
            'N' => waypoint += Dir::North.forward() * n,
            'E' => waypoint += Dir::East.forward() * n,
            'S' => waypoint += Dir::South.forward() * n,
            'W' => waypoint += Dir::West.forward() * n,
            'F' => pos += waypoint * n,
            'L' => {
                for _ in 0..n / 90 {
                    waypoint = waypoint.rotate_left();
                }
            }
            'R' => {
                for _ in 0..n / 90 {
                    waypoint = waypoint.rotate_right();
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{:?} -> {}", pos, pos.manhattan_distance());
}
