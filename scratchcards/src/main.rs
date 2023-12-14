use std::fs::File;
use std::io::Read;

fn main() {
    let input = File::open("input.txt");
    let mut content =  String::new();

    let _ = input.expect("File not found").read_to_string(&mut content);
    let lines = content.lines();

    let mut sum = 0;

    for line in lines {
        let card: Vec<_> = line.split("|").collect();
        let win: Vec<_> = card[0]
            .split(":")
            .collect::<Vec<_>>()[1]
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let pull: Vec<_> = card[1]
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let mut intersection = Vec::new();
        for i in &win {
            if pull.contains(&i) {
                intersection.push(i);
            }
        }

        if intersection.len() > 0 {
            sum += 2_i32.pow((intersection.len() - 1).try_into().unwrap())
        }

        // println!("Win: {:?}", win);
        // println!("Pull: {:?}", pull);
        // println!("Intersection: {:?}", intersection);
    }

    println!("{}", sum);
}
