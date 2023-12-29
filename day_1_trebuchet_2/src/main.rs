use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[derive(PartialEq, Hash, Debug)]
struct Digit {
    num_str: String,
    position: usize
}

fn main() {
    let input = File::open("input.txt");
    let mut content =  String::new();

    let _ = input.expect("File not found").read_to_string(&mut content);
    let mut lines = content.lines();

    let mut sum = 0;
    let numbers = HashMap::from([
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
        ("0", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("zero", "0"),
    ]);

    while let Some(line) = lines.next() {
        let mut digit_str = String::new();
        let mut min = Digit {
            num_str: String::new(),
            position: line.len() + 1,
        };

        let mut max = Digit {
            num_str: String::new(),
            position: 0,
        };

        for number in numbers.keys() {
            if Some(min.position) > line.find(number) && line.find(number) != None {
                min.num_str = number.to_string();
                min.position = line.find(number).unwrap();
            }
            if Some(max.position) <= line.rfind(number) && line.rfind(number) != None {
                max.num_str = number.to_string();
                max.position = line.rfind(number).unwrap();
            }
        }

        let first = numbers.get(&*min.num_str).unwrap();
        let last = numbers.get(&*max.num_str).unwrap();
        digit_str.push_str(first);
        digit_str.push_str(last);

        sum += digit_str.parse::<u32>().unwrap();
    }
    println!("{}", sum);
}
