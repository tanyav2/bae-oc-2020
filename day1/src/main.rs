#![feature(map_into_keys_values)]
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const THREE_SUM: i32 = 2020;

fn create_map(filename: &str) -> Result<HashMap<i32, i32>> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut buf = String::new();

    let mut map = HashMap::new();

    while let Ok(len) = reader.read_line(&mut buf) {
        if len <= 0 {
            break;
        }
        let num: i32 = buf.trim().parse()?;
        map.insert(num, 1);
        buf.clear();
    }

    Ok(map)
}

fn find_sum_pair(map: &HashMap<i32, i32>, sum: i32) -> Result<(i32, i32)> {
    for (&k, _v) in map {
        if map.contains_key(&(sum - k)) {
            return Ok((k, sum - k))
        }
    }

    Err(anyhow!("No valid pair found"))
}

fn find_3sum(mut map: HashMap<i32, i32>) -> Result<(i32, i32, i32)> {
    let keys: Vec<i32> = map.clone().into_keys().collect();
    for k in keys {
        map.remove(&k).unwrap();
        let sum = THREE_SUM - k;
        if let Ok((x, y)) = find_sum_pair(&map, sum) {
            return Ok((x, y, k))
        }
    }

    Err(anyhow!("No valid 3sum found"))
}

fn main() {
    let map = create_map("input1.txt").unwrap();
    let (x, y, z) = find_3sum(map.clone()).unwrap();
    println!("x * y * z = {}", x * y * z);
}
