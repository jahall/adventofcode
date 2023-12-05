pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let (seeds, maps_list) = parse_content(content);
    let mut min_id: usize = 0;
    for seed in seeds {
        let mut id = seed.clone();
        for maps in &maps_list {
            for map in maps {
                if (id >= map[1]) & (id < map[1] + map[2]) {
                    id = map[0] + id - map[1];
                    break;
                }
            }
        }
        if (min_id == 0) | (id < min_id) { min_id = id };
    }
    println!("PART 1: {}", min_id);
}


fn part2(content: &str) {
    println!("PART 2: {}", -1);
}

fn parse_content(content: &str) -> (Vec<usize>, Vec<Vec<[usize; 3]>>) {
    let lines: Vec<&str> =
        content.split("\n")
        .map(|line| line.trim())
        .collect();
    let seeds: Vec<usize> = 
        lines[0].split(": ")
        .last()
        .unwrap()
        .split(" ")
        .map(|v| v.parse().unwrap())
        .collect();
    let mut maps: Vec<Vec<[usize; 3]>> = vec![];
    let mut buffer: Vec<[usize; 3]> = vec![];
    for (i, line) in lines.iter().enumerate() {
        if (i < 3) | (line == &"") { continue };
        if line.chars().nth(0).unwrap().is_digit(10) {
            let parts: Vec<usize> =
                line.split(" ")
                .map(|v| v.parse().unwrap())
                .collect();
            buffer.push([parts[0], parts[1], parts[2]]);
        } else {
            maps.push(buffer);
            buffer = vec![];
        }
    }
    maps.push(buffer);
    (seeds, maps)
}