use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt::Formatter;

#[derive(Clone, Debug, PartialEq)]
enum Spring {
    Point,
    Unknown,
    Damaged,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Spring::Point,
            '#' => Spring::Damaged,
            _ => Spring::Unknown,
        }
    }
}

#[derive(Debug)]
struct Record {
    line: Vec<Spring>,
    mask: Vec<usize>,
    line5: Vec<Spring>,
    mask5: Vec<usize>,
}

impl From<&str> for Record {
    fn from(value: &str) -> Self {
        let [line, dmg] = value.split_whitespace().take(2).collect::<Vec<_>>()[..] else {
            panic!("Wrong input data");
        };

        let mask: Vec<usize> = dmg
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect();

        let mut mask5: Vec<usize> = Vec::new();
        let mut line5: Vec<&str> = Vec::new();
        for _ in 0..5 {
            for m in &mask {
                mask5.push(*m);
            }
            line5.push(line);
        }

        let line: Vec<Spring> = line.chars().map(|c| c.into()).collect();

        let line5 = line5.join("?");
        let line5: Vec<Spring> = line5.chars().map(|c| c.into()).collect();

        Self {
            line,
            mask,
            line5,
            mask5,
        }
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut chars: Vec<char> = Vec::new();
        for value in &self.line {
            match value {
                Spring::Point => chars.push('.'),
                Spring::Damaged => chars.push('#'),
                _ => chars.push('?'),
            };
        }

        let str = chars.iter().collect::<String>();
        write!(f, "{}", str)
    }
}

impl Record {
    fn arrangements(&self, x5: bool) -> usize {
        let mut dp: HashMap<(usize, usize, usize), usize> = HashMap::new();

        self.solutions(x5, 0, 0, 0, &mut dp)
    }

    fn solutions(
        &self,
        x5: bool,
        i: usize,
        bi: usize,
        current: usize,
        dp: &mut HashMap<(usize, usize, usize), usize>,
    ) -> usize {
        let key = (i, bi, current);
        if let Some(&val) = dp.get(&key) {
            return val;
        }

        let (&ref line, &ref mask) = if x5 {
            (&self.line5, &self.mask5)
        } else {
            (&self.line, &self.mask)
        };

        if i == line.len() {
            return if (bi == mask.len() && current == 0)
                || (bi == mask.len() - 1 && mask[bi] == current)
            {
                1
            } else {
                0
            };
        }

        let mut result = 0;
        for spring in &[Spring::Point, Spring::Damaged] {
            if line[i] == *spring || line[i] == Spring::Unknown {
                if *spring == Spring::Point && current == 0 {
                    result += self.solutions(x5, i + 1, bi, 0, dp);
                } else if *spring == Spring::Point
                    && current > 0
                    && bi < mask.len()
                    && mask[bi] == current
                {
                    result += self.solutions(x5, i + 1, bi + 1, 0, dp);
                } else if *spring == Spring::Damaged {
                    result += self.solutions(x5, i + 1, bi, current + 1, dp);
                }
            }
        }
        dp.insert(key, result);
        result
    }
}

fn main() {
    let time = std::time::Instant::now();
    let input = std::fs::read_to_string("input.txt").expect("Unable to read file");
    let springs: Vec<Record> = input.lines().map(|line| line.into()).collect();
    let results: Vec<usize> = springs
        .par_iter()
        .map(|spring| spring.arrangements(false))
        .collect();

    let result: usize = results.into_iter().sum();
    println!("Result: {:?} ({:?})", result, time.elapsed());

    let results: Vec<usize> = springs
        .par_iter()
        .map(|spring| spring.arrangements(true))
        .collect();

    let result: usize = results.into_iter().sum();
    println!("Result: {:?} ({:?})", result, time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = std::fs::read_to_string("test.txt").unwrap();
        let springs: Vec<Record> = input.lines().map(|line| line.into()).collect();
        let springs: Vec<usize> = springs
            .iter()
            .map(|spring| spring.arrangements(false))
            .collect();

        let result: usize = springs.into_iter().sum();
        assert_eq!(result, 21);
    }

    #[test]
    fn part_2() {
        let input = std::fs::read_to_string("test.txt").unwrap();
        let springs: Vec<Record> = input.lines().map(|line| line.into()).collect();
        let springs: Vec<usize> = springs
            .iter()
            .map(|spring| spring.arrangements(true))
            .collect();

        let result: usize = springs.into_iter().sum();
        assert_eq!(result, 525152);
    }
}
