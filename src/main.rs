use std::env;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_and_question = &args[1];

    match day_and_question.as_str() {
        "1.1" => day1::question1(),
        "1.2" => day1::question2(),
        _ => println!("Not matching format '[day][question]'"),
    }
}
