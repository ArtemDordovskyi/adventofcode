#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn winners(&self, from: usize) -> usize {
        let mut winners = 0;

        for i in from..(self.time - from + 1) {
            let distance = (self.time - i) * i;
            if distance > self.distance {
                winners += 1;
            }
        }

        winners
    }

    fn new(content: String) -> Self {
        let mut time: usize = 0;
        let mut distance: usize = 0;

        for (i, line) in content.lines().enumerate() {
            let mut line = line.to_string();
            line.retain(|c| !c.is_whitespace());
            let number = line
                .split(':')
                .collect::<Vec<_>>()[1]
                .parse::<usize>()
                .unwrap();
            if i == 0 {
                time = number;
            } else {
                distance = number;
            }
        }
        Self {
            time,
            distance
        }
    }
}

fn parse_1(content: String) -> Vec<Race> {
    let mut times: Vec<usize> = Vec::new();
    let mut distances: Vec<usize> = Vec::new();

    for (i, line) in content.lines().enumerate() {
        let numbers = line
            .split_whitespace()
            .filter_map(|x| match x.parse::<usize>() {
                Ok(n) => Some(n),
                _ => None
            })
            .collect();
        if i == 0 {
            times = numbers;
        } else {
            distances = numbers;
        }
    }

    let mut races: Vec<Race> = Vec::new();
    for (i, time) in times.into_iter().enumerate() {
        races.push(Race {
            time,
            distance: distances[i],
        });
    }

    races
}

fn main() {
    let time = std::time::Instant::now();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let races = parse_1(input.clone());

    let result: usize = races
        .iter()
        .map(|race| race.winners(0))
        .product();

    println!("Races: {:?}, ({:?})", result, time.elapsed());

    let race = Race::new(input);
    let result = race.winners(14);

    println!("Races: {:?}, ({:?})", result, time.elapsed());
}

#[cfg(test)]
mod tests {
    use crate::{parse_1, Race};

    #[test]
    fn part_1() {
        let test = std::fs::read_to_string("test.txt").unwrap();
        let races = parse_1(test);

        let result: usize = races
            .iter()
            .map(|race| race.winners(0))
            .product();

        assert_eq!(result, 288)
    }

    #[test]
    fn part_2() {
        let test = std::fs::read_to_string("test.txt").unwrap();
        let race = Race::new(test);
        println!("{:?}", race);
        let result = race.winners(14);

        assert_eq!(result, 71503)
    }
}