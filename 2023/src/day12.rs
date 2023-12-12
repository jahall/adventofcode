use std::collections::HashSet;
use itertools::Itertools;


pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let records = to_records(content);
    println!("PART 1: {}", solve(records));
}


fn part2(content: &str) {
    let records: Vec<Record> = to_records(content)
        .iter()
        .map(|r| r.unfold())
        .collect();
    println!("PART 2: {}", solve(records));
}


fn solve(records: Vec<Record>) -> usize {
    records.iter()
        .map(|r| r.arrangements())
        .sum()
}


#[derive(Debug, Clone)]
struct Record {
    springs: Vec<char>,
    groups: Vec<usize>,
}

impl Record {
    fn new(line: &str) -> Record {
        let parts: Vec<&str> = line.trim().split(" ").collect();
        Record{
            springs: parts[0].chars().collect(),
            groups: parts[1].split(",").map(|v| v.parse().unwrap()).collect(),
        }
    }

    fn unfold(&self) -> Record {
        let mut springs: Vec<char> = vec![];
        let mut groups: Vec<usize> = vec![];
        for i in 0..5 {
            springs.extend(&self.springs);
            groups.extend(&self.groups);
            if i < 4 { springs.push('?') }
        }
        Record{ springs, groups }
    }

    fn arrangements(&self) -> usize {
        // find indices of gaps
        let gaps: Vec<usize> = self.springs.iter()
            .enumerate()
            .filter(|x| *x.1 == '?')
            .map(|x| x.0)
            .collect();

        // find num remaining damaged
        let n_damaged_total: usize = self.groups.iter().sum();
        let n_damaged_known = self.springs.iter()
            .filter(|x| **x == '#')
            .count();
        let k = n_damaged_total - n_damaged_known;
        
        // iterate over possibilities
        gaps.into_iter()
            .combinations(k)
            .map(
                |combo| {
                    let combo: HashSet<usize> = HashSet::from_iter(combo);
                    let poss: Vec<char> = self.springs.iter()
                        .enumerate()
                        .map(|x|
                            if combo.contains(&x.0) { '#' }
                            else if *x.1 == '?' { '.' }
                            else { x.1.clone() }
                        )
                        .collect();
                    if self.is_valid(poss) { 1usize } else { 0usize }
                }
            )
            .sum()
    }

    fn is_valid(&self, poss: Vec<char>) -> bool {
        let mut buffer: usize = 0;
        let mut groups: Vec<usize> = vec![];
        for spring in poss {
            if spring == '#' {
                buffer += 1;
            } else {
                if buffer > 0 { groups.push(buffer.clone()) };
                buffer = 0;
            }
        }
        if buffer > 0 { groups.push(buffer.clone()) };
        groups == self.groups
    }
}


fn to_records(content: &str) -> Vec<Record> {
    content.split("\n").map(Record::new).collect()
}