use std::fs::File;
use std::io::Read;

fn main() {
    let now = std::time::Instant::now();

    let input = File::open("input.txt");
    let mut content =  String::new();

    let _ = input.expect("File not found").read_to_string(&mut content);
    let lines = content.lines();

    let mut safe_reports = 0;
    let mut tolerate_reports = 0;
    
    for line in lines {
        let data: Vec<u8> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        // Iterate through the vector, starting from the second element
        let mut sign: i8 = 0;
        let mut error: u8 = 0;
        for i in 1..data.len() {
            let current = data[i];
            let previous = data[i - 1];
            
            if current > previous && (current - previous) < 4 && (sign == 0 || sign == 1) {
                sign = 1;
            } else if current < previous && (previous - current) < 4 && (sign == 0 || sign == -1) {
                sign = -1;
            } else {
                if error == 0 {
                    let mut increased = 0;

                    let filtered_data: Vec<u8> = data
                        .iter()
                        .enumerate()
                        .filter(|&(j, _)| j != i - 1)
                        .map(|(_, &value)| value)
                        .collect();
                    let mut filtered_sign: i8 = 0;
                    let mut filtered_error: u8 = 0;
                    
                    for j in 1..filtered_data.len() {
                        let cur = filtered_data[j];
                        let prev = filtered_data[j - 1];
                        
                        if cur > prev && (cur - prev) < 4 && (filtered_sign == 0 || filtered_sign == 1) {
                            filtered_sign = 1;
                        } else if cur < prev && (prev - cur) < 4 && (filtered_sign == 0 || filtered_sign == -1) {
                            filtered_sign = -1;
                        } else {
                            filtered_error = 1;
                        }
    
                        if j == (data.len() - 2) {
                            if filtered_error == 0 {
                                increased = 1;
                                tolerate_reports += 1;
                            }
                        }
                    }
    
                    if increased == 0 {
                        let filtered_data: Vec<u8> = data
                        .iter()
                        .enumerate()
                        .filter(|&(j, _)| j != i)
                        .map(|(_, &value)| value)
                        .collect();
                        let mut filtered_sign: i8 = 0;
                        let mut filtered_error: u8 = 0;
                        
                        for j in 1..filtered_data.len() {
                            let cur = filtered_data[j];
                            let prev = filtered_data[j - 1];
                            
                            if cur > prev && (cur - prev) < 4 && (filtered_sign == 0 || filtered_sign == 1) {
                                filtered_sign = 1;
                            } else if cur < prev && (prev - cur) < 4 && (filtered_sign == 0 || filtered_sign == -1) {
                                filtered_sign = -1;
                            } else {
                                filtered_error = 1;
                            }
    
                            if j == (data.len() - 2) {
                                if filtered_error == 0 {
                                    tolerate_reports += 1;
                                }
                            }
                        }
                    }
                }
                error = 1;
            }

            if i == (data.len() - 1) {
                if error == 0 {
                    safe_reports += 1;
                    tolerate_reports += 1;
                }
            }
        }
    }

    println!("Safe reports: {} ({:?})", safe_reports, now.elapsed());
    println!("Tolerate reports: {} ({:?})", tolerate_reports, now.elapsed());
}
