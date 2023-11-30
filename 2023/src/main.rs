use std::env;
use std::fs;
use std::path;

mod day1;


fn main() {
    let args: Vec<String> = env::args().collect();
    let data: Data = Data::new(&args);
    
    if data.day == "1" {
        day1::run(data.content);
    }
}

struct Data {
    day: String,
    content: String,
}

impl Data {
    fn new(args: &[String]) -> Data {
        // Absolutely hideous way of getting the relative path to the data dir!!
        let this_file = file!();
        let abspath = fs::canonicalize(&this_file).expect("Oops");
        let root_dir = abspath.parent()
            .expect("Oops")
            .parent()
            .expect("Oops");
        let data_dir = root_dir.join(path::Path::new("data"));

        let day = args[1].trim();
        let filename = format!("day{day}.txt");
        let filepath = data_dir.join(path::Path::new(&filename));

        let error_msg = format!("Can't find file!");
        let content = fs::read_to_string(filepath).expect(&error_msg);
        Data { day: day.to_string(), content }
    }
}