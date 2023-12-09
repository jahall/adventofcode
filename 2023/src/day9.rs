// 30 mins total

pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let histories = parse_content(content);
    let answer: i64 =
        histories.iter()
        .map(extrapolate)
        .sum();
    println!("PART 1: {}", answer);
}


fn part2(content: &str) {
    let histories = parse_content(content);
    let answer: i64 =
        histories.iter()
        .map(baxtrapolate)
        .sum();
    println!("PART 2: {}", answer);
}


fn extrapolate(values: &Vec<i64>) -> i64 {
    let mut values = values.clone();
    let mut prevs: Vec<i64> = vec![values[values.len() - 1].clone()];
    loop {
        values = to_diffs(&values);
        if all_zeros(&values) { break; }
        prevs.push(values[values.len() - 1].clone());
    }
    prevs.iter().sum()
}


fn baxtrapolate(values: &Vec<i64>) -> i64 {
    let mut values = values.clone();
    let mut firsts: Vec<i64> = vec![values[0].clone()];
    loop {
        values = to_diffs(&values);
        if all_zeros(&values) { break; }
        firsts.push(values[0].clone());
    }
    let mut forecast = 0;
    for v in firsts.iter().rev() {
        forecast = v - forecast;
    }
    forecast
}


fn to_diffs(values: &Vec<i64>) -> Vec<i64> {
    values[1..].iter()
    .enumerate()
    .map(
        |iv| iv.1 - values[iv.0]
    )
    .collect()
}


fn all_zeros(values: &Vec<i64>) -> bool {
    values.iter().map(|v| v.abs()).sum::<i64>() == 0
}


fn parse_content(content: &str) -> Vec<Vec<i64>> {
    content.split("\n")
    .map(
        |x| {
            x.trim()
            .split(" ")
            .map(
                |v| v.parse().unwrap()
            )
            .collect()
        }
    )
    .collect()
}