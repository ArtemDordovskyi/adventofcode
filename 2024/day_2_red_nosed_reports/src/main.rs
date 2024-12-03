use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let now = std::time::Instant::now();
    
    let mut input = File::open("input.txt")?;
    let mut content = String::new();
    input.read_to_string(&mut content)?;
    let lines = content.lines();
    
    let mut safe_reports = 0;
    let mut tolerate_reports = 0;
    
    for line in lines {
        let data: Vec<u8> = parse_line_to_numbers(line);
        if is_safe_report(&data) {
            safe_reports += 1;
        }
        if is_tolerate_report(&data) {
            tolerate_reports += 1;
        }
    }
    
    println!("Safe reports: {} ({:?})", safe_reports, now.elapsed());
    println!("Tolerate reports: {} ({:?})", tolerate_reports, now.elapsed());
    Ok(())
}

fn parse_line_to_numbers(line: &str) -> Vec<u8> {
    line.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn is_safe_report(data: &[u8]) -> bool {
    let (mut sign, mut error) = (0, 0);
    for i in 1..data.len() {
        let (current, previous) = (data[i], data[i - 1]);
        if !is_valid_transition(current, previous, &mut sign) {
            error = 1;
            break;
        }
    }
    
    error == 0
}

fn is_tolerate_report(data: &[u8]) -> bool {
    if is_safe_report(data) {
        return true;
    }
    
    for i in 0..data.len() {
        let filtered_data: Vec<u8> = data.iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, &value)| value)
            .collect();
        if is_safe_report(&filtered_data) {
            return true;
        }
    }

    false 
}

fn is_valid_transition(current: u8, previous: u8, sign: &mut i8) -> bool {
    if current > previous && (current - previous) < 4 && (*sign == 0 || *sign == 1) {
        *sign = 1;
    } else if current < previous && (previous - current) < 4 && (*sign == 0 || *sign == -1) {
        *sign = -1;
    } else {
        return false;
    }

    true
}
