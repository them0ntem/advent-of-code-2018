// https://adventofcode.com/2018/day/4
use std::fs::File;
use std::io::{BufRead, BufReader};
use chrono::prelude::*;
use regex::Regex;
use std::collections::HashMap;

pub fn puzzle1() {
    let filename = "input/4.txt";
    let file = File::open(filename).expect("There was a problem opening the file:");

    let re = Regex::new(r"\[(\d*-\d*-\d* \d*:\d*)].*?(\w{5}) #?(\d{0,4})").unwrap();

    let mut current_guard: u32 = 0;
    let mut start: DateTime<Utc> = Utc::now();

    let mut times: HashMap<u32, i64> = HashMap::new();
    let mut guards: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    for line in BufReader::new(file).lines() {
        let line: String = line.expect("Reading line from buffer failed").parse().unwrap();
        for cap in re.captures_iter(line.as_str()) {
            let date = Utc.datetime_from_str(&cap[1], "%Y-%m-%d %H:%M")
                .expect("Unable to parse datetime");
            match &cap[2] {
                "Guard" => {
                    current_guard = cap[3].parse().expect("Unable to parse guard id");
                }
                "falls" => {
                    start = date;
                }
                "wakes" => {
                    let guard = guards.entry(current_guard).or_default();
                    for min in start.minute()..date.minute() {
                        *guard.entry(min).or_default() += 1;
                    }

                    *times.entry(current_guard).or_default() += date.signed_duration_since(start)
                        .num_seconds();
                }
                _ => println!("Not matching format"),
            }
        }
    }

    let (guard_id, _) = times.iter().max_by_key(|x| x.1).unwrap();
    let (minute, _) = guards.get(&guard_id).expect("").iter().max_by_key(|x| x.1).unwrap();

    print!("Guard you chose multiplied by the minute you chose: {}", minute * guard_id);
}


pub fn puzzle2() {
    let filename = "input/4.txt";
    let file = File::open(filename).expect("There was a problem opening the file:");

    let re = Regex::new(r"\[(\d*-\d*-\d* \d*:\d*)].*?(\w{5}) #?(\d{0,4})").unwrap();

    let mut current_guard: u32 = 0;
    let mut start: DateTime<Utc> = Utc::now();

    let mut times: HashMap<u32, i64> = HashMap::new();
    let mut guards: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    for line in BufReader::new(file).lines() {
        let line: String = line.expect("Reading line from buffer failed").parse().unwrap();
        for cap in re.captures_iter(line.as_str()) {
            let date = Utc.datetime_from_str(&cap[1], "%Y-%m-%d %H:%M")
                .expect("Unable to parse datetime");
            match &cap[2] {
                "Guard" => {
                    current_guard = cap[3].parse().expect("Unable to parse guard id");
                }
                "falls" => {
                    start = date;
                }
                "wakes" => {
                    let guard = guards.entry(current_guard).or_default();
                    for min in start.minute()..date.minute() {
                        *guard.entry(min).or_default() += 1;
                    }

                    *times.entry(current_guard).or_default() += date.signed_duration_since(start)
                        .num_seconds();
                }
                _ => println!("Not matching format"),
            }
        }
    }

    let mut guard_id: u32 = 0;
    let mut max_minute: u32 = 0;
    let mut max_count: u32 = 0;

    for guard in guards.iter() {
        for time in  guard.1.iter() {
            if *time.1 > max_count {
                guard_id = *guard.0;
                max_minute = *time.0;
                max_count = *time.1;
            }
        }
    };

    println!("Guard you chose multiplied by the minute you chose: {}", guard_id*max_minute);
}