// https://adventofcode.com/2018/day/1
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn question1() {
    let filename = "input/1.1.txt";
    let file = File::open(filename).expect("There was a problem opening the file:");

    let mut drift = 0;

    for line in BufReader::new(file).lines() {
        let line = line.expect("Reading line from buffer failed");

        let frequency: i32 = line.parse().expect("There was a problem converting String to i32");

        drift += frequency;
    }

    println!("Frequency Drift: {}", drift);
}

pub fn question2() {
    let file_name = "./input/1.2.txt";

    let file = File::open(file_name).expect("failed to open file");
    let buf_reader = BufReader::new(file);

    let mut frequency_changes: Vec<i32> = vec![];
    for line in buf_reader.lines() {
        let line = line.expect("Reading line from buffer failed");

        frequency_changes.push(line.parse().unwrap())
    }


    let mut frequency_drifted: HashSet<i32> = HashSet::new();
    frequency_drifted.insert(0);

    let mut drift = 0;
    'outer: loop {
        for frequency_change in frequency_changes.iter() {
            drift += frequency_change;

            if frequency_drifted.contains(&drift) {
                print!("First frequency repeated twice: {}", drift);
                break 'outer;
            }
            frequency_drifted.insert(drift);
        }
    }
}