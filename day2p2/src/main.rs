fn main() {
    let data = std::fs::read_to_string("data/games.txt").unwrap();
    let games = data.split("\n");
    let mut tot = 0;

    for game in games {
        let (mut red, mut green, mut blue) = (1, 1, 1);
        for set in game.split(":").last().unwrap().split(";") {
            for part in set.split(",") {
                let count = part.split(" ").nth(1).unwrap().parse::<u32>().unwrap();

                if part.contains("red") && count > red {
                    red = count;
                } else if part.contains("green") && count > green {
                    green = count;
                } else if part.contains("blue") && count > blue {
                    blue = count;
                }
            }
        }
        tot += red * green * blue;
    }
    println!("{}", tot);
}
