use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Galaxy {
    x: i64,
    y: i64,
    e_row: i64,
    e_col: i64,
}

impl Galaxy {
    fn parse(str: String) -> Vec<Self> {
        let e_rows = Self::empty_rows(str.clone());
        let rotated_str = Self::rotate(str.lines().collect()).join("\n");
        let e_cols = Self::empty_rows(rotated_str);

        let mut galaxies = Vec::new();
        for (y, line) in str.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    galaxies.push(Galaxy {
                        x: x as i64,
                        y: y as i64,
                        e_row: e_rows.clone().into_iter().filter(|i| i < &y).count() as i64,
                        e_col: e_cols.clone().into_iter().filter(|i| i < &x).count() as i64,
                    })
                }
            }
        }

        galaxies
    }

    fn empty_rows(str: String) -> Vec<usize> {
        str.lines()
            .enumerate()
            .filter_map(|(i, line)| {
                if line.chars().all(|c| c == '.') {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }

    fn rotate(lines: Vec<&str>) -> Vec<String> {
        let mut matrix = HashMap::new();
        let max_x = lines.len();
        let mut max_y = 0;
        for (x, line) in lines.iter().enumerate() {
            if max_y == 0 {
                max_y = line.chars().count();
            }
            for (y, char) in line.chars().enumerate() {
                matrix.insert((x, y), char);
            }
        }

        let mut lines = Vec::new();
        for y in 0..max_y {
            let mut chars = Vec::new();
            for x in 0..max_x {
                if let Some(char) = matrix.get(&(x, y)) {
                    chars.push(char);
                }
            }
            let line = chars.into_iter().collect::<String>();
            lines.push(line);
        }

        lines
    }

    fn distance(&self, galaxy: Self, years: i64) -> i64 {
        let mut distance = 0;
        let cols = (self.e_col - galaxy.e_col).abs();
        let rows = (self.e_row - galaxy.e_row).abs();
        let x = (self.x - galaxy.x).abs();
        let y = (self.y - galaxy.y).abs();

        distance += (cols + rows) * years;
        distance += if cols > 0 { x - cols } else { x };
        distance += if rows > 0 { y - rows } else { y };

        distance
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt");
    match input {
        Ok(str) => {
            let time = std::time::Instant::now();

            let galaxies = Galaxy::parse(str.clone());

            let mut result = 0;
            for i in 0..galaxies.len() - 1 {
                for j in i + 1..galaxies.len() {
                    result += galaxies[i].distance(galaxies[j].clone(), 2);
                }
            }

            println!("Result: {:?} ({:?})", result, time.elapsed());

            let mut result = 0;
            for i in 0..galaxies.len() - 1 {
                for j in i + 1..galaxies.len() {
                    result += galaxies[i].distance(galaxies[j].clone(), 1_000_000);
                }
            }

            println!("Result: {:?} ({:?})", result, time.elapsed())
        }
        _ => panic!("Not a string"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let test = std::fs::read_to_string("test.txt");
        match test {
            Ok(input) => {
                let galaxies = Galaxy::parse(input.clone());
                println!("{:?}", &galaxies);
                let mut result = 0;
                for i in 0..galaxies.len() - 1 {
                    for j in i + 1..galaxies.len() {
                        result += galaxies[i].distance(galaxies[j].clone(), 2);
                    }
                }

                assert_eq!(result, 374);
            }
            _ => panic!("Not a string"),
        };
    }

    #[test]
    fn part_2() {
        let test = std::fs::read_to_string("test.txt");
        match test {
            Ok(input) => {
                let galaxies = Galaxy::parse(input.clone());
                println!("{:?}", &galaxies);
                let mut result = 0;
                for i in 0..galaxies.len() - 1 {
                    for j in i + 1..galaxies.len() {
                        result += galaxies[i].distance(galaxies[j].clone(), 10);
                    }
                }

                assert_eq!(result, 1030);
            }
            _ => panic!("Not a string"),
        }
    }

    #[test]
    fn part_3() {
        let test = std::fs::read_to_string("test.txt");
        match test {
            Ok(input) => {
                let galaxies = Galaxy::parse(input.clone());
                println!("{:?}", &galaxies);
                let mut result = 0;
                for i in 0..galaxies.len() - 1 {
                    for j in i + 1..galaxies.len() {
                        result += galaxies[i].distance(galaxies[j].clone(), 100);
                    }
                }

                assert_eq!(result, 8410);
            }
            _ => panic!("Not a string"),
        }
    }
}
