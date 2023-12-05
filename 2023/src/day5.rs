// 1 hour for part 1 ...3+ hours for part 2!

pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &str) {
    let (seeds, maps) = parse_content(content);
    let mut min_id: usize = 0;
    for seed in seeds {
        let mut id = seed.clone();
        for map in &maps {
            id = map.forward(id);
        }
        if (min_id == 0) | (id < min_id) { min_id = id };
    }
    println!("PART 1: {}", min_id);
}


fn part2(content: &str) {
    let (seeds, maps_list) = parse_content(content);

    // convert seeds to ranges
    let seeds: Vec<Range> = 
        seeds.chunks(2)
        .map(|x| Range::new(x[0], x[0] + x[1]))
        .collect();

    // determine an end to end map from seed to location
    let mut end_to_end_map = maps_list[0].clone();
    for map in maps_list[1..].iter() {
        end_to_end_map = Map::collapse(&end_to_end_map, &map);
    }
    end_to_end_map.sort();

    // traverse up the destinations till we hit a seed!
    for range in end_to_end_map.ranges.iter() {
        for val in range.source.start..range.source.end {
            for chunk in seeds.iter() {
                if chunk.contains(&val) {
                    println!("PART 2: {}", end_to_end_map.forward(val));
                    return;
                }
            }
        }
    }
}


#[derive(Debug, Copy, Clone)]
struct Range { start: usize, end: usize }

impl Range {
    fn new(start: usize, end: usize) -> Range {
        Range{ start, end }
    }

    fn contains(&self, value: &usize) -> bool {
        (value >= &self.start) & (value < &self.end)
    }
}


#[derive(Debug, Copy, Clone)]
struct RangeMap {
    dest: Range,
    source: Range,
}


impl RangeMap {
    fn new(input: Vec<usize>) -> RangeMap {
        RangeMap{
            dest: Range::new(input[0], input[0] + input[2]),
            source: Range::new(input[1], input[1] + input[2]),
        }
    }
}


#[derive(Debug, Clone)]
struct Map {
    ranges: Vec<RangeMap>
}


impl Map {
    fn new(ranges: Vec<RangeMap>) -> Map {
        Map{ ranges }
    }

    fn collapse(map1: &Map, map2: &Map) -> Map {
        let mut combined = map1.dest_boundaries();
        combined.extend(map2.source_boundaries());
        combined.sort();
        combined.dedup();
        let len = combined.len();
        let mut buffer: Vec<RangeMap> = vec![];
        for (i, value) in combined[..len - 1].iter().enumerate() {
            let source = map1.backward(*value);
            let dest = map2.forward(*value);
            let length = combined[i + 1] - value;
            buffer.push(RangeMap::new(vec![dest, source, length]));
        }
        Map::new(buffer)
    }

    fn forward(&self, value: usize) -> usize {
        for range in &self.ranges {
            if (value >= range.source.start) & (value < range.source.end) {
                return range.dest.start + value - range.source.start;
            }
        }
        value
    }

    fn backward(&self, value: usize) -> usize {
        for range in &self.ranges {
            if (value >= range.dest.start) & (value < range.dest.end) {
                return range.source.start + value - range.dest.start;
            }
        }
        value
    }

    fn sort(&mut self) {
        self.ranges.sort_by(|a, b| a.dest.start.cmp(&b.dest.start))
    }

    fn source_boundaries(&self) -> Vec<usize> {
        let mut x: Vec<usize> = vec![];
        for range in &self.ranges {
            x.push(range.source.start);
            x.push(range.source.end);
        }
        x.sort();
        x.dedup();
        x
    }

    fn dest_boundaries(&self) -> Vec<usize> {
        let mut x: Vec<usize> = vec![];
        for range in &self.ranges {
            x.push(range.dest.start);
            x.push(range.dest.end);
        }
        x.sort();
        x.dedup();
        x
    }
}


fn parse_content(content: &str) -> (Vec<usize>, Vec<Map>) {
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
    let mut maps: Vec<Map> = vec![];
    let mut buffer: Vec<RangeMap> = vec![];
    for (i, line) in lines.iter().enumerate() {
        if (i < 3) | (line == &"") { continue };
        if line.chars().nth(0).unwrap().is_digit(10) {
            let parts: Vec<usize> =
                line.split(" ")
                .map(|v| v.parse().unwrap())
                .collect();
            buffer.push(RangeMap::new(parts));
        } else {
            maps.push(Map::new(buffer));
            buffer = vec![];
        }
    }
    maps.push(Map::new(buffer));
    (seeds, maps)
}
