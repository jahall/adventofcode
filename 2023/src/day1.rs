pub fn run(content: String) {
    let parts = content.split("\n").map(|line| line.trim());
    let mut buffer = 0;
    let mut biggest = 0;
    for part in parts {
        if part.len() == 0 {
            if buffer > biggest {
                biggest = buffer
            };
            buffer = 0;
            continue;
        }
        let value: u32 = part.parse().expect("Noo");
        buffer += value;
    }
    if buffer > biggest {
        biggest = buffer
    };
    println!("PART 1: {}", biggest);
}