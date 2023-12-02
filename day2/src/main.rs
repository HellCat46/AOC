fn main() {
    let data = std::fs::read_to_string("data/games.txt").unwrap();
    let games = data.split("\n");
    let mut tot = 0;

    for game in games {
        // I know i can just use index instead of parsing the gameid 
        let gameid: i32 = game
            .split(":")
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let mut playable = true;
        for set in game.split(":").last().unwrap().split(";") {
            for part in set.split(",") {
                let count = part.split(" ").nth(1).unwrap().parse::<u32>().unwrap();
                if part.contains("red") && count > 12 {
                    playable = false;
                    break;
                } else if part.contains("green") && count > 13 {
                    playable = false;
                    break;
                } else if part.contains("blue") && count > 14 {
                    playable = false;
                    break;
                }
            }

            if !playable {
                break;
            }
        }
        if playable {
            tot += gameid;
        }
    }
    println!("{}", tot);
}
