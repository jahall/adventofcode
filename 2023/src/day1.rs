pub fn run(content: String) {
    let parts = content.split("\n").map(|line| line.trim());
    for part in parts {
        dbg!(part.len());
        println!("{}", part);
    }
}