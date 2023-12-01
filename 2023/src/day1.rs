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
        let num: u32 = digits.first().unwrap() + digits.last().unwrap();
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
            for value in VALUES {
              if part[index..].starts_with(value.str1) | part[index..].starts_with(value.str2) {
                  if start < 0 { start = value.value };
                  end = value.value;
              }
            }
        }
        total += start * 10 + end;
    }
    println!("PART 2: {}", total);
}

struct Value {
    value: i32,
    str1: &'static str,
    str2: &'static str,
}

const VALUES: [Value; 10] = [
    Value { value: 0, str1: "0", str2: "zero" },
    Value { value: 1, str1: "1", str2: "one" },
    Value { value: 2, str1: "2", str2: "two" },
    Value { value: 3, str1: "3", str2: "three" },
    Value { value: 4, str1: "4", str2: "four" },
    Value { value: 5, str1: "5", str2: "five" },
    Value { value: 6, str1: "6", str2: "six" },
    Value { value: 7, str1: "7", str2: "seven" },
    Value { value: 8, str1: "8", str2: "eight" },
    Value { value: 9, str1: "9", str2: "nine" },
];