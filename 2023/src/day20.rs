// 2 hours for part 1 - maybe another hour for part 2

use std::{collections::{HashMap, VecDeque}, fmt::Debug};

use itertools::Itertools;

pub fn run(content: String) {
    part1(&content, false);
    part2(&content);
}


fn part1(content: &str, verbose: bool) {
    let mut modules = parse_content(content);
    let mut nlow = 0_usize;
    let mut nhigh = 0_usize;
    for i in 0..1000 {
        if verbose{ println!("********** {}", i); }
        let mut queue = VecDeque::new();
        queue.push_back((String::from("button"), String::from("broadcaster"), Pulse::Low));
        while let Some((source, dest, pulse)) = queue.pop_front() {
            if verbose{ println!("{} -{}-> {}", &source, pulse.to_string(), &dest); }
            match pulse {
                Pulse::Low => { nlow += 1 },
                Pulse::High => { nhigh += 1 },
            }
            if let Some(module) = modules.get_mut(&dest) {
                for (next, pulse) in module.pulse(&source, pulse) {
                    queue.push_back((dest.clone(), next, pulse));
                }
            }
        }
    }
    println!("PART 1: {}", nlow * nhigh);
}


/// The node "rx" gets a low signal when all inputs to conjunction "vr" are low
/// The four inputs to "vr" are "bm", "cl", "tn", "dr" - so assume they operate
/// on some fixed (prime-number) cycle
fn part2(content: &str) {
    let mut modules = parse_content(content);
    let mut cycle_lengths = HashMap::new();
    for node in ["bm", "cl", "tn", "dr"] {
        cycle_lengths.insert(String::from(node), usize::MAX);
    }
    let mut n_presses = 0_usize;
    loop {
        n_presses += 1;
        let mut queue = VecDeque::new();
        queue.push_back((String::from("button"), String::from("broadcaster"), Pulse::Low));
        while let Some((source, dest, pulse)) = queue.pop_front() {
            if let Some(module) = modules.get_mut(&dest) {
                if dest == "vr" {
                    module.update_cycle_lengths(n_presses, &mut cycle_lengths);
                    if cycle_lengths.values().all(|v| *v < usize::MAX) {
                        println!("PART 2: {}", cycle_lengths.values().product::<usize>());
                        return;
                    }
                }
                for (next, pulse) in module.pulse(&source, pulse) {
                    queue.push_back((dest.clone(), next, pulse));
                }
            }
        }
    }
    
}


/// Definition of a pulse
#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse { Low, High }

impl Pulse {
    fn to_string(&self) -> String {
        String::from(if *self == Pulse::Low { "low" } else { "high" })
    }
}


/// Definition of a state
#[derive(Debug, Clone, Copy, PartialEq)]
enum State { On, Off }

impl State {
    fn to_string(&self) -> String {
        String::from(if *self == State::On { "on" } else { "high" })
    }
}


/// Common functionality for a module
trait Module {
    fn name(&self) -> String;
    fn pulse(&mut self, from: &String, pulse: Pulse) -> Vec<(String, Pulse)>;
    fn to_string(&self) -> String;
    fn update_cycle_lengths(&self, _: usize, _: &mut HashMap<String, usize>) {}

    fn send(&self, dests: &Vec<String>, pulse: Pulse) -> Vec<(String, Pulse)> {
        dests.iter().map(|d| (d.clone(), pulse)).collect()
    }
}

impl Debug for dyn Module {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


/// The broadcast module
#[derive(Debug, Clone)]
struct Broadcaster {
    name: String,
    dests: Vec<String>
}

impl Broadcaster {
    fn new(name: &str, dests: Vec<String>) -> Broadcaster {
        Broadcaster{ name: String::from(name), dests }
    }
}

impl Module for Broadcaster {
    fn name(&self) -> String { self.name.clone() }

    /// Pass the pulse on to the destinations
    fn pulse(&mut self, _from: &String, pulse: Pulse) -> Vec<(String, Pulse)> {
        self.send(&self.dests, pulse)
    }

