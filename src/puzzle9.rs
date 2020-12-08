extern crate regex;
use std::io::{Cursor, BufRead};
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::str::FromStr;
use std::collections::HashSet;

#[macro_use] mod instructions;
use instructions::*;

fn main() {
    let mut instructions = read_instructions!("inputs/puzzle9.txt");
}