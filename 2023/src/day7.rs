use std::collections::HashMap;

pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let mut hands = to_hands(content);
    hands.sort_by_key(|x| x.0.sort_key());
    let egg: Vec<&String> = hands.iter().map(|v| &v.0.cards).collect();
    dbg!(egg);
    let winnings: usize = hands.iter()
        .enumerate()
        .map(|x| (x.0 + 1) * x.1.1)
        .sum();
    println!("PART 1: {}", winnings);
}


fn part2(_content: &str) {
    println!("PART 2: {}", -1);
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
        let mut counts: HashMap<char, u32> = HashMap::new();
        for card in self.cards.chars() {
            counts.entry(card)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
        let mut counts: Vec<&u32> = counts.values().collect();
        counts.sort();
        counts.reverse();

        let mut kind = 0;
        if *counts[0] == 5 {
            kind = 6  // 5 of a kind
        } else if *counts[0] == 4 {
            kind = 5   // 4 of a kind
        } else if (*counts[0] == 3) & (*counts[1] == 2) {
            kind = 4  // full house
        } else if *counts[0] == 3 {
            kind = 3  // 3 of a kind
        } else if (*counts[0] == 2) & (*counts[1] == 2) {
            kind = 2  // 2 pairs
        } else if *counts[0] == 2 {
            kind = 1  // 1 pair
        }

        let values: Vec<u32> = 
            self.cards.chars().map(
                |c| match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => c.try_into().unwrap(),
                }
            )
            .collect();

        (kind, values[0], values[1], values[2], values[3], values[4])
    }
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