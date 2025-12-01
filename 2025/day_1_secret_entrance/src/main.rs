use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let now = std::time::Instant::now();

    let content = read_file_to_string("input.txt");
    let mut point = 50;
    let mut password = 0;
    let mut password_two = 0;

    for line in content?.lines() {
        if let Some((direction, mut value)) = parse_line(line) {
            value = handle_large_value(value, &mut password_two);
            process_move(direction, value, &mut point, &mut password, &mut password_two);
        }
    }

    println!("password: {} ({:?})", password, now.elapsed());
    println!("password method 0x434C49434B: {} ({:?})", password_two, now.elapsed());

    Ok(())
}

fn read_file_to_string(path: &str) -> io::Result<String> {
    let mut input = File::open(path)?;
    let mut content = String::new();
    input.read_to_string(&mut content)?;
    Ok(content)
}

fn parse_line(line: &str) -> Option<(char, i32)> {
    let mut chars = line.chars();
    let direction = chars.next()?;
    let value = chars.as_str().parse::<i32>().ok()?;
    Some((direction, value))
}

fn handle_large_value(mut value: i32, password_two: &mut i32) -> i32 {
    if value > 100 {
        let digits = value.to_string().len();
        let divisor = 10_i32.pow((digits - 1) as u32);
        *password_two += value / divisor;
        value %= divisor;
    }
    value
}

fn process_move(direction: char, value: i32, point: &mut i32, password: &mut i32, password_two: &mut i32) {
    match direction {
        'L' => {
            if value > *point {
                if *point > 0 {
                    *password_two += 1;
                }
                *point = 100 - (value - *point);
            } else {
                *point -= value;
            }
        }
        'R' => {
            if value + *point > 100 {
                *point = *point + value - 100;
                *password_two += 1;
            } else {
                *point += value;
            }
        }
        _ => {}
    }

    if *point == 0 || *point == 100 {
        *point = 0;
        *password += 1;
        *password_two += 1;
    }
}

