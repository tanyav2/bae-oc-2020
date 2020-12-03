use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Rule {
    pub min: usize,
    pub max: usize,
    pub letter: char,
}

fn parse_line(line: &str) -> Result<(String, Rule)> {
    lazy_static! {
        pub static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)$").unwrap();
    }
    if RE.is_match(line) {
        let mut captures = RE.captures_iter(line);
        let blocks = captures.next().unwrap();
        return Ok((
            blocks[4].to_string(),
            Rule {
                min: blocks[1].parse::<usize>()?,
                max: blocks[2].parse::<usize>()?,
                letter: blocks[3].parse::<char>()?,
            },
        ));
    }
    Err(anyhow!("Parsing failed"))
}

fn get_num_valid_passwords(filename: &str) -> u16 {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    let mut valid_num = 0;

    while let Ok(len) = reader.read_line(&mut buf) {
        if len <= 0 {
            break;
        }
        let (password, rule) = parse_line(buf.trim()).unwrap();
        if is_valid_pwd_pr2(&password, &rule) {
            valid_num += 1;
        }
        buf.clear();
    }
    valid_num
}

fn is_valid_pwd_pr2(password: &str, rule: &Rule) -> bool {
    let (first, second) = (
        password.as_bytes()[rule.min - 1] as char,
        password.as_bytes()[rule.max - 1] as char,
    );
    (first == rule.letter) ^ (second == rule.letter)
}

fn main() {
    println!(
        "num_valid_passwords: {}",
        get_num_valid_passwords("input.txt")
    );
}
