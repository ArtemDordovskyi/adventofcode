use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;

fn main() -> io::Result<()> {   
    let content = read_file_to_string("input.txt")?;
    let now = std::time::Instant::now();
    let mut xmas_count = 0;
    let mut x_mas_count = 0;

    let mut letters: HashMap<(u32, u32), char> = HashMap::new();
    // Iterate through each line and each character to populate the HashMap
    for (y, line) in content.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            letters.insert((x as u32, y as u32), ch);
        }
    } 
    // Print the resulting HashMap
    for ((x, y), ch) in &letters {
        if ch == &'X' {
            // Check all directions for "XMAS"
            xmas_count += check_direction(&letters, *x, *y, 0, 1);
            xmas_count += check_direction(&letters, *x, *y, 0, -1);
            xmas_count += check_direction(&letters, *x, *y, 1, 0);
            xmas_count += check_direction(&letters, *x, *y, -1, 0);
            xmas_count += check_direction(&letters, *x, *y, 1, 1);
            xmas_count += check_direction(&letters, *x, *y, -1, -1);
            xmas_count += check_direction(&letters, *x, *y, -1, 1);
            xmas_count += check_direction(&letters, *x, *y, 1, -1);
        }
        if ch == &'A' {
            x_mas_count += check_x_mas(&letters, *x, *y);
        }
    }
    println!("XMAS: {} ({:?})", xmas_count, now.elapsed());
    println!("X-MAS: {} ({:?})", x_mas_count, now.elapsed());

    Ok(())
}

fn read_file_to_string(path: &str) -> io::Result<String> {
    let mut input = File::open(path)?;
    let mut content = String::new();
    input.read_to_string(&mut content)?;
    Ok(content)
}

fn check_direction(letters: &HashMap<(u32, u32), char>, x: u32, y: u32, dx: i32, dy: i32) -> u32 {
    let directions = [
        (1, 'M'),
        (2, 'A'),
        (3, 'S'),
    ];
    
    for (i, ch) in directions.iter() {
        let new_x = (x as i32 + i * dx) as u32;
        let new_y = (y as i32 + i * dy) as u32;
        if letters.get(&(new_x, new_y)) != Some(ch) {
            return 0; 
        }
    }

    1
}

fn check_x_mas(letters: &HashMap<(u32, u32), char>, x: u32, y: u32) -> u32 {    
    let new_x_minus_1 = (x as i32 - 1) as u32;
    let new_x_plus_1 = (x as i32 + 1) as u32;
    let new_y_minus_1 = (y as i32 - 1) as u32;
    let new_y_plus_1 = (y as i32 + 1) as u32;

    if ((letters.get(&(new_x_minus_1, new_y_minus_1)) == Some(&'M') && letters.get(&(new_x_plus_1, new_y_plus_1)) == Some(&'S')) ||
    (letters.get(&(new_x_minus_1, new_y_minus_1)) == Some(&'S') && letters.get(&(new_x_plus_1, new_y_plus_1)) == Some(&'M'))) &&
    ((letters.get(&(new_x_minus_1, new_y_plus_1)) == Some(&'M') && letters.get(&(new_x_plus_1, new_y_minus_1)) == Some(&'S')) || 
    (letters.get(&(new_x_minus_1, new_y_plus_1)) == Some(&'S') && letters.get(&(new_x_plus_1, new_y_minus_1)) == Some(&'M'))) {
        return 1;
    }
    
    0
}
