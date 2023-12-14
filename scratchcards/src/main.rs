use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let input = File::open("input.txt");
    let mut content =  String::new();

    let _ = input.expect("File not found").read_to_string(&mut content);
    let lines = content.lines();

    let mut sum = 0;
    let mut instances = HashMap::new();

    for (index, line) in lines.enumerate() {
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

        instances.entry(index).and_modify(|i| *i += 1).or_insert(1);
        let copies: usize = *instances.get(&index).unwrap();
        // println!("Intersection: {:?}", intersection);
        // println!("Copies: {:?}", copies);

        let indices = intersection.len() + 1;
        let mut ind = 1;
        while ind < indices {
            instances.entry(ind + index).and_modify(|i| *i += copies).or_insert(copies);
            ind += 1
        }


        if intersection.len() > 0 {
            sum += 2_i32.pow((intersection.len() - 1).try_into().unwrap())
        }

        // println!("Win: {:?}", win);
        // println!("Pull: {:?}", pull);
        // println!("Intersection: {:?}", intersection);
    }

    let sum2: usize = instances.values().sum();

    println!("{}", sum);
    println!("{:?}", sum2);
}
