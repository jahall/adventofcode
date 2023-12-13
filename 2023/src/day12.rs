// wow...so long, first went with combinations, then recursion...then finally figured out caching!

use std::collections::{HashSet, HashMap};
use itertools::Itertools;


pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let records = to_records(content);
    println!("PART 1: {}", solve(&records));
}


fn part2(content: &str) {
    let records: Vec<Record> = to_records(content)
        .iter()
        .map(|r| r.unfold())
        .collect();
    println!("PART 2: {}", solve(&records));
}


fn solve(records: &[Record]) -> usize {
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
        let mut cache = HashMap::new();
        self.find_arrangements(&self.springs, &self.groups, &mut cache)
    }

    fn find_arrangements(&self, springs: &[char], groups: &[usize], cache: &mut HashMap<String, usize>) -> usize {
        // first check the cache!!
        let key = self.to_key(springs, groups);
        if cache.contains_key(&key) {
            return cache[&key];
        }
        // if no further groups, must be no further known damaged
        if groups.is_empty() {
            return if self.any_known(springs, '#') { 0 } else { 1 };
        }
        // handle case where not enough springs left
        let group = groups[0];
        if springs.len() < group {
            return 0;
        }
        let mut counts = 0usize;
        let stop = springs.len() - group + 1;
        for (i, c) in springs[..stop].iter().enumerate() {

            // ignore operational springs
            if *c == '.' {
                continue;
            } else {
                let part = &springs[i..i + group];
                if (part.len() == group) & !self.any_known(&part, '.') {

                    // last group and no known damaged after this point
                    if (groups.len() == 1) & !self.any_known(&springs[i + group..], '#') {
                        counts += 1;
                    }

                    // handle remaining groups
                    else if springs.len() > i + group + 1 {
                        if springs[i + group] != '#' {
                            counts += self.find_arrangements(
                                &springs[i + group + 1..], 
                                &groups[1..],
                                cache,
                            )
                        }
                    }
                }
                // can't proceed past a known damaged spring
                if *c == '#' {
                    break;
                }
            }
        }
        cache.insert(key, counts);
        counts
    }

    fn any_known(&self, springs: &[char], type_: char) -> bool {
        springs.iter().any(|&c| c == type_)
    }

    fn to_key(&self, springs: &[char], groups: &[usize]) -> String {
        let mut key = springs.iter().join("");
        let groups = groups.iter().map(|i| i.to_string()).join(",");
        key.push_str(&groups);
        key
    }

    // SLOW CODE I STARTED WITH
    fn _arrangements_slow(&self) -> usize {
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
                    if self._is_valid(poss) { 1usize } else { 0usize }
                }
            )
            .sum()
    }

    fn _is_valid(&self, poss: Vec<char>) -> bool {
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