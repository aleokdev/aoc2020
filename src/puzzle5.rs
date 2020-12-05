extern crate regex;
use std::io::{Cursor, BufRead};
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::str::FromStr;
use std::collections::HashSet;

fn main() {
    let cursor = Cursor::new(include_str!("inputs/puzzle5.txt"));
    let mut set = {
        let mut set = HashSet::new();
        for column in 0..8 {
            for row in 1..127 {
                set.insert(row * 8 + column);
            }
        }
        set
    };
    let mut highest = 0;
    for line in cursor.lines() {
        let line = line.unwrap();
        let mut row = 0;
        for char in line.chars().collect::<Vec<_>>()[0..7].iter() {
            match char {
                'B' => row = (row << 1) + 1,
                _ => row = (row << 1),
            }
        }

        let mut column = 0;
        for char in line.chars().collect::<Vec<_>>()[7..].iter() {
            match char {
                'R' => column = (column << 1) + 1,
                _ => column = (column << 1),
            }
        }

        let seat_id = row * 8 + column;

        set.remove(&seat_id);
        if seat_id > highest {
            highest = seat_id;
        }
    }


    println!("Highest: {}", highest);

    for i in 0..127*8 {
        if !set.contains(&(i-1)) && set.contains(&(i)) && !set.contains(&(i+1)) {
            println!("My seat: {}", i);
        }
    }
}