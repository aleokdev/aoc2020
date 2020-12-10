extern crate regex;
use std::io::{Cursor, BufRead};
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::str::FromStr;

fn main() {
    let cursor = Cursor::new(include_str!("inputs/puzzle10.txt"));
    let mut adapters = Vec::new();
    for line in cursor.lines() {
        let line = line.unwrap();
        adapters.push(usize::from_str(line.as_str()).unwrap());
    }

    adapters.sort();
    adapters.insert(0, 0);
    let mut diff1 = 0;
    let mut diff3 = 1;
    let mut last_adapter = 0;
    for &adapter in adapters.iter() {
        match adapter - last_adapter {
            0 => (),
            1 => diff1 += 1,
            3 => diff3 += 1,
            _ => unreachable!(),
        }
        last_adapter = adapter;
    }
    println!("1 diff: {}, 3 diff: {}. Mult: {}", diff1, diff3, diff1 * diff3);

    let mut arrangements = 1;
    let mut paths = Vec::new();
    paths.resize(adapters.len(), 0usize);
    paths[0] = 1usize;
    for i in 0..adapters.len() {
        println!("{}: {}", i, paths[i]);
        if let Some(&a) = adapters.get(i+1) {
            if a <= adapters[i] + 3 {
                *paths.get_mut(i+1).unwrap() += paths[i];
            }
        }
        if let Some(&a) = adapters.get(i+2) {
            if a <= adapters[i] + 3 {
                *paths.get_mut(i+2).unwrap() += paths[i];
            }
        }
        if let Some(&a) = adapters.get(i+3) {
            if a <= adapters[i] + 3 {
                *paths.get_mut(i+3).unwrap() += paths[i];
            }
        }
    }
    println!("{}", paths.last().unwrap());
}