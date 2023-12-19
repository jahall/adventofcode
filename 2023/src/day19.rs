// 45 mins for part 1, maybe 1.5 hours for part 2
use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

pub fn run(content: String) {
    part1(&content);
    part2(&content, false);
}


fn part1(content: &str) {
    let (workflows, parts) = load_workflows_and_parts(content);
    let mut accepted = 0_usize;
    for part in parts {
        let mut name = "in";
        loop {
            let next = workflows[name].apply(&part);
            match next {
                "A" => { accepted += part.value(); break },
                "R" => break,
                _ => { name = next },
            }
        }
    }
    println!("PART 1: {}", accepted);
}


fn part2(content: &str, verbose: bool) {
    let (workflows, _) = load_workflows_and_parts(content);
    let mut queue: VecDeque<(_, &str)> = VecDeque::new();
    let start = PartRange::new((1, 4000), (1, 4000), (1, 4000), (1, 4000));
    let mut n_combinations = 0_usize;
    queue.push_back((start, "in"));
    while let Some((range, name)) = queue.pop_front() {
        if verbose { println!("{}\t{}", name, range.to_string()); }
        match name {
            "A" => { n_combinations += range.n_combinations() },
            "R" => (),
            _ => {
                for result in workflows[name].apply_to_range(&range) {
                    if verbose { println!(" -> {}\t{}", result.1, result.0.to_string()); }
                    queue.push_back(result);
                }
            },
        }
    }
    println!("PART 2: {}", n_combinations);
}


#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn from_string(line: &str) -> Workflow {
        let name = line.split("{").next().unwrap();
        Workflow {
            name: String::from(name),
            rules: line[name.len() + 1..line.len() - 1]
                .split(",")
                .map(|r| Rule::from_string(r))
                .collect(),
        }
    }

    /// Apply the workflow to this part
    fn apply(&self, part: &Part) -> &str {
        for rule in self.rules.iter() {
            if rule.is_valid(part) {
                return &rule.destination;
            }
        }
        ""
    }

    /// Apply the workflow to a range of parts
    fn apply_to_range(&self, start: &PartRange) -> Vec<(PartRange, &str)> {
        let mut ranges = vec![(*start, "")];
        for rule in self.rules.iter() {
            let mut next: Vec<(_, &str)> = vec![];
            for (range, attr) in ranges.iter() {
                if *attr == "" {
                    let (passed, failed) = rule.apply_to_range(range);
                    if let Some(passed) = passed { next.push((passed, &rule.destination)); }
                    if let Some(failed) = failed { next.push((failed, "")); }
                } else {
                    next.push((*range, attr))
                }
            }
            ranges = next;
        }
        ranges
    }
}


#[derive(Debug, Clone)]
struct Rule {
    destination: String,
    category: char,
    condition: char,
    value: usize,
}

impl Rule {
    fn from_string(part: &str) -> Rule {
        let bits = part.split(":").collect_vec();
        if bits.len() == 1 {
            Rule{
                destination: String::from(bits[0]),
                category: '.',
                condition: '.',
                value: 0,
            }
        } else {
            Rule{
                destination: String::from(bits[1]),
                category: bits[0].chars().nth(0).unwrap(),
                condition: bits[0].chars().nth(1).unwrap(),
                value: bits[0][2..].parse().unwrap(),
            }
        }
    }

    // Does this part pass the rule?
    fn is_valid(&self, part: &Part) -> bool {
        match self.condition {
            '<' => part.get(self.category) < self.value,
            '>' => part.get(self.category) > self.value,
            _ => true,
        }
    }

    /// Return ranges which pass and fail the rule
    fn apply_to_range(&self, range: &PartRange) -> (Option<PartRange>, Option<PartRange>) {
        let (low, high) = range.get(self.category);
        if ((self.condition != '<') && (self.condition != '>')) ||
            ((self.condition == '<') && (high < self.value)) ||
            ((self.condition == '>') && (low > self.value)) {
            (Some(*range), None)

        } else if self.condition == '<' {
            let split = range.split(self.category, self.value);
            (Some(split[0]), Some(split[1]))

        } else {
            let split = range.split(self.category, self.value + 1);
            (Some(split[1]), Some(split[0]))
        }
    }
}


#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new(x: usize, m: usize, a: usize, s: usize) -> Part {
        Part{ x, m, a, s }
    }

    fn value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    fn get(&self, attr: char) -> usize {
        match attr {'x' => self.x, 'm' => self.m, 'a' => self.a, _ => self.s}
    }
}


#[derive(Debug, Clone, Copy)]
struct PartRange {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl PartRange {
    fn new(
        x: (usize, usize),  // inclusive
        m: (usize, usize),
        a: (usize, usize),
        s: (usize, usize),
    ) -> PartRange {
        PartRange{ x, m, a, s }
    }

    fn n_combinations(&self) -> usize {
        let nx = self.x.1 - self.x.0 + 1;
        let nm = self.m.1 - self.m.0 + 1;
        let na = self.a.1 - self.a.0 + 1;
        let ns = self.s.1 - self.s.0 + 1;
        nx * nm * na * ns
    }

    fn get(&self, attr: char) -> (usize, usize) {
        match attr {'x' => self.x, 'm' => self.m, 'a' => self.a, _ => self.s}
    }

    /// Split into two ranges based on split 1 < value and split 2 >= value
    fn split(&self, attr: char, value: usize) -> Vec<PartRange> {
        let (low, high) = self.get(attr);
        if (value <= low) || (value > high) {
            vec![*self]
        } else {
            let (mut lower, mut upper) = (self.clone(), self.clone());
            match attr {
                'x' => { lower.x = (self.x.0, value - 1); upper.x = (value, self.x.1) },
                'm' => { lower.m = (self.m.0, value - 1); upper.m = (value, self.m.1) },
                'a' => { lower.a = (self.a.0, value - 1); upper.a = (value, self.a.1) },
                's' => { lower.s = (self.s.0, value - 1); upper.s = (value, self.s.1) },
                _ => ()
            }
            vec![lower, upper]
        }
    }

    fn to_string(&self) -> String {
        format!(
            "[x({}, {}) m({} {}) a({} {}) s({} {})]",
            self.x.0, self.x.1,
            self.m.0, self.m.1,
            self.a.0, self.a.1,
            self.s.0, self.s.1,
        )
    }
}


fn load_workflows_and_parts(content: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let mut workflows = HashMap::new();
    let mut parts = vec![];
    let mut passed_cutoff = false;
    for line in content.split("\n") {
        if line.is_empty() { 
            passed_cutoff = true;
            continue;
        }
        if !passed_cutoff {
            let workflow = Workflow::from_string(line);
            workflows.insert(workflow.name.clone(), workflow);
        } else {
            let vals: Vec<usize> = line[1..line.len() - 1]
                .split(",")
                .map(|x| x[2..].parse().unwrap())
                .collect_vec();
            parts.push(Part::new(vals[0], vals[1], vals[2], vals[3]));
        }
    }
    (workflows, parts)
}