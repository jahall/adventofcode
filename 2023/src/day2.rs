// 26 mins

pub fn run(content: String) {
    part1(&content);
    part2(&content);
}


fn part1(content: &String) {
    let games = content.split("\n").map(|line| line.trim());
    let mut id_sum = 0;
    for game in games {
        let parts: Vec<&str> = game.split(": ").collect();
        let id: u32 = parts[0][5..].parse().unwrap();
        let sets: Vec<&str> = parts[1].split("; ").collect();
        let mut success = true;
        for set in sets {
            let balls: Vec<(i32, &str)> = set.split(", ").map(to_counts).collect();
            for (count, color) in balls {
                if ((color == "red") & (count > 12)) |
                    ((color == "green") & (count > 13)) | 
                    ((color == "blue") & (count > 14)) {
                    success = false;
                    break;
                }
            }
        }
        if success {
            id_sum += id;
        }
    }
    println!("PART 1: {}", id_sum);
}


fn part2(content: &String) {
    let games = content.split("\n").map(|line| line.trim());
    let mut power_sum = 0;
    for game in games {
        let parts: Vec<&str> = game.split(": ").collect();
        let sets: Vec<&str> = parts[1].split("; ").collect();
        let mut n_red = 0;
        let mut n_green = 0;
        let mut n_blue = 0;
        for set in sets {
            let balls: Vec<(i32, &str)> = set.split(", ").map(to_counts).collect();
            for (count, color) in balls {
                if (color == "red") & (count > n_red) { n_red = count };
                if (color == "green") & (count > n_green) { n_green = count };
                if (color == "blue") & (count > n_blue) { n_blue = count };
            }
        }
        power_sum += n_red * n_green * n_blue;
    }
    println!("PART 2: {}", power_sum);
}


fn to_counts(count_str: &str) -> (i32, &str) {
    let parts: Vec<&str> = count_str.split(" ").collect();
    let count: i32 = parts[0].parse().unwrap();
    (count, parts[1])
}