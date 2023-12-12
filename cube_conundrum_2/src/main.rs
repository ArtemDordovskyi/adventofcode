use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Cube {
    color: String,
    count: u32,
}

#[derive(Debug)]
struct Cubes {
    vec: Vec<Cube>,
}

impl Cubes {
    fn max_by_color(&self, color: String) -> Cube {
        let cubes = self.vec
            .clone()
            .into_iter()
            .filter(|x: &Cube| x.color == color)
            .collect::<Vec<Cube>>();

        let cube = cubes
            .iter()
            .max_by_key(|x: &&Cube| x.count)
            .unwrap()
            .clone();

        cube
    }
}

impl Game {

    fn new() -> Game {
        Game {
            red: 1,
            green: 1,
            blue: 1,
        }
    }
    fn get_by_color(&self, color: &str) -> u32 {
        match color {
            "red" => self.red,
            "green" => self.green,
            "blue" => self.blue,
            &_ => 0
        }
    }

    fn update_by_color(&mut self, color: &str, count: u32) {
        match color {
            "red" => self.red = count,
            "green" => self.green = count,
            "blue" => self.blue = count,
            &_ => {}
        }
    }
}

fn main() {
    let input = File::open("input.txt");
    let mut content =  String::new();

    let _ = input.expect("File not found").read_to_string(&mut content);
    let mut lines = content.lines();
    let mut games: Vec<Game> = Vec::new();

    while let Some(line) = lines.next() {
        let game_str: Vec<_> = line.split(": ").collect();

        // Second way to solve it.
        // let game_cubes = game_str[1].replace(";", ",");
        // let subsets: Vec<_> = game_cubes.split(", ").collect();
        // let cubes: Vec<Cube> = subsets.clone().iter().map(|&x| {
        //     let cube: Vec<_> = x.split_whitespace().collect();
        //     Cube {
        //         color: cube[1].to_string(),
        //         count: cube[0].parse::<u32>().unwrap(),
        //     }
        // }).collect::<Vec<Cube>>();
        //
        // let cubes_arr = Cubes {
        //     vec: cubes.clone(),
        // };
        //
        // let max_red = cubes_arr.max_by_color("red".to_string());
        // let max_green = cubes_arr.max_by_color("green".to_string());
        // let max_blue = cubes_arr.max_by_color("blue".to_string());
        //
        // let game = Game {
        //     red: max_red.count,
        //     green: max_green.count,
        //     blue: max_blue.count,
        // };


        let subsets: Vec<_> = game_str[1].split("; ").collect();
        let mut game = Game::new();
        for subset in subsets {
            let sets: Vec<_> = subset.split(", ").collect();
            for set in sets {
                let mut color_with_count: Vec<_> = set.split(" ").collect();
                let color = color_with_count.pop().unwrap();
                let count = color_with_count.pop().unwrap().parse::<u32>().unwrap();
                if &game.get_by_color(color) < &count {
                    game.update_by_color(color, count);
                }
            }
        }
        games.push(game)
    }

    let sum: u32 = games.iter().map(|game| game.red * game.green * game.blue).sum();
    println!("{:?}", sum);
}
