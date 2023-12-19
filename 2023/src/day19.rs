// 45 mins for part 1
use std::collections::HashMap;

use itertools::Itertools;

pub fn run(content: String) {
    part1(&content);
    part2(&content);
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


fn part2(_content: &str) {
    println!("PART 2: {}", -1);
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