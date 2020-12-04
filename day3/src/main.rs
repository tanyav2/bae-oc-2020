use itertools::zip;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_num_trees(filename: &str) -> u32 {
    let f = File::open(filename).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = String::new();
    let width = reader.read_line(&mut buf).unwrap();
    buf.clear();
    let mut coords = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let init = coords.clone();
    let mut total_trees = vec![0, 0, 0, 0, 0];
    let mut i = 1;

    while let Ok(len) = reader.read_line(&mut buf) {
        if len <= 0 {
            break;
        }
        // if i % 2 == 0 {
        //     if is_tree(buf.trim(), x % (width - 1)) {
        //         total_tree += 1;
        //     }
        //     x += 1;

        // }
        i += 1;
        for (idx, (x, y)) in coords.iter().enumerate() {
            if i % y == 0 {
                if is_tree(buf.trim(), x % (width - 1)) {
                    total_trees[idx] += 1;
                }
            }
        }
        coords.iter().map(|)
        // let mut xs_new = Vec::new();
        // for (i, x) in xs.iter().enumerate() {
        //     if is_tree(buf.trim(), x % (width-1)) {
        //         total_trees[i] += 1;
        //     }
        //     xs_new.push(x + init[i]);
        // }
        // xs = xs_new;
        buf.clear();
    }
    total_trees.iter().fold(1, |acc, x| acc * x)
}

fn is_tree(line: &str, x: usize) -> bool {
    line.as_bytes()[x] as char == '#' 
}

fn main() {
    println!(
        "num_trees: {:?}",
        get_num_trees("input.txt")
    );
}