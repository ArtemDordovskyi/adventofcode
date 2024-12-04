use std::fs::File;
use std::io::{self, Read};

extern crate regex;
use regex::Regex;

fn main() -> io::Result<()> {    
    let content = read_file_to_string("input.txt")?;
    let now = std::time::Instant::now();
    let instruction_produces = calculate_instructions(&content);
    println!("Instructions produces: {} ({:?})", instruction_produces, now.elapsed());
    
    let now = std::time::Instant::now();
    let instruction_produces = calculate_enabled_instructions(&content);
    println!("Instructions produces: {} ({:?})", instruction_produces, now.elapsed());

    Ok(())
}

fn read_file_to_string(path: &str) -> io::Result<String> {
    let mut input = File::open(path)?;
    let mut content = String::new();
    input.read_to_string(&mut content)?;
    Ok(content)
}

fn calculate_instructions(content: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(content)
        .map(|cap| {
            let x = cap[1].parse::<u32>().unwrap();
            let y = cap[2].parse::<u32>().unwrap();
            x * y
        })
        .sum()
}

fn calculate_enabled_instructions(content: &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for cap in re.captures_iter(&content) {
        match &cap[0] {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    let x = cap[1].parse::<u32>().unwrap();
                    let y = cap[2].parse::<u32>().unwrap();
                    sum += x * y;
                }
            }
        }
    }
    sum
}
