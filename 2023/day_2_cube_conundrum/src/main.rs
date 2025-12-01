use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    id: u32,
    possible: bool,
}

// possible: only 12 red cubes, 13 green cubes, and 14 blue cubes
// others impossible
// calculate sum of possible games

fn main() {
    let possibilities = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let input = File::open("input.txt");
    let mut content =  String::new();

    let _ = input.expect("File not found").read_to_string(&mut content);
    let mut lines = content.lines();
    let mut games: Vec<Game> = Vec::new();

    while let Some(line) = lines.next() {
        let game: Vec<_> = line.split(": ").collect();
        let game_with_id: Vec<_> = game[0].split_whitespace().collect();
        let id = game_with_id.last().unwrap().parse::<u32>().unwrap();
        let subsets: Vec<_> = game[1].split("; ").collect();
        let mut possible = true;

        for subset in subsets {
            let sets: Vec<_> = subset.split(", ").collect();
            for set in sets {
                let mut color_with_count: Vec<_> = set.split(" ").collect();
                let color = color_with_count.pop().unwrap();
                let count = color_with_count.pop().unwrap().parse::<u32>().unwrap();
                if possibilities.get(&color).unwrap() < &count {
                    possible = false;
                    break;
                }
            }
            if !possible {
                break;
            }
        }

        let game = Game {
            id,
            possible,
        };

        games.push(game)
    }

    let sum: u32 = games.iter().filter(|game| game.possible).map(|game| game.id).sum();
    println!("{:?}", sum);
}
