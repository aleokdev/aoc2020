use std::str::FromStr;
use std::collections::HashSet;
use std::io::BufRead;
use std::io::Cursor;
extern crate regex;
use regex::Regex;

fn main() {
    let cursor = Cursor::new(include_str!("inputs/puzzle4.txt"));

    let reg = Regex::new(r"([^:]+):([^ ]+)").unwrap();

    fn required_fields() -> HashSet<&'static str> {
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].into_iter().collect::<HashSet<&'static str>>()
    }

    let mut fields = required_fields();
    let mut valid_passports = 0;
    for line in cursor.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            fields = required_fields();
        } else {
            for field in reg.find_iter(line.as_str()) {
                let f_str = field.as_str().trim();
                let captures = reg.captures(f_str).unwrap();
                let f_name = captures.get(1).unwrap().as_str();
                let f_val = captures.get(2).unwrap().as_str();

                let valid = match f_name {
                    "byr" => {
                        let byr = u32::from_str(f_val).unwrap();
                        byr >= 1920 && byr <= 2002
                    },
                    "iyr" => {
                        let iyr = u32::from_str(f_val).unwrap();
                        iyr >= 2010 && iyr <= 2020
                    },
                    "eyr" => {
                        let eyr = u32::from_str(f_val).unwrap();
                        eyr >= 2020 && eyr <= 2030
                    },
                    "hgt" => {
                        let reg2 = Regex::new(r"([0-9]+)(cm|in)").unwrap();
                        if let Some(captures) = reg2.captures(f_val) {
                            let hgt = u32::from_str(captures.get(1).unwrap().as_str()).unwrap();
                            if f_val.ends_with("cm") {
                                hgt >= 150 && hgt <= 193
                            } else {
                                hgt >= 59 && hgt <= 76
                            }
                        } else { false }

                    },
                    "hcl" => {
                        let reg2 = Regex::new(r"#[0-9a-f]{6}").unwrap();
                        reg2.is_match(f_val)
                    },
                    "ecl" => {
                        let reg2 = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
                        reg2.is_match(f_val)
                    },
                    "pid" => {
                        let reg2 = Regex::new(r"^[0-9]{9}$").unwrap();
                        reg2.is_match(f_val)
                    },
                    _ => false,
                };
                if valid { fields.retain(|x| x != &f_name); }
                if fields.len() == 0 {
                    fields = required_fields();
                    valid_passports = valid_passports + 1;
                }
            }
        }
    }

    println!("Valid passports: {}", valid_passports);
}