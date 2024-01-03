// 1.5 hours for part 1

use itertools::Itertools;


pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let hailstones = load_hailstones(content);
    let (min, max) = if hailstones.len() < 100 {
        (7.0, 27.0)
    } else {
        (200000000000000.0, 400000000000000.0)
    };
    let count =
        hailstones.iter().enumerate()
            .cartesian_product(hailstones.iter().enumerate())
            .filter(|(h1, h2)| h1.0 < h2.0)
            .map(|(h1, h2)| h1.1.future_xy_crossing(h2.1))
            .filter(|xy| is_in_box(*xy, min, max))
            .count();
    println!("PART 1: {}", count);
}


fn part2(_content: &str) {
    println!("PART 2: {}", -1);
}


#[derive(Debug, Clone, Copy)]
struct Hailstone {
    px: f64,
    py: f64,
    _pz: f64,
    vx: f64,
    vy: f64,
    _vz: f64,
}

impl Hailstone {
    /// Instantiate hailstone
    fn new(px: f64, py: f64, pz: f64, vx: f64, vy: f64, vz: f64) -> Hailstone {
        Hailstone{ px, py, _pz: pz, vx, vy, _vz: vz }
    }

    /// Return location of future crossing...if there is a future crossing
    fn future_xy_crossing(&self, other: &Hailstone) -> Option<(f64, f64)> {
        // gradients
        let m1 = self.vy / self.vx;
        let m2 = other.vy / other.vx;
        // trajectories run parallel and in same direction
        // NOTE: ...one could still overtake the other, so not quite right here!
        if (m1 - m2).abs() < 1e-12 {
            return None;
        }
        // offsets
        let c1 = self.py - m1 * self.px;
        let c2 = other.py - m2 * other.px;
        // crossover
        let x = (c2 - c1) / (m1 - m2);
        let y = m1 * x + c1;
        // was it in the past?
        if self.is_xy_in_past(x, y) || other.is_xy_in_past(x, y) {
            return None;
        }
        // otherwise return crossover
        Some((x, y))
    }

    fn is_xy_in_past(&self, px: f64, py: f64) -> bool {
        let xdiff = px - self.px;
        let ydiff = py - self.py;
        ((xdiff > 0.0) && self.vx < 0.0) ||
        ((xdiff < 0.0) && self.vx > 0.0) ||
        ((ydiff > 0.0) && self.vy < 0.0) ||
        ((ydiff < 0.0) && self.vy > 0.0)
    }

}


// Is this (optional) xy position in the box?
fn is_in_box(xy: Option<(f64, f64)>, min: f64, max: f64) -> bool {
    match xy {
        Some(xy) => {
            (xy.0 >= min) &&
            (xy.0 <= max) &&
            (xy.1 >= min) &&
            (xy.1 <= max)
        },
        _ => false,
    }
}


fn load_hailstones(content: &str) -> Vec<Hailstone> {
    content.split("\n")
        .map(
            |line| {
                let parts = line.split_whitespace()
                    .filter(|x| *x != "@")
                    .map(|x| x.replace(",", ""))
                    .map(|x| x.parse().unwrap())
                    .collect_vec();
                Hailstone::new(parts[0], parts[1], parts[2], parts[3], parts[4], parts[5])
            }
        )
        .collect()
}