use std::fs::File;
use std::io::Read;


#[derive(Clone, Debug)]
struct Point(i64, i64);

impl Point {
    fn mov(&self, dir: &Point) -> Point {
        let (x1, y1): (i64, i64) = (self.0, self.1);
        let (x2, y2): (i64, i64) = (dir.0, dir.1);
        Point(x1 + x2, y1 + y2)
    }
}

#[derive(Debug)]
struct Polygon {
    points: Vec<Point>,
    boundary: u64,
}

impl Polygon {
    fn build(file: &str) -> Result<Polygon, std::io::Error> {
        let mut file = File::open(file)?;
        let mut content =  String::new();
        file.read_to_string(&mut content)?;

        let lines = content.lines();
        let points_count = &lines.clone().count();

        let mut points: Vec<Point> = Vec::with_capacity(*points_count);
        points.push(Point(0,0));
        let mut boundary= 0;

        for line in lines {
            let point = Self::next_point(line, &points.last().unwrap()).unwrap();
            boundary += point.1;
            points.push(point.0);
        }

        Ok(Polygon { points, boundary })
    }

    fn next_point(line: &str, point: &Point) -> Result<(Point, u64), ()> {
        let dir = Self::parse_point(line).unwrap();
        Ok((point.mov(&dir.0), dir.1))
    }

    fn parse_point(line: &str) -> Result<(Point, u64), ()> {
        // part 1
        // let [dir, step] = line
        //     .split_whitespace()
        //     .take(2)
        //     .collect::<Vec<_>>()[..]
        //     else { panic!("wrong data in file") };

        let [_, _, hex] = line
            .split_whitespace()
            .take(3)
            .collect::<Vec<_>>()[..]
            else { panic!("wrong data in file") };

        let mut chars = hex.chars();
        chars.next();
        chars.next();
        chars.next_back();
        let dir = chars.next_back();
        let hex = chars.as_str();

        // part 1
        // let dir = match dir {
        //     "R" => Point(step.parse::<i64>().unwrap(), 0),
        //     "L" => Point(-1 * step.parse::<i64>().unwrap(), 0),
        //     "U" => Point(0, -1 * step.parse::<i64>().unwrap()),
        //     "D" => Point(0, step.parse::<i64>().unwrap()),
        //     _ => panic!("{dir}"),
        // };

        let step = i64::from_str_radix(hex, 16).unwrap();

        let dir = match dir {
            Some('0') => Point(step, 0),
            Some('2') => Point(-1 * step, 0),
            Some('3') => Point(0, -1 * step),
            Some('1') => Point(0, step),
            _ => panic!("{}", dir.unwrap()),
        };

        Ok((dir, step.try_into().unwrap()))
    }

    fn area(&self) -> u64 {
        let mut sum: i64 = 0;
        let points = self.points.clone();

        let mut i: usize = 0;
        while i < points.len() {
            let j = if (i + 1) == points.len() {
                0
            } else {
                i + 1
            };
            sum += points[i].0 * points[j].1 - points[i].1 * points[j].0;
            i += 1;
        }

        let geo_area: u64 = (sum.abs() / 2).try_into().unwrap();
        let pixel_area = geo_area - self.boundary / 2 + 1;
        pixel_area + self.boundary
    }
}

fn main() {
    let polygon = Polygon::build("input.txt").unwrap();
    let area = polygon.area();
    println!("{}", area);
}
