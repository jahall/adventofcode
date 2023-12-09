use std::env;
use std::fs;
use std::path;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
pub mod utils;


fn main() {
    let args: Vec<String> = env::args().collect();
    let data: Data = Data::new(&args);
    
    if (data.suffix == "1") | (data.suffix.starts_with("1_")) {
        day1::run(data.content);
    } else if (data.suffix == "2") | (data.suffix.starts_with("2_")) {
        day2::run(data.content);
    } else if (data.suffix == "3") | (data.suffix.starts_with("3_")) {
        day3::run(data.content);
    } else if (data.suffix == "4") | (data.suffix.starts_with("4_")) {
        day4::run(data.content);
    } else if (data.suffix == "5") | (data.suffix.starts_with("5_")) {
        day5::run(data.content);
    } else if (data.suffix == "6") | (data.suffix.starts_with("6_")) {
        day6::run(data.content);
    } else if (data.suffix == "7") | (data.suffix.starts_with("7_")) {
        day7::run(data.content);
    } else if (data.suffix == "8") | (data.suffix.starts_with("8_")) {
        day8::run(data.content);
    } else if (data.suffix == "9") | (data.suffix.starts_with("9_")) {
        day9::run(data.content);
    } else if (data.suffix == "10") | (data.suffix.starts_with("10_")) {
        day10::run(data.content);
    } else if (data.suffix == "11") | (data.suffix.starts_with("11_")) {
        day11::run(data.content);
    } else if (data.suffix == "12") | (data.suffix.starts_with("12_")) {
        day12::run(data.content);
    } else if (data.suffix == "13") | (data.suffix.starts_with("13_")) {
        day13::run(data.content);
    } else if (data.suffix == "14") | (data.suffix.starts_with("14_")) {
        day14::run(data.content);
    } else if (data.suffix == "15") | (data.suffix.starts_with("15_")) {
        day15::run(data.content);
    } else if (data.suffix == "16") | (data.suffix.starts_with("16_")) {
        day16::run(data.content);
    } else if (data.suffix == "17") | (data.suffix.starts_with("17_")) {
        day17::run(data.content);
    } else if (data.suffix == "18") | (data.suffix.starts_with("18_")) {
        day18::run(data.content);
    } else if (data.suffix == "19") | (data.suffix.starts_with("19_")) {
        day19::run(data.content);
    }
}

struct Data {
    suffix: String,
    content: String,
}

impl Data {
    fn new(args: &[String]) -> Data {
        // Absolutely hideous way of getting the relative path to the data dir!!
        let this_file = file!();
        let abspath = fs::canonicalize(&this_file).expect("Oops");
        let root_dir = abspath.parent().unwrap().parent().unwrap();
        let data_dir = root_dir.join(path::Path::new("data"));

        let suffix = args[1].trim();
        let filename = format!("day{suffix}.txt");
        let filepath = data_dir.join(path::Path::new(&filename));

        let error_msg = format!("Can't find file!");
        let content = fs::read_to_string(filepath).expect(&error_msg);
        Data { suffix: suffix.to_string(), content }
    }
}