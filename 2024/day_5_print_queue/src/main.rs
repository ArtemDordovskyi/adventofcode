use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let now = std::time::Instant::now();
    let input_data = read_file_to_string("input.txt")?;
    let (rules, updates) = parse_input(&input_data);

    let mut correct_middle_sum = 0;
    let mut incorrect_middle_sum = 0;

    for update in updates.iter() {
        if check_order(update, &rules) {
            correct_middle_sum += find_middle(update)
        } else {
            let sorted_update = sort_update(update, &rules);
            incorrect_middle_sum += find_middle(&sorted_update);
        } 
    }

    println!("Correct Middle page sum: {} ({:?})", correct_middle_sum, now.elapsed());
    println!("Incorrect Middle page sum: {} ({:?})", incorrect_middle_sum, now.elapsed());
    Ok(())
}

fn read_file_to_string(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut lines = input.lines();
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    
    for line in &mut lines {
        if line.contains('|') {
            let parts: Vec<&str> = line.split('|').collect();
            let rule = (parts[0].parse().unwrap(), parts[1].parse().unwrap());
            rules.push(rule);
        } else if !line.is_empty() {
            let update: Vec<i32> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            updates.push(update);
        }
    }
    
    (rules, updates)
}

fn check_order(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    let positions: HashMap<_, _> = update.iter().enumerate().map(|(i, &page)| (page, i)).collect();
    for &(x, y) in rules {
        if let (Some(&pos_x), Some(&pos_y)) = (positions.get(&x), positions.get(&y)) {
            if pos_x > pos_y {
                return false;
            }
        }
    } 
    
    true
}

fn find_middle(pages: &Vec<i32>) -> i32 {
    pages[pages.len() / 2]
}

fn sort_update(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut sorted_update = update.clone();
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut in_degree: HashMap<i32, usize> = HashMap::new();
    
    for &page in update {
        graph.entry(page).or_default();
        in_degree.entry(page).or_insert(0);
    }
    
    for &(x, y) in rules {
        if update.contains(&x) && update.contains(&y) {
            graph.entry(x).or_default().insert(y);
            *in_degree.entry(y).or_insert(0) += 1;
        }
    }
    
    let mut queue: Vec<i32> = in_degree.iter().filter(|&(_, &deg)| deg == 0).map(|(&k, _)| k).collect();
    sorted_update.clear(); while let Some(node) = queue.pop() {
        sorted_update.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                let degree = in_degree.get_mut(&neighbor).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    queue.push(neighbor);
                }
            }
        }
    } 
    
    sorted_update
}
