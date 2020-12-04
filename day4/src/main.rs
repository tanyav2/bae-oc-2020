use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// lazy_static! {
//     static ref FILTERS: [|&str| -> bool; 7] =
//         [
//             |byr: &str| {
//                 let byr: u32 = byr.parse().unwrap();
//                 byr >= 1920 && byr <= 2002
//             },
//             |iyr: &str| {
//                 let iyr: u32 = iyr.parse().unwrap();
//                 iyr >= 2010 && iyr <= 2020
//             },
//             |eyr: &str| {
//                 let eyr: u32 = eyr.parse().unwrap();
//                 eyr >= 2020 && eyr <= 2030
//             },
//             |hgt: &str| {
//                 if hgt.ends_with("cm") {
//                     let num = hgt.strip_suffix("cm").unwrap();
//                     let n: u32 = num.parse().unwrap();
//                     n >= 150 && n <= 193
//                 } else if hgt.ends_with("in") {
//                     let num = hgt.strip_suffix("in").unwrap();
//                     let n: u32 = num.parse().unwrap();
//                     n >= 59 && n <= 76
//                 } else {
//                     false
//                 }
//             },
//             |hcl: &str| {
//                 let re = Regex::new(r"#[0-9a-f]{6}$").unwrap();
//                 re.is_match(hcl)
//             },
//             |ecl: &str| {
//                 let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
//                 1 == colors.iter().fold(0, |acc, x| acc + ((&ecl == x) as i32))
//             },
//             |pid: &str| {
//                 let re = Regex::new(r"^[0-9]{9}$").unwrap();
//                 re.is_match(pid)
//             },
//         ]
// }

fn num_valid_passports(filename: &str) -> u32 {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    let mut total_valid = 0;

    while let Ok(len) = reader.read_line(&mut buf) {
        if len <= 1 {
            if is_valid(&buf.trim().replace("\n", " ")) {
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

fn is_valid(line: &str) -> bool {
    let fields = vec!["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];
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
            if hgt.ends_with("cm") {
                let num = hgt.strip_suffix("cm").unwrap();
                let n: u32 = num.parse().unwrap();
                n >= 150 && n <= 193
            } else if hgt.ends_with("in") {
                let num = hgt.strip_suffix("in").unwrap();
                let n: u32 = num.parse().unwrap();
                n >= 59 && n <= 76
            } else {
                false
            }
        },
        |hcl: &str| {
            let re = Regex::new(r"#[0-9a-f]{6}$").unwrap();
            re.is_match(hcl)
        },
        |ecl: &str| {
            let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            1 == colors.iter().fold(0, |acc, x| acc + ((&ecl == x) as i32))
        },
        |pid: &str| {
            let re = Regex::new(r"^[0-9]{9}$").unwrap();
            re.is_match(pid)
        },
    ];

    if fields.iter().fold(true, |acc, x| acc && line.contains(x)) {
        let values = fields.iter().map(|x| {
            let postfix: Vec<&str> = line.split(x).collect();
            let newthing: Vec<&str> = postfix[1].split(" ").collect();
            newthing[0]
        });
        let iterator = values.zip(FILTERS.iter());
        return iterator.fold(true, |acc, (val, f)| acc && f(val));
    }
    false
}

fn main() {
    println!("num_valid_passports: {}", num_valid_passports("input.txt"));
}
