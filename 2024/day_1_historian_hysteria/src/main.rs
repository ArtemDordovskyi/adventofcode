use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let now = std::time::Instant::now();

    let input = File::open("input.txt");
    let mut content =  String::new();

    let _ = input.expect("File not found").read_to_string(&mut content);
    let lines = content.lines();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        left.push(parts[0].parse::<i32>().unwrap());
        right.push(parts[1].parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    // Calculate the sum of differences
    let total_distance: i32 = left.iter()
                                  .zip(right.iter())
                                  .map(|(a, b)| (a - b).abs())
                                  .sum();
    println!("Total distance: {} ({:?})", total_distance, now.elapsed());

    let mut left_counts = HashMap::new();
    for &number in &left {
        *left_counts.entry(number).or_insert(0) += 1;
    }

    let mut right_counts = HashMap::new();
    for &number in &right {
        *right_counts.entry(number).or_insert(0) += 1;
    }

    let mut similarities = HashMap::new();
    let mut similarity_score = 0;

    for (&key, &left_count) in &left_counts {
        if let Some(&right_count) = right_counts.get(&key) {
            let similarity = key * left_count * right_count;
            similarities.insert(key, similarity);
            similarity_score += similarity;
        }
    }

    println!("similarity score: {} ({:?})", similarity_score, now.elapsed());
}
