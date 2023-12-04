// 25 mins for part 1, 35 for part 2

use std::collections::HashSet;

pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let cards = content.split("\n").map(|line| line.trim());
    let mut total = 0;
    for card in cards {
        let (winning, mine) = parse_card(card);
        let matching = count_matching(&winning, &mine);
        let score = calc_score(&matching);
        total += score;
    }
    println!("PART 1: {}", total);
}


fn part2(content: &str) {
    let cards: Vec<(Vec<u32>, Vec<u32>)> =
        content.split("\n")
        .map(parse_card)
        .collect();
    let mut counts: Vec<u32> = vec![1; cards.len()];
    for (i, (winning, mine)) in cards.iter().enumerate() {
        let n: usize = count_matching(winning, mine).try_into().unwrap();
        for j in (i + 1)..=(i + n) {
            counts[j] += counts[i];
        }
    }
    println!("PART 2: {}", counts.iter().sum::<u32>());
}


fn count_matching(winning: &Vec<u32>, mine: &Vec<u32>) -> u32 {
    let winning: HashSet<&u32> = HashSet::from_iter(winning);    
    mine.iter()
        .filter(|v| winning.contains(*v))
        .collect::<Vec<&u32>>()
        .len()
        .try_into()
        .unwrap()
}


fn calc_score(matching: &u32) -> u32 {
    let base: u32 = 2;
    if *matching == 0 { 0 } else { base.pow(*matching - 1) }
}


fn parse_card(line: &str) -> (Vec<u32>, Vec<u32>) {
    let hands: &str = line.trim().split(": ").last().unwrap();
    let hands: Vec<&str> = hands.split(" | ").collect();
    let winning = to_numbers(hands[0]);
    let mine = to_numbers(hands[1]);
    (winning, mine)
}


fn to_numbers(line: &str) -> Vec<u32> {
    line.trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect::<Vec<u32>>()
}