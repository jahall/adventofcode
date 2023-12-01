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