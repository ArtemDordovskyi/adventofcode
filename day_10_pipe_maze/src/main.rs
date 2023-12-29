#[derive(Clone, Debug)]
struct Point {
    char: char,
    coords: (i32, i32),
}

impl Point {
    fn next_point(&self, prev: Point) -> Option<(i32, i32)> {
        let coords = match self.char {
            '|' => {
                if self.coords.1 - prev.coords.1 > 0 {
                    (self.coords.0, self.coords.1 + 1)
                } else {
                    (self.coords.0, self.coords.1 - 1)
                }
            },
            '-' => {
                if self.coords.0 - prev.coords.0 > 0 {
                    (self.coords.0 + 1, self.coords.1)

                } else {
                    (self.coords.0 - 1, self.coords.1)
                }
            },
            'L' => {
                if self.coords.1 - prev.coords.1 > 0 {
                    (self.coords.0 + 1, self.coords.1)

                } else {
                    (self.coords.0, self.coords.1 - 1)
                }
            },
            'J' => {
                if self.coords.1 - prev.coords.1 > 0 {
                    (self.coords.0 - 1, self.coords.1)

                } else {
                    (self.coords.0, self.coords.1 - 1)
                }
            },
            '7' => {
                if self.coords.0 - prev.coords.0 > 0 {
                    (self.coords.0, self.coords.1 + 1)

                } else {
                    (self.coords.0 - 1, self.coords.1)
                }
            },
            'F' => {
                if self.coords.0 - prev.coords.0 < 0 {
                    (self.coords.0, self.coords.1 + 1)

                } else {
                    (self.coords.0 + 1, self.coords.1)
                }
            },
            _ => return None
        };

        Some(coords)
    }

    fn ways(&self, points: Vec<Point>) -> Vec<Point> {
        let mut coords = Vec::new();
        coords.push((self.coords.0 - 1, self.coords.1));
        coords.push((self.coords.0 + 1, self.coords.1));
        coords.push((self.coords.0, self.coords.1 + 1));
        coords.push((self.coords.0, self.coords.1 - 1));

        let points: Vec<Point> = points
            .into_iter()
            .filter(|p| coords.contains(&p.coords))
            .filter(|p|
                match p.char {
                    '|' => p.coords.1 != self.coords.1,
                    '-' => p.coords.0 != self.coords.0,
                    'L' => (p.coords.1 - self.coords.1) == 1 || (p.coords.0 - self.coords.0) == -1,
                    'J' => (p.coords.1 - self.coords.1) == 1 || (p.coords.0 - self.coords.0) == 1,
                    '7' => (p.coords.1 - self.coords.1) == -1 || (p.coords.0 - self.coords.0) == 1,
                    'F' => (p.coords.1 - self.coords.1) == -1 || (p.coords.0 - self.coords.0) == -1,
                    _ => false
                }
            ).collect();

        points
    }

    fn start(&self, points: Vec<Point>) -> usize {
        let mut ways = self.ways(points.clone());

        let mut result = 1;
        let mut previous_points = vec![self.clone(), self.clone()];
        while !ways.iter().all(|e| e.coords == ways[0].coords) {
            result += 1;
            let mut new_ways = Vec::new();
            for way in ways {
                let prev = previous_points.remove(0);
                let coords = way.next_point(prev).unwrap();
                let point = points.iter().find(|p| p.coords == coords).unwrap();
                new_ways.push(point.clone());
                previous_points.push(way);
            }
            ways = new_ways;
        }
        result
    }
}

fn main() {
    let time = std::time::Instant::now();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut lines: Vec<Point> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let chars = line.chars();
        for (j, char) in chars.enumerate() {
            lines.push(Point {
                char,
                coords: (j as i32, i as i32),
            })
        }
    }

    let start_point = lines.iter().find(|p| p.char == 'S');
    match start_point {
        Some(s_point) => {
            let point = s_point.clone();
            let result = point.start(lines);
            println!("Result {:?} ({:?})", result, time.elapsed())
        }
        _ => panic!("No start point")
    }
}

#[cfg(test)]
mod tests {
    use crate::Point;

    #[test]
    fn part_1() {
        let test = std::fs::read_to_string("test.txt").unwrap();
        let mut lines: Vec<Point> = Vec::new();

        for (i, line) in test.lines().enumerate() {
            let chars = line.chars();
            for (j, char) in chars.enumerate() {
                lines.push(Point {
                    char,
                    coords: (j as i32, i as i32),
                })
            }
        }

        let start_point = lines.iter().find(|p| p.char == 'S');
        match start_point {
            Some(s_point) => {
                let point = s_point.clone();
                let result = point.start(lines);
                assert_eq!(result, 8)
            }
            _ => panic!("No start point")
        }
    }
}