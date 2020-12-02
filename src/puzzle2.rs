extern crate regex;
use std::io::{Cursor, BufRead};
use regex::Regex;
use std::str::FromStr;

fn main() {
    let cursor = Cursor::new(include_str!("inputs/puzzle2.txt"));
    let reg = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>[a-z]): (?P<pass>.*)").unwrap();
    let mut n_passes = 0;
    for line in cursor.lines() {
        let line = line.unwrap();
        let captures = reg.captures(line.as_str()).unwrap();
        let min = usize::from_str(&captures["min"]).unwrap();
        let max = usize::from_str(&captures["max"]).unwrap();
        let letter = char::from_str(&captures["letter"]).unwrap();
        let pass = &captures["pass"];
        let chars = pass.chars().filter(|x| x == &letter).collect::<String>().len();
        if chars >= min && chars <= max {
            n_passes += 1;
        }
    }

    println!("Puzzle 1: {}", n_passes);

    let cursor = Cursor::new(include_str!("inputs/puzzle2.txt"));
    let mut n_passes = 0;
    for line in cursor.lines() {
        let line = line.unwrap();
        let captures = reg.captures(line.as_str()).unwrap();
        let pos1 = usize::from_str(&captures["min"]).unwrap();
        let pos2 = usize::from_str(&captures["max"]).unwrap();
        let letter = char::from_str(&captures["letter"]).unwrap();
        let pass = &captures["pass"];
        let chars = pass.chars().enumerate().filter(|(i, x)| ((i+1) == pos1 || (i+1) == pos2) && x == &letter).count();
        if chars == 1 {
            n_passes += 1;
        }
    }

    println!("Puzzle 2: {}", n_passes);
}