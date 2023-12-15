// 45 mins all in

use itertools::Itertools;

pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let solution: usize = content.split(",").map(to_hash).sum();
    println!("PART 1: {}", solution);
}


fn part2(content: &str) {
    let boxes = install(content);
    println!("PART 2: {}", calc_focusing_power(boxes));
}


#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

impl Lens {
    fn new(label: &str, focal_length: usize) -> Lens {
        Lens{ label: String::from(label), focal_length }
    }
}


fn install(content: &str) -> Vec<Vec<Lens>> {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    for step in content.split(",") {
        // update or add lens
        if step.contains("=") {
            let parts = step.split("=").collect_vec();
            let box_num = to_hash(parts[0]);
            let focal_length: usize = parts[1].parse().unwrap();
            let lens = Lens::new(parts[0], focal_length);
            let mut found = false;
            for (i, existing) in boxes[box_num].iter().enumerate() {
                if existing.label == lens.label {
                    found = true;
                    boxes[box_num][i] = lens.clone();
                    break;
                }
            }
            if !found {
                boxes[box_num].push(lens);
            }
        // or remove lens
        } else {
            let key = String::from(&step[..step.len() - 1]);
            let box_num = to_hash(&key);
            let list = boxes[box_num].clone();
            for (i, existing) in list.iter().enumerate() {
                if existing.label == key {
                    boxes[box_num].remove(i);
                }
            }
        }
    }
    boxes
}


fn to_hash(step: &str) -> usize {
    let mut value = 0usize;
    for c in step.chars() {
        value = ((value + c as usize) * 17) % 256;
    }
    value
}


fn calc_focusing_power(boxes: Vec<Vec<Lens>>) -> usize {
    boxes.iter()
        .enumerate()
        .map(
            |x| x.1.iter()
                .enumerate()
                .map(|y|
                    (x.0 + 1) * (y.0 + 1) * y.1.focal_length
                )
                .sum::<usize>()
        )
        .sum()
}