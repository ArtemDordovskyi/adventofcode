use std::fs::File;
use std::io::Read;

fn main() {
    let input = File::open("input.txt");
    let mut content =  String::new();

    let _ = input.expect("File not found").read_to_string(&mut content);
    let mut lines = content.lines();

    let mut sum = 0;

    while let Some(line) = lines.next() {
        let mut digit_str = String::new();

        let chs = line.chars();
        let first = chs.clone().find(|x| x.to_digit(10).is_some()).unwrap();
        let last = chs.clone().rfind(|x| x.to_digit(10).is_some()).unwrap();

        digit_str.push_str(&first.to_string());
        digit_str.push_str(&last.to_string());

        sum += digit_str.parse::<u32>().unwrap();
    }
    println!("{}", sum);
}
