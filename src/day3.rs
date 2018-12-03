// https://adventofcode.com/2018/day/3
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::collections::HashMap;

struct Claim {
    id: u16,
    left_edge: u16,
    top_edge: u16,
    width: u16,
    height: u16,
}

pub fn puzzle1() {
    let filename = "input/3.1.txt";
    let file = File::open(filename).expect("There was a problem opening the file:");

    let re = Regex::new(r"#(\d*)\s@\s(\d*),(\d*):\s(\d*)x(\d*)").unwrap();
    let mut claim_matrix: HashMap<(u16, u16), u16> = HashMap::new();

    for line in BufReader::new(file).lines() {
        let line: String = line.expect("Reading line from buffer failed").parse().unwrap();


        for cap in re.captures_iter(line.as_str()) {
            let left_edge: u16 = cap[2].parse().expect("not a u16");
            let top_edge: u16 = cap[3].parse().expect("not a u16");
            let width: u16 = cap[4].parse().expect("not a u16");
            let height: u16 = cap[5].parse().expect("not a u16");
            for x in left_edge..left_edge + width {
                for y in top_edge..top_edge + height {
                    *claim_matrix.entry((x, y)).or_default() += 1;
                }
            }
        }
    }

    let count = claim_matrix.values().filter(|&&count| count > 1).count();
    print!("Square inches of fabric are within two or more claims: {}", count);
}

pub fn puzzle2() {
    let filename = "input/3.2.txt";
    let file = File::open(filename).expect("There was a problem opening the file:");
    let re = Regex::new(r"#(\d*)\s@\s(\d*),(\d*):\s(\d*)x(\d*)").unwrap();

    let mut claim_matrix: HashMap<(u16, u16), u16> = HashMap::new();
    let mut claims: Vec<Claim> = vec![];
    for line in BufReader::new(file).lines() {
        let line: String = line.expect("Reading line from buffer failed").parse().unwrap();

        for cap in re.captures_iter(line.as_str()) {
            let id: u16 = cap[1].parse().expect("not a u16");
            let left_edge: u16 = cap[2].parse().expect("not a u16");
            let top_edge: u16 = cap[3].parse().expect("not a u16");
            let width: u16 = cap[4].parse().expect("not a u16");
            let height: u16 = cap[5].parse().expect("not a u16");
            for x in left_edge..left_edge + width {
                for y in top_edge..top_edge + height {
                    *claim_matrix.entry((x, y)).or_default() += 1;
                }
            }

            claims.push(Claim {
                id,
                left_edge,
                top_edge,
                width,
                height,
            })
        }
    }

    for claim in claims {
        let mut flag = true;
        'outer: for x in claim.left_edge..claim.left_edge + claim.width {
            for y in claim.top_edge..claim.top_edge + claim.height {
                if claim_matrix[&(x, y)] > 1 {
                    flag = false;
                    break 'outer;
                }
            }
        }

        if flag {
            println!("Only claim that doesn't overlap: {}", claim.id)
        }
    }
}