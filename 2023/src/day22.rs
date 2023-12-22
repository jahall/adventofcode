// 2 hours all in - takes a couple of mins to run part 2

use std::{fmt::Debug, collections::{HashSet, VecDeque, HashMap}};

use itertools::Itertools;


pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let bricks = load_bricks(content);
    let bricks = find_resting_positions(&bricks);
    // count bricks safe to distintegrate i.e. which aren't the only support for another
    let support_map = calc_support_map(&bricks);
    let count = bricks.iter()
        .filter(|b| is_safe_to_disintegrate(&support_map, b))
        .count();
    println!("PART 1: {}", count);
}


fn part2(content: &str) {
    let bricks = load_bricks(content);
    let bricks = find_resting_positions(&bricks);
    let mut count = 0;
    for brick in bricks.iter() {
        let mut support_map = calc_support_map(&bricks);
        support_map = remove_bricks(&support_map, &[brick]);
        loop {
            // find unsupported bricks
            let unsupported = support_map
                .iter()
                .filter(|(k, v)| v.is_empty() && !k.on_ground())
                .map(|(k, _)| k)
                .collect_vec();
            // if no unsupported we're all good
            if unsupported.is_empty() { break; }
            // otherwise, remove the unsupported bricks and see what happens
            count += unsupported.len();
            support_map = remove_bricks(&support_map, &unsupported);
        }
    }
    println!("PART 2: {}", count);
}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    x: (u64, u64),
    y: (u64, u64),
    z: (u64, u64),
}

impl Brick {
    /// Constructor
    fn new(x: (u64, u64), y: (u64, u64), z: (u64, u64)) -> Brick {
        Brick{ x, y, z }
    }

    /// Is this on the ground?
    fn on_ground(&self) -> bool { self.z.0 == 1 }

    /// Does this overlap with another brick?
    fn overlaps(&self, other: &Brick) -> bool {
        !(
            (self.x.0 > other.x.1) ||
            (self.x.1 < other.x.0) ||
            (self.y.0 > other.y.1) ||
            (self.y.1 < other.y.0) ||
            (self.z.0 > other.z.1) ||
            (self.z.1 < other.z.0)
        )
    }

    /// Does this brick support another?
    fn supports(&self, other: &Brick) -> bool {
        let area_above = Brick::new(
            self.x,
            self.y,
            (self.z.1 + 1, self.z.1 + 1),
        );
        other.overlaps(&area_above)
    }

    /// Does this brick rest on another?
    fn rests_on(&self, other: &Brick) -> bool { other.supports(self) }

    /// Drop brick down a level
    fn drop(&self) -> Brick {
        Brick::new(self.x, self.y, (self.z.0 - 1, self.z.1 - 1))
    }

    /// Utility to instantiate brick
    fn from_string(line: &str) -> Brick {
        let parts = line.split("~").collect_vec();
        let args = parts[0]
            .split(",")
            .zip(parts[1].split(","))
            .map(|pair|
                (pair.0.parse().unwrap(), pair.1.parse().unwrap())
            )
            .collect_vec();
        Brick{ x: args[0], y: args[1], z: args[2] }
    }

    /// Pretty representation
    fn to_string(&self) -> String {
        format!(
            "[x({}-{}), y({}-{}), z({}-{})]",
            self.x.0, self.x.1,
            self.y.0, self.y.1,
            self.z.0, self.z.1,
        )
    }
}

impl Debug for Brick {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}


/// Find resting positions of all the bricks
fn find_resting_positions(bricks: &[Brick]) -> Vec<Brick> {
    let mut at_rest: Vec<Brick> = vec![];
    let mut falling: HashSet<Brick> = HashSet::new();
    let mut queue: VecDeque<Brick> = VecDeque::new();

    falling.extend(bricks.iter());
    queue.extend(bricks.iter());
    while let Some(brick) = queue.pop_front() {
        falling.remove(&brick);

        // brick has come to rest on the ground or on some other brick
        if brick.on_ground() || at_rest.iter().any(|b| brick.rests_on(b)) {
            at_rest.push(brick);
            continue;
        }
        // check if this brick can drop down a level
        let dropped = brick.drop();
        let cant_drop =
            falling.iter().any(|b| b.overlaps(&dropped)) ||
            at_rest.iter().any(|b| b.overlaps(&dropped));
        let next = if cant_drop { brick } else { dropped };
        falling.insert(next);
        queue.push_back(next);
    }
    at_rest
}


/// Mapping from brick to set of bricks it is supported by
fn calc_support_map(bricks: &[Brick]) -> HashMap<Brick, HashSet<Brick>> {
    HashMap::from_iter(
        bricks.iter()
        .map(
            |brick| (
                *brick,  // k
                bricks.iter()  // v
                    .filter(|b| brick.rests_on(b))
                    .map(|b| b.clone())
                    .collect()
            )
        )
    )
}


/// Is this brick safe to distintegrate?
fn is_safe_to_disintegrate(map: &HashMap<Brick, HashSet<Brick>>, brick: &Brick) -> bool {
    for supporting in map.values() {
        if (supporting.len() == 1) && (supporting.contains(brick)) {
            return false;
        }
    }
    true
}


/// Remove bricks from map
fn remove_bricks(map: &HashMap<Brick, HashSet<Brick>>, bricks: &[&Brick]) -> HashMap<Brick, HashSet<Brick>> {
    let mut map = map.clone();
    for brick in bricks {
        map.remove(brick);
        for supporting in map.values_mut() {
            if supporting.contains(brick) { supporting.remove(brick); }
        }
    }
    map
}


fn load_bricks(content: &str) -> Vec<Brick> {
    content.split("\n").map(Brick::from_string).collect()
}