    fn to_string(&self) -> String {
        format!("{}->[{}]", self.name(), self.dests.join(","))
    }
}


#[derive(Debug, Clone)]
struct FlipFlop {
    name: String,
    dests: Vec<String>,
    state: State,
}

impl FlipFlop {
    fn new(name: &str, dests: Vec<String>) -> FlipFlop {
        FlipFlop{ name: String::from(name), dests, state: State::Off }
    }
}

impl Module for FlipFlop {
    fn name(&self) -> String { self.name.clone() }

    /// Flip flop if input pulse is low
    fn pulse(&mut self, _from: &String, pulse: Pulse) -> Vec<(String, Pulse)> {
        match (pulse, self.state) {
            (Pulse::Low, State::Off) => {
                self.state = State::On;
                self.send(&self.dests, Pulse::High)
            },
            (Pulse::Low, State::On) => {
                self.state = State::Off;
                self.send(&self.dests, Pulse::Low)
            },
            _ => { vec![] },
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{}->[{}]({})",
            self.name(),
            self.dests.join(","),
            self.state.to_string(),
        )
    }
}


/// The conjunction module
#[derive(Debug, Clone)]
struct Conjunction {
    name: String,
    dests: Vec<String>,
    memory: HashMap<String, Pulse>,
}

impl Conjunction {
    fn new(name: &str, dests: Vec<String>, inputs: &Vec<String>) -> Conjunction {
        Conjunction{
            name: String::from(name),
            dests,
            memory: HashMap::from_iter(inputs.iter().map(|i| (i.clone(), Pulse::Low))),
        }
    }
}

impl Module for Conjunction {
    fn name(&self) -> String { self.name.clone() }

    /// Flip flop if input pulse is low
    fn pulse(&mut self, from: &String, pulse: Pulse) -> Vec<(String, Pulse)> {
        self.memory.insert(from.clone(), pulse);
        let all_high = self.memory.values().all(|v| *v == Pulse::High);
        self.send(
            &self.dests,
            if all_high { Pulse::Low } else { Pulse::High },
        )
    }

    fn to_string(&self) -> String {
        let mem = self.memory
            .iter()
            .map(|x| format!("{}:{}", x.0, x.1.to_string()))
            .join(",");
        format!("{}->[{}]({})", self.name(), self.dests.join(","), mem)
    }

    /// Hacky solution for part 2
    fn update_cycle_lengths(&self, n_presses: usize, cycle_lengths: &mut HashMap<String, usize>) {
        for node in self.memory
            .iter()
            .filter(|kv| *kv.1 == Pulse::High)
            .map(|kv| kv.0) {
            if n_presses < cycle_lengths[node] {
                cycle_lengths.insert(node.clone(), n_presses);
            }
        }
    }
}


fn parse_content(content: &str) -> HashMap<String, Box<dyn Module>> {
    let mut inputs = HashMap::new();
    for line in content.split("\n") {
        let (source, dests) = parse_line(line);
        for dest in dests {
            inputs.entry(dest)
                .and_modify(|v: &mut Vec<String>| v.push(source.clone()))
                .or_insert(vec![source.clone()]);
        }
    }
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
    for line in content.split("\n") {
        let (name, dests) = parse_line(line);
        if line.starts_with('%') {
            let module = FlipFlop::new(&name, dests);
            modules.insert(module.name(), Box::new(module));
        } else if line.starts_with('&') {
            let (name, _) = parse_line(line);
            let module = Conjunction::new(&name, dests, &inputs[&name]);
            modules.insert(module.name(), Box::new(module));
        } else {
            let module = Broadcaster::new(&name, dests);
            modules.insert(module.name(), Box::new(module));
        }
    }
    modules
}


fn parse_line(line: &str) -> (String, Vec<String>) {
    let parts = line.split(" -> ").collect_vec();
    let mut name = String::from(parts[0]);
    if name.starts_with('%') || name.starts_with('&') {
        name = String::from(&name[1..]);
    }
    (
        name,
        parts[1]
            .split(", ")
            .map(|x| String::from(x))
            .collect()
    )
}