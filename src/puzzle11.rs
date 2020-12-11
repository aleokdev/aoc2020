extern crate regex;
use std::fmt::Write;
use std::io::{Cursor, BufRead};
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::str::FromStr;
use std::fmt::Display;
use std::io::{stdin, Read};

#[derive(Eq, PartialEq, Clone, Copy)]
enum Seat {
    Occupied,
    Unoccupied,
    None
}

impl Display for Seat {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        fmt.write_char(
            match &self {
                Seat::None => '.',
                Seat::Unoccupied => 'L',
                Seat::Occupied => '#'
            }
        )
    }
}

fn process(seats: &Vec<Seat>, w: usize, h: usize) -> (usize, Vec<Seat>) {
    let mut new_seats = seats.clone();
    let mut changed_seats = 0;
    let n_occupied = |x: i32,y: i32| -> usize {
        if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 { 0 }
        else { usize::from(seats.get((x + y * w as i32) as usize).unwrap_or(&Seat::None) == &Seat::Occupied) }
    };
    for i in 0..w as i32 {
        for j in 0..h as i32 {
            let seat = seats[(i + j * w as i32) as usize];
            if seat == Seat::None { continue; }
            let adjacent =
                n_occupied(i-1, j-1) + n_occupied(i, j-1) + n_occupied(i+1, j-1) +
                n_occupied(i-1, j) + n_occupied(i+1, j) +
                n_occupied(i-1, j+1) + n_occupied(i, j+1) + n_occupied(i+1, j+1);
            if adjacent == 0 && seat == Seat::Unoccupied {
                new_seats[(i + j * w as i32) as usize] = Seat::Occupied;
                changed_seats += 1;
            } else if adjacent >= 4 && seat == Seat::Occupied {
                new_seats[(i + j * w as i32) as usize] = Seat::Unoccupied;
                changed_seats += 1;
            }
        }
    }
    (changed_seats, new_seats)
}

fn process_2(seats: &Vec<Seat>, w: usize, h: usize) -> (usize, Vec<Seat>) {
    let mut new_seats = seats.clone();
    let mut changed_seats = 0;
    let n_occupied = |x: i32,y: i32| -> usize {
        if x < 0 || y < 0 || x >= w as i32 || y >= h as i32 { 0 }
        else { usize::from(seats.get((x + y * w as i32) as usize).unwrap_or(&Seat::None) == &Seat::Occupied) }
    };
    for i in 0..w as i32 {
        for j in 0..h as i32 {
            let sight = |step_x: i32, step_y: i32| -> usize {
                let mut ix = i+step_x;
                let mut iy = j+step_y;
                loop {
                    if ix < 0 || iy < 0 || ix >= w as i32 || iy >= h as i32 { return 0 }
                    else { match seats[(ix + iy * w as i32) as usize] { Seat::Occupied => return 1, Seat::Unoccupied => return 0, _ => () }}
                    ix += step_x;
                    iy += step_y;
                }
            };

            let seat = seats[(i + j * w as i32) as usize];
            if seat == Seat::None { continue; }
            let adjacent =
                sight(-1, -1) + sight(-1, 0) + sight(-1, 1) +
                sight(0, -1) + sight(0, 1) +
                sight(1, -1) + sight(1, 0) + sight(1, 1);

            if adjacent == 0 && seat == Seat::Unoccupied {
                new_seats[(i + j * w as i32) as usize] = Seat::Occupied;
                changed_seats += 1;
            } else if adjacent >= 5 && seat == Seat::Occupied {
                new_seats[(i + j * w as i32) as usize] = Seat::Unoccupied;
                changed_seats += 1;
            }
        }
    }
    (changed_seats, new_seats)
}

fn main() {
    let cursor = Cursor::new(include_str!("inputs/puzzle11.txt"));
    let mut seats = Vec::new();
    let mut seats_h = 0;
    let mut seats_w = 0;
    for line in cursor.lines() {
        let line = line.unwrap();
        seats_w = line.len();
        for ch in line.chars() {
            seats.push(
                match ch {
                    '.' => Seat::None,
                    'L' => Seat::Unoccupied,
                    '#' => Seat::Occupied,
                    _ => unreachable!()
                }
            )
        }
        seats_h += 1;
    }

    fn debug(seats: &Vec<Seat>, seats_h: usize, seats_w: usize) {
        for j in 0..seats_h {
                for i in 0..seats_w {
                        print!("{}", seats[i + j * seats_w]);
                }
            println!();
        }
    }

    debug(&seats, seats_w, seats_h);

    {
        let mut seats = seats.clone();
        loop {
            let (changes, new_seats) = process(&seats, seats_w, seats_h);
            match changes {
                0 => break,
                _ => seats = new_seats
            }
        }
    
        println!("Occupied seats: {}", seats.iter().filter(|&&x| x == Seat::Occupied).count());
    }

    {
        let mut seats = seats.clone();
        loop {
            let (changes, new_seats) = process_2(&seats, seats_w, seats_h);
            match changes {
                0 => break,
                _ => seats = new_seats
            }
        }
    
        println!("Occupied seats: {}", seats.iter().filter(|&&x| x == Seat::Occupied).count());
    }
}