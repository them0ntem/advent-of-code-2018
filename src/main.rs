use std::env;
extern crate regex;
extern crate chrono;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_and_question = &args[1];

    match day_and_question.as_str() {
        "1.1" => day1::puzzle1(),
        "1.2" => day1::puzzle2(),
        "2.1" => day2::puzzle1(),
        "2.2" => day2::puzzle2(),
        "3.1" => day3::puzzle1(),
        "3.2" => day3::puzzle2(),
        "4.1" => day4::puzzle1(),
        "4.2" => day4::puzzle2(),
        _ => println!("Not matching format '[day][question]'"),
    }
}
