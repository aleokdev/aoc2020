extern crate regex;
use std::io::{Cursor, BufRead};
use regex::Regex;
use std::str::FromStr;

fn main() {
    let cursor = Cursor::new(include_str!("inputs/puzzle3.txt"));
    let slopes = vec![(1,1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let lines = cursor.lines().collect::<Vec<_>>();
    let mut result: usize = 1;
    for (slope_x, slope_y) in slopes.iter() {
        let mut tx = 0;
        let mut trees = 0;
        for line in lines.iter().step_by(*slope_y as usize) {
            let line = line.as_ref().unwrap();

            if line.chars().nth(tx as usize).unwrap() == '#' {
                trees += 1;
            }

            tx += slope_x;
            tx %= line.len();
        }

        println!("{}", trees);
        result *= trees;
    }

    println!("{}", result);
}