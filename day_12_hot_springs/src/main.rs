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
    value: String,
    line: Vec<Spring>,
    mask: Vec<usize>,
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

        let line: Vec<Spring> = line.chars().map(|c| Spring::from(c)).collect();

        Self {
            value: value.to_string(),
            line,
            mask,
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
    fn arrangements(&self) -> usize {
        let sharps_sum: usize = self.mask.clone().into_iter().sum();

        let sharp_uses = self.line.iter().filter(|&c| c == &Spring::Damaged).count();
        let sharps_left = sharps_sum - sharp_uses;
        let question_indices: Vec<usize> = self
            .line
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if c == &Spring::Unknown { Some(i) } else { None })
            .collect();

        let mut str = "".to_string();
        for _ in &question_indices {
            str.push_str(&"?")
        }

        let results = self.substitution(&str, sharps_left, Vec::new(), Vec::new());

        let mut variants = Vec::new();
        for result in results {
            let mut res = self.line.clone();
            for (i, char) in result.into_iter().enumerate() {
                let index = question_indices[i];
                res[index] = char;
            }
            variants.push(res);
        }

        let mut good: Vec<String> = Vec::new();
        for variant in variants {
            if self.check(variant.clone()) {
                let str = self.to_str(variant.clone());
                good.push(str);
            }
        }

        good.len()
    }

    fn substitution(
        &self,
        a: &str,
        b: usize,
        mut result: Vec<Spring>,
        mut results: Vec<Vec<Spring>>,
    ) -> Vec<Vec<Spring>> {
        if b == 0 {
            for _ in result.len()..a.len() {
                result.push(Spring::Point);
            }

            results.push(result);
            return results;
        } else {
            let mut res1 = result.clone();
            let mut res2 = result.clone();

            res1.push(Spring::Damaged);
            results = self.substitution(a, b - 1, res1, results);
            if result.len() < a.len() - b {
                res2.push(Spring::Point);
                results = self.substitution(a, b, res2, results);
            }

            results
        }
    }

    fn check(&self, res: Vec<Spring>) -> bool {
        let str = self.to_str(res);

        let msk: Vec<usize> = str
            .split('.')
            .filter_map(|str| {
                if str.contains(&"#") {
                    Some(str.chars().count())
                } else {
                    None
                }
            })
            .collect();

        msk == self.mask
    }

    fn to_str(&self, v: Vec<Spring>) -> String {
        let mut chars: Vec<char> = Vec::new();
        for value in &v {
            match value {
                Spring::Point => chars.push('.'),
                Spring::Damaged => chars.push('#'),
                _ => chars.push('?'),
            };
        }

        chars.iter().collect::<String>()
    }
}

fn main() {
    let time = std::time::Instant::now();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let springs: Vec<Record> = input.lines().map(|line| line.into()).collect();
    let springs: Vec<usize> = springs.iter().map(|spring| spring.arrangements()).collect();

    let result: usize = springs.into_iter().sum();
    println!("Result: {:?} ({:?})", result, time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = std::fs::read_to_string("test.txt").unwrap();
        let springs: Vec<Record> = input.lines().map(|line| line.into()).collect();
        let springs: Vec<usize> = springs.iter().map(|spring| spring.arrangements()).collect();

        let result: usize = springs.into_iter().sum();
        assert_eq!(result, 21);
    }
}
