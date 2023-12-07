// 50 mins for part 1, 1 hour for part 2

use std::collections::HashMap;

pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let mut hands = to_hands(content);
    hands.sort_by_cached_key(|x| x.0.sort_key());
    println!("PART 1: {}", calc_winnings(hands));
}


fn part2(content: &str) {
    let mut hands = to_hands(content);
    hands.sort_by_cached_key(|x| x.0.joker_sort_key());
    println!("PART 2: {}", calc_winnings(hands));
}


#[derive(Debug)]
struct Hand {
    cards: String
}

impl Hand {
    fn new(cards: String) -> Hand {
        Hand { cards }
    }

    fn sort_key(&self) -> (u32, u32, u32, u32, u32, u32) {
        let counts = self.to_counts();
        let score = self.to_score(&counts);
        self.to_key(score, 11)
    }

    fn joker_sort_key(&self) -> (u32, u32, u32, u32, u32, u32) {
        let mut counts = self.to_counts();
        let score = self.to_joker_score(&mut counts);
        self.to_key(score, 1)
    }

    fn to_counts(&self) -> HashMap<char, u32> {
        let mut counts = HashMap::new();
        for card in self.cards.chars() {
            counts.entry(card)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
        counts
    }

    fn to_joker_score(&self, counts: &mut HashMap<char, u32>) -> u32 {
        if !counts.contains_key(&'J') {
            return self.to_score(counts);  // no jokers
        } else if counts.len() == 1 {
            return 6;  // all jokers
        }
        let nj = counts.remove(&'J').unwrap();
        let mut vals: Vec<&u32> = counts.values().collect();
        vals.sort();
        vals.reverse();

        if *vals[0] + nj == 5 {
            return 6;  // can make 5 of a kind
        } else if *vals[0] + nj == 4 {
            return 5;  // can make 4 of a kind
        } else if vals == vec![&2, &2] {
            return 4;  // can make full house
        } else if (vals == vec![&2, &1, &1]) | (vals == vec![&1, &1, &1]) {
            return 3;  // can make 3 of a kind
        } else {
            return 1;  // can always make a pair
        }
    }

    fn to_score(&self, counts: &HashMap<char, u32>) -> u32 {
        let mut counts: Vec<&u32> = counts.values().collect();
        counts.sort();
        counts.reverse();

        if *counts[0] == 5 {
            return 6;  // 5 of a kind
        } else if *counts[0] == 4 {
            return 5;  // 4 of a kind
        } else if (*counts[0] == 3) & (*counts[1] == 2) {
            return 4;  // full house
        } else if *counts[0] == 3 {
            return 3;  // 3 of a kind
        } else if (*counts[0] == 2) & (*counts[1] == 2) {
            return 2;  // 2 pairs
        } else if *counts[0] == 2 {
            return 1;  // 1 pair
        } else {
            return 0;
        }
    }

    fn to_key(&self, score: u32, joker: u32) -> (u32, u32, u32, u32, u32, u32) {
        let v: Vec<u32> =
            self.cards.chars().map(
                |c| match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => joker,
                    'T' => 10,
                    _ => c.to_digit(10).unwrap(),
                }
            )
            .collect();
        (score, v[0], v[1], v[2], v[3], v[4])
    }
}


fn calc_winnings(hands: Vec<(Hand, usize)>) -> usize {
    hands.iter()
        .enumerate()
        .map(|x| (x.0 + 1) * x.1.1)
        .sum()
}


fn to_hands(content: &str) -> Vec<(Hand, usize)> {
    let mut hands: Vec<(Hand, usize)> = vec![];
    for line in content.split("\n").map(|line| line.trim()) {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let hand = Hand::new(String::from(parts[0]));
        let bid: usize = parts[1].parse().unwrap();
        hands.push((hand, bid))
    }
    hands
}