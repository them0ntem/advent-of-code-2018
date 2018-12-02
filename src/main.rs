use std::env;

mod day1;
mod day2;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_and_question = &args[1];

    match day_and_question.as_str() {
        "1.1" => day1::question1(),
        "1.2" => day1::question2(),
        "2.1" => day2::question1(),
        "2.2" => day2::question2(),
        _ => println!("Not matching format '[day][question]'"),
    }
}
