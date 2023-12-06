// 40 mins for part 1 ...then an hour to figure out how to go beyond f64 for part 2!
use rug::{Float, Integer};


pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let (times, records) = parse_part1(content);
    let solution: Integer = 
        times.iter()
        .zip(records.iter())
        .map(|x| find_ways_to_beat(x.0, x.1))
        .product();
    println!("PART 1: {}", solution);
}


fn part2(content: &str) {
    let (time, record) = parse_part2(content);
    println!("PART 2: {}", find_ways_to_beat(&time, &record));
}


fn find_ways_to_beat(time: &i64, record: &i64) -> Integer {
    let time = Float::with_val(128, *time);
    let record = Float::with_val(128, *record);
    let (lower, upper) = solve_quadratic(
        Float::with_val(128, 1.0), -time, record);
    let mut lower_ceil = lower.clone().ceil().to_integer().unwrap();
    let upper_ceil = upper.ceil().to_integer().unwrap();
    if (lower - &lower_ceil).abs() < 0.0001 {
        lower_ceil += 1;
    }
    upper_ceil - lower_ceil
}


fn solve_quadratic(a: Float, b: Float, c: Float) -> (Float, Float) {
    let b_ = b.clone();
    let s: Float = (b_ * &b - Float::with_val(128, 4.0) * &a * &c).sqrt();
    let den: Float = Float::with_val(128, 2.0) * &a;

    let b_ = b.clone();
    let lower = (-b_ - &s) / &den;
    let upper = (-b + &s) / &den;
    (lower, upper)
}


fn parse_part1(content: &str) -> (Vec<i64>, Vec<i64>) {
    let lines: Vec<Vec<i64>> =
        content.split("\n")
        .map(|line|
            line.trim()
            .split_whitespace()
            .collect::<Vec<&str>>()[1..]
            .iter()
            .map(|v| v.parse().unwrap())
            .collect()
        )
        .collect();
    (lines[0].clone(), lines[1].clone())
}


fn parse_part2(content: &str) -> (i64, i64) {
    let lines: Vec<i64> =
        content.split("\n")
        .map(|line|
            line.trim()
            .split_whitespace()
            .collect::<Vec<&str>>()[1..]
            .join("")
            .parse()
            .unwrap()
        )
        .collect();
    (lines[0], lines[1])
}