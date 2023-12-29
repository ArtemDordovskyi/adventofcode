use std::collections::HashMap;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point(i32, i32);

impl Point {
    fn next_point(&self, prev: Point, char: char) -> Option<Point> {
        let coords = match char {
            '|' => {
                if self.1 - prev.1 > 0 {
                    (self.0, self.1 + 1)
                } else {
                    (self.0, self.1 - 1)
                }
            },
            '-' => {
                if self.0 - prev.0 > 0 {
                    (self.0 + 1, self.1)

                } else {
                    (self.0 - 1, self.1)
                }
            },
            'L' => {
                if self.1 - prev.1 > 0 {
                    (self.0 + 1, self.1)

                } else {
                    (self.0, self.1 - 1)
                }
            },
            'J' => {
                if self.1 - prev.1 > 0 {
                    (self.0 - 1, self.1)

                } else {
                    (self.0, self.1 - 1)
                }
            },
            '7' => {
                if self.0 - prev.0 > 0 {
                    (self.0, self.1 + 1)

                } else {
                    (self.0 - 1, self.1)
                }
            },
            'F' => {
                if self.0 - prev.0 < 0 {
                    (self.0, self.1 + 1)

                } else {
                    (self.0 + 1, self.1)
                }
            },
            _ => return None
        };

        Some(Point(coords.0, coords.1))
    }

    fn ways(&self, points: HashMap<Point, char>) -> Vec<Point> {
        let mut coords = Vec::new();
        coords.push(Point(self.0 - 1, self.1));
        coords.push(Point(self.0 + 1, self.1));
        coords.push(Point(self.0, self.1 + 1));
        coords.push(Point(self.0, self.1 - 1));

        let points: Vec<Point> = points
            .into_iter()
            .filter(|(p, _)| coords.contains(p))
            .filter(|(p, char)|
                match char {
                    '|' => p.1 != self.1,
                    '-' => p.0 != self.0,
                    'L' => (p.1 - self.1) == 1 || (p.0 - self.0) == -1,
                    'J' => (p.1 - self.1) == 1 || (p.0 - self.0) == 1,
                    '7' => (p.1 - self.1) == -1 || (p.0 - self.0) == 1,
                    'F' => (p.1 - self.1) == -1 || (p.0 - self.0) == -1,
                    _ => false
                }
            )
            .map(|(p, _)| p)
            .collect();

        points
    }

    fn start(&self, points: HashMap<Point, char>) -> usize {
        let mut ways = self.ways(points.clone());

        let mut result = 1;
        let mut previous_points = vec![self.clone(), self.clone()];
        while !ways.iter().all(|e| e == &ways[0]) {
            result += 1;
            let mut new_ways = Vec::new();
            for way in ways {
                let prev = previous_points.remove(0);
                let char = points[&way];
                let coords: Point = way.next_point(prev, char).unwrap();
                new_ways.push(coords.clone());
                previous_points.push(way.clone());
            }
            ways = new_ways;
        }
        result
    }
}

fn main() {
    let time = std::time::Instant::now();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut points: HashMap<Point, char> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let chars = line.chars();
        for (j, char) in chars.enumerate() {
            points.insert(Point(j as i32, i as i32), char);
        }
    }

    let (start_point, _) = points.clone().into_iter().find(|(_, c)| c == &'S').unwrap();
    let result = start_point.start(points);
    println!("Result {:?} ({:?})", result, time.elapsed())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::Point;

    #[test]
    fn part_1() {
        let test = std::fs::read_to_string("test.txt").unwrap();
        let mut points: HashMap<Point, char> = HashMap::new();

        for (i, line) in test.lines().enumerate() {
            let chars = line.chars();
            for (j, char) in chars.enumerate() {
                points.insert(Point(j as i32, i as i32), char);
            }
        }

        let (start_point, _) = points.clone().into_iter().find(|(_, c)| c == &'S').unwrap();
        let result = start_point.start(points);
        assert_eq!(result, 8)
    }
}