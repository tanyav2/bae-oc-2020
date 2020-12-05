use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

lazy_static! {
    static ref FIELDS: [&'static str; 7] = ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];
    static ref RE_HCL: Regex = Regex::new(r"#[0-9a-f]{6}$").unwrap();
    static ref RE_PID: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    static ref ECL: [&'static str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
}

fn num_valid_passports(filename: &str) -> u32 {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    let filters = [
        |byr: &str| {
            let byr: u32 = byr.parse().unwrap();
            byr >= 1920 && byr <= 2002
        },
        |iyr: &str| {
            let iyr: u32 = iyr.parse().unwrap();
            iyr >= 2010 && iyr <= 2020
        },
        |eyr: &str| {
            let eyr: u32 = eyr.parse().unwrap();
            eyr >= 2020 && eyr <= 2030
        },
        |hgt: &str| {
            if let Some(num) = hgt.strip_suffix("cm") {
                let n: u32 = num.parse().unwrap();
                n >= 150 && n <= 193
            } else if let Some(num) = hgt.strip_suffix("in") {
                let n: u32 = num.parse().unwrap();
                n >= 59 && n <= 76
            } else {
                false
            }
        },
        |hcl: &str| RE_HCL.is_match(hcl),
        |ecl: &str| 1 == ECL.iter().fold(0, |acc, x| acc + ((&ecl == x) as i32)),
        |pid: &str| RE_PID.is_match(pid),
    ];
    let mut total_valid = 0;

    while let Ok(len) = reader.read_line(&mut buf) {
        if len <= 1 {
            if is_valid(&buf.trim().replace("\n", " "), &filters) {
                total_valid += 1;
            }
            if len <= 0 {
                break;
            }
            buf.clear();
        }
    }
    total_valid
}

fn is_valid(line: &str, filters: &[impl Fn(&str) -> bool; 7]) -> bool {
    if FIELDS
        .iter()
        .fold(true, |acc, field| acc && line.contains(field))
    {
        let values = FIELDS.iter().map(|field| {
            let postfix = line.split(field).last().unwrap();
            postfix.split_whitespace().next().unwrap()
        });
        return values
            .zip(filters.iter())
            .fold(true, |acc, (val, f)| acc && f(val));
    }
    false
}

fn main() {
    println!("num_valid_passports: {}", num_valid_passports("input.txt"));
}
