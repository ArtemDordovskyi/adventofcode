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

    fn distance(&self, points: &HashMap<Point, char>) -> usize {
        let poly = self.poly(points);
        if poly.len() % 2 == 0 {
            poly.len() / 2
        } else {
            (poly.len() + 1) / 2
        }
    }

    fn poly(&self, points: &HashMap<Point, char>) -> Vec<Point> {
        let mut way = self.ways(points.clone())[0].clone();
        let mut poly = Vec::new();
        poly.push(way.clone());
        let mut prev = self.clone();

        while &way != self {
            let char = points[&way];
            let next_point = way.next_point(prev.clone(), char).unwrap();
            prev = way.clone();
            way = next_point;
            poly.push(way.clone());
        }

        poly
    }

    //Ray casting algorithm
    fn is_point_in_path(&self, poly: &[Point]) -> bool {
        let mut j = poly.len() - 1;
        let mut c = false;
        for i in 0..poly.len() {
            if (poly[i].0 == self.0) && (poly[i].1 == self.1) {
                // point is a corner
                return true;
            }
            if (poly[i].1 > self.1) != (poly[j].1 > self.1) {
                let slope = (self.0 - poly[i].0) * (poly[j].1 - poly[i].1)
                    - (poly[j].0 - poly[i].0) * (self.1 - poly[i].1);
                if slope == 0 {
                    // point is on boundary
                    return true;
                }
                if (slope < 0) != (poly[j].1 < poly[i].1) {
                    c = !c;
                }
            }
            j = i;
        }
        c
    }
}

fn parse(input: String) -> HashMap<Point, char> {
    let mut points: HashMap<Point, char> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let chars = line.chars();
        for (j, char) in chars.enumerate() {
            points.insert(Point(j as i32, i as i32), char);
        }
    }

    points
}

fn main() {
    let time = std::time::Instant::now();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let points = parse(input);

    let (start_point, _) = points.clone().into_iter().find(|(_, c)| c == &'S').unwrap();
    let result = start_point.distance(&points);
    println!("Result {:?} ({:?})", result, time.elapsed());

    let poly = start_point.poly(&points);
    let checked: Vec<Point> = points
        .clone()
        .into_iter()
        .filter(|(p, _)| !poly.contains(p) )
        .map(|(p, _)| p)
        .filter(|point| point.is_point_in_path(&poly))
        .map(|p| p)
        .collect();

    println!("Result {:?} ({:?})", checked.len(), time.elapsed());
}

#[cfg(test)]
mod tests {
    use crate::{parse, Point};

    #[test]
    fn part_1() {
        let test = std::fs::read_to_string("test.txt").unwrap();
        let points = parse(test);

        let (start_point, _) = points.clone().into_iter().find(|(_, c)| c == &'S').unwrap();
        let result = start_point.distance(&points);

        assert_eq!(result, 8)
    }

    #[test]
    fn part_2() {
        let test = std::fs::read_to_string("test2.txt").unwrap();
        let points = parse(test);

        let (start_point, _) = points.clone().into_iter().find(|(_, c)| c == &'S').unwrap();

        let poly = start_point.poly(&points);
        let checked: Vec<Point> = points
            .clone()
            .into_iter()
            .filter(|(p, _)| !poly.contains(p) )
            .map(|(p, _)| p)
            .filter(|point| point.is_point_in_path(&poly))
            .map(|p| p)
            .collect();
        let result = checked.len();
        assert_eq!(result, 10)
    }
}