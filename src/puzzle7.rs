extern crate regex;
use std::io::{Cursor, BufRead};
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct Bag(String);
struct BagContents(Vec<(i32, Bag)>);

fn main() {
    let mut bags = HashMap::<Bag, BagContents>::new();

    let cursor = Cursor::new(include_str!("inputs/puzzle7.txt"));
    let bag_re = Regex::new(r"((?:\w ?)*) bags contain ").unwrap();
    let content_re = Regex::new(r"(\d+|no) ((?:\w ?)*) bags?,?").unwrap();
    for line in cursor.lines() {
        let line = line.unwrap();

        let bag_match = bag_re.captures(line.as_str()).unwrap();
        let bag_name = bag_match.get(1).unwrap().as_str();
        let mut bag_contents = Vec::new();
        for content in content_re.captures_iter(line.as_str()) {
            let content_name = content.get(2).unwrap().as_str();
            // no other bags
            if content_name == "other" { break; }
            bag_contents.push((
                    i32::from_str(content.get(1).unwrap().as_str()).unwrap_or(0),
                    Bag(String::from(content.get(2).unwrap().as_str()))
            ))
        }

        bags.insert(Bag(String::from(bag_name)), BagContents(bag_contents));
    }

    fn contains_shiny_bags(bags: &HashMap<Bag, BagContents>, bag: &Bag, contains: &mut bool) -> bool {
        if bag.0 == "shiny gold" {
            *contains = true
        } else {
            for (_, content) in bags[bag].0.iter() {
                contains_shiny_bags(bags, content, contains);
            }
        }
        *contains
    };

    fn count_bags_recursively(bags: &HashMap<Bag, BagContents>, bag: &Bag, count: &mut i32, multiplier: i32) {
        for (cnt, content) in bags[bag].0.iter() {
            *count += cnt * multiplier;
            count_bags_recursively(bags, content, count, (*cnt * multiplier));
        }
    }

    let mut contain_count = 0;
    let mut inside_count = 0;
    for (bag, contents) in bags.iter() {
        if bag.0 == "shiny gold" {
            count_bags_recursively(&bags, bag, &mut inside_count, 1);
        } else {
            contain_count += if contains_shiny_bags(&bags, bag, &mut false) { 1 } else { 0 };
        }
    }

    println!("{}", contain_count);
    println!("{}", inside_count);
}