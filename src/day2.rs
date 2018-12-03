// https://adventofcode.com/2018/day/2
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn puzzle1() {
    let filename = "input/2.1.txt";
    let file = File::open(filename).expect("There was a problem opening the file:");

    let (mut two_count, mut three_count) = (0, 0);

    let mut frequencies = [0u16; 256];
    for line in BufReader::new(file).lines() {
        let line = line.expect("Reading line from buffer failed");

        for frequency in frequencies.iter_mut() {
            *frequency = 0;
        }
        for character in line.as_bytes().iter().map(|&b| b as usize) {
            frequencies[character] = frequencies[character].saturating_add(1);
        }
        if frequencies.iter().any(|&frequency| frequency == 2) {
            two_count += 1;
        }

        if frequencies.iter().any(|&frequency| frequency == 3) {
            three_count += 1;
        }
    }

    println!("Checksum: {}", two_count * three_count)
}

pub fn puzzle2() {
    let filename = "input/2.2.txt";
    let file = File::open(filename).expect("There was a problem opening the file:");

    let mut ids: Vec<String> = vec![];
    for line in BufReader::new(file).lines() {
        let line: String = line.expect("Reading line from buffer failed").parse().unwrap();
        ids.push(line);
    }

    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            let id1: &str = &ids[i];
            let id2: &str = &ids[j];

            let mut diff_count = 0;
            for (id_ch1, id_ch2) in id1.chars().zip(id2.chars()) {
                if id_ch1 != id_ch2 {
                    diff_count += 1;
                    if diff_count > 1 {
                        break;
                    }
                }
            }

            if diff_count == 1 {
                let string_without_diff: String = id1.chars().zip(id2.chars())
                    .filter(|&(c1, c2)| c1 == c2)
                    .map(|(c, _)| c)
                    .collect();
                println!("Letters are common between the two correct box IDs: {}", string_without_diff);
            }
        }
    }
}