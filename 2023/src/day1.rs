pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &String) {
    let parts = content.split("\n").map(|line| line.trim());
    let mut total: u32 = 0;
    for part in parts {
        let digits: Vec<u32> =
            part.chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        if digits.len() == 0 {
            continue;
        }
        let num: u32 = 10 * digits.first().unwrap() + digits.last().unwrap();
        total += num;
    }
    println!("PART 1: {}", total);
}


fn part2(content: &String) {
    let parts = content.split("\n").map(|line| line.trim());
    let mut total: i32 = 0;

    for part in parts {
        let mut start = -1;
        let mut end = -1;
        for index in 0..part.len() {
            for (num, name) in VALUES {
              if part[index..].starts_with(&num.to_string()) | part[index..].starts_with(name) {
                  if start < 0 { start = num };
                  end = num;
              }
            }
        }
        total += start * 10 + end;
    }
    println!("PART 2: {}", total);
}


const VALUES: [(i32, &'static str); 10] = [
    ( 0, "zero" ),
    ( 1, "one" ),
    ( 2, "two" ),
    ( 3, "three" ),
    ( 4, "four" ),
    ( 5, "five" ),
    ( 6, "six" ),
    ( 7, "seven" ),
    ( 8, "eight" ),
    ( 9, "nine" ),
];