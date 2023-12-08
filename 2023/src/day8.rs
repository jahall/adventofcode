// 30 mins part 1, 45 mins part 2

use std::collections::{HashMap, HashSet};
use primes::factors;

pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let (mut moves, graph) = parse_content(content);
    let mut node = &String::from("AAA");
    let mut step = 0;
    loop {
        if *node == "ZZZ" { break }
        let dir = moves.next();
        let (left, right) = &graph.map[node];
        node = if *dir == 'L' { left } else { right };
        step += 1;
    }
    println!("PART 1: {}", step);
}


fn part2(content: &String) {
    let (mut moves, graph) = parse_content(content);
    let nodes: Vec<&String> =
        graph.map.keys()
        .filter(|k| k.chars().nth(2).unwrap() == 'A')
        .collect();
    let mut factset: HashSet<u64> = HashSet::new();
    for node in nodes {
        let mut node = &node.clone();
        let mut step: u64 = 0;
        loop {
            if node.chars().nth(2).unwrap() == 'Z' {
                break
            }
            let dir = moves.next();
            let (left, right) = &graph.map[node];
            node = if *dir == 'L' { left } else { right };
            step += 1;
        }
        factset.extend(factors(step).iter());
    }
    println!("PART 2: {}", factset.iter().product::<u64>());
}


#[derive(Debug)]
struct Moves {
    moves: Vec<char>,
    index: usize,
}

impl Moves {
    fn new(line: &str) -> Moves {
        Moves{ moves: line.chars().collect(), index: 0 }
    }

    fn next(&mut self) -> &char {
        let next = &self.moves[self.index];
        self.index += 1;
        if self.index >= self.moves.len() { self.index = 0 }
        next
    }
}


#[derive(Debug)]
struct Graph {
    map: HashMap<String, (String, String)>,
}

impl Graph {
    fn new(lines: Vec<&str>) -> Graph {
        Graph { map: HashMap::from_iter(
            lines.iter()
            .map(
                |x| (
                    String::from(&x[0..3]),
                    (String::from(&x[7..10]), String::from(&x[12..15]))
                )
            )
        ) }
    }
}


fn parse_content(content: &str) -> (Moves, Graph) {
    let lines: Vec<&str> = content.split("\n").map(|x| x.trim()).collect();
    (
        Moves::new(lines[0]),
        Graph::new(lines[2..].to_vec())
    )
}