#[derive(Clone, Debug)]
struct Sequence {
    val: Vec<i32>
}

impl From<&str> for Sequence {
    fn from(value: &str) -> Self {
        let val: Vec<i32> = value
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        Self {
            val,
        }
    }
}

impl Sequence {
    fn next_seq(val: Vec<i32>, up: bool) -> i32 {
        if val.iter().all(|e| e == &0) {
            return 0
        }

        let mut next_val = Vec::new();
        for i in 0..val.len() - 1 {
            next_val.push(val[i+1] - val[i]);
        }

        if up {
            val.last().unwrap() + Sequence::next_seq(next_val, up)
        } else {
            val.first().unwrap() - Sequence::next_seq(next_val, up)
        }

    }

    fn next(&self, up: bool) -> i32 {
        Sequence::next_seq(self.val.clone(), up)
    }
}

fn main() {
    let time = std::time::Instant::now();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<Sequence> = input.lines().map(|line| line.into()).collect();

    let result: i32 = lines.clone().into_iter().map(|line| line.next(true)).sum();

    println!("Result: {:?} ({:?})", result, time.elapsed());

    let result: i32 = lines.into_iter().map(|line| line.next(false)).sum();

    println!("Result: {:?} ({:?})", result, time.elapsed());
}

#[cfg(test)]
mod tests {
    use crate::Sequence;

    #[test]
    fn part_1() {
        let test = std::fs::read_to_string("test.txt").unwrap();
        let lines: Vec<Sequence> = test.lines().map(|line| line.into()).collect();

        let result: i32 = lines.into_iter().map(|line| line.next(true)).sum();

        assert_eq!(result, 114)
    }

    #[test]
    fn part_2() {
        let test = std::fs::read_to_string("test.txt").unwrap();
        let lines: Vec<Sequence> = test.lines().map(|line| line.into()).collect();

        let result: i32 = lines.into_iter().map(|line| line.next(false)).sum();

        assert_eq!(result, 2)
    }
}