use ndarray::prelude::*;
use ndarray_linalg::Solve;

#[derive(Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

// Used to parse the input.
impl From<&str> for Point {
    fn from(input: &str) -> Self {
        let mut split = input.split(',');
        let x = split.next().unwrap().trim().parse().unwrap();
        let y = split.next().unwrap().trim().parse().unwrap();
        let z = split.next().unwrap().trim().parse().unwrap();
        Self { x, y, z }
    }
}

#[derive(Debug, PartialEq)]
struct Speed {
    x: f64,
    y: f64,
    z: f64,
}

impl From<&str> for Speed {
    fn from(input: &str) -> Self {
        let mut split = input.split(',');
        let x = split.next().unwrap().trim().parse().unwrap();
        let y = split.next().unwrap().trim().parse().unwrap();
        let z = split.next().unwrap().trim().parse().unwrap();
        Self { x, y, z }
    }
}

#[derive(Debug, PartialEq)]
struct Coef {
    a: f64,
    b: f64,
    c: f64,
}
#[derive(Debug, PartialEq)]
struct Hailstone {
    point: Point,
    v: Speed,
    coef: Coef,
    coef2d: Coef
}

impl From<&str> for Hailstone {
    fn from(input: &str) -> Self {
        let mut split = input.split(" @ ");
        let point: Point = split.next().unwrap().into();
        let v: Speed = split.next().unwrap().into();

        let coef = Coef {
            a: point.y * v.z - point.z * v.y,
            b: point.z * v.x - point.x * v.z,
            c: point.x * v.y - point.y * v.x
        };

        let coef2d = Coef {
            a: v.y,
            b: - 1.0 * v.x,
            c: point.x * v.y - point.y * v.x
        };

        Self { point, v, coef, coef2d }
    }
}

#[derive(Clone, Debug)]
struct Point2d {
    x: f64,
    y: f64
}

impl Point2d {
    fn is_inside(&self, square: &Square) -> bool {
        self.x >= square.tl.x
            && self.y >= square.tl.y
            && self.x <= square.br.x
            && self.y <= square.br.y
    }
}

struct Square {
    tl: Point2d,
    br: Point2d,
}

impl Hailstone {
    fn is_positive_speed(&self, point2d: &Point2d) -> bool {
        (point2d.x - self.point.x) * self.v.x >= 0.0
            && (point2d.y - self.point.y) * self.v.y >= 0.0
    }

    fn is_parallel(&self, hailstone: &Hailstone) -> bool {
        self.coef2d.a * hailstone.coef2d.b == self.coef2d.b * hailstone.coef2d.a
    }

    fn intersections2d(&self, hailstones: &[Hailstone]) -> Vec<Point2d> {
        let hailstones = hailstones
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| i > hailstones.iter().position(|x| x == self).unwrap())
            .map(|(_, h)| h);

        let mut intersections: Vec<Point2d> = Vec::new();

        for hailstone in hailstones {
            if self.is_parallel(hailstone) {
                continue;
            }

            let x = (self.coef2d.c * hailstone.coef2d.b - self.coef2d.b * hailstone.coef2d.c)
                / (self.coef2d.a * hailstone.coef2d.b - self.coef2d.b * hailstone.coef2d.a);

            let y = (self.coef2d.a * hailstone.coef2d.c - self.coef2d.c * hailstone.coef2d.a)
                / (self.coef2d.a * hailstone.coef2d.b - self.coef2d.b * hailstone.coef2d.a);

            let point = Point2d {
                x,
                y,
            };

            if self.is_positive_speed(&point) && hailstone.is_positive_speed(&point) {
                intersections.push(point);
            }
        }

        intersections
    }
}

fn main() {
    let now = std::time::Instant::now();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut hailstones: Vec<Hailstone> = input.lines().map(|line| line.into()).collect();

    let tl = Point2d { x: 200000000000000.0, y: 200000000000000.0 };
    let br = Point2d { x: 400000000000000.0, y: 400000000000000.0 };
    let square = Square { tl, br };

    let count = hailstones
        .iter()
        .map(|h| h.intersections2d(&hailstones))
        .flatten()
        .filter(|p| p.is_inside(&square))
        .count();

    println!("Intersection: {} ({:?})", count, now.elapsed());
    let mut i: f64 = 0.0;
    let mut sum: f64 = 0.0;
    while hailstones.len() > 2 {
        let stone = hailstones.remove(0);
        let v0 = stone.v;
        let p0 = stone.point;

        let stone = hailstones.remove(0);
        let v1 = stone.v;
        let p1 = stone.point;

        let stone = hailstones.remove(0);
        let v2 = stone.v;
        let p2 = stone.point;

        let a: Array2<f64> = array![
        [v1.y - v0.y, v0.x - v1.x, 0.0, p0.y - p1.y, p1.x - p0.x, 0.0],
        [v2.y - v0.y, v0.x - v2.x, 0.0, p0.y - p2.y, p2.x - p0.x, 0.0],
        [v1.z - v0.z, 0.0, v0.x - v1.x, p0.z - p1.z, 0.0, p1.x - p0.x],
        [v2.z - v0.z, 0.0, v0.x - v2.x, p0.z - p2.z, 0.0, p2.x - p0.x],
        [0.0, v1.z - v0.z, v0.y - v1.y, 0.0, p0.z - p1.z, p1.y - p0.y],
        [0.0, v2.z - v0.z, v0.y - v2.y, 0.0, p0.z - p2.z, p2.y - p0.y],
    ];

        let b: Array1<f64> = array![
        (p0.y * v0.x - p1.y * v1.x) - (p0.x * v0.y - p1.x * v1.y),
        (p0.y * v0.x - p2.y * v2.x) - (p0.x * v0.y - p2.x * v2.y),
        (p0.z * v0.x - p1.z * v1.x) - (p0.x * v0.z - p1.x * v1.z),
        (p0.z * v0.x - p2.z * v2.x) - (p0.x * v0.z - p2.x * v2.z),
        (p0.z * v0.y - p1.z * v1.y) - (p0.y * v0.z - p1.y * v1.z),
        (p0.z * v0.y - p2.z * v2.y) - (p0.y * v0.z - p2.y * v2.z),
    ];

        let rock = a.solve_into(b).unwrap();
        sum += rock[0] + rock[1] + rock[2];
        i += 1.0;
        println!("Rock: {} ({:?})", rock[0] + rock[1] + rock[2], now.elapsed());
    }

    println!("Rock: {} ({:?})", sum/i, now.elapsed());
}


#[cfg(test)]
mod tests {
    use crate::Hailstone;
    use crate::Point2d;
    use crate::Square;

    #[test]
    fn intersections() {
        let input = std::fs::read_to_string("test.txt").unwrap();
        let hailstones: Vec<Hailstone> = input.lines().map(|line| line.into()).collect();

        let tl = Point2d { x: 7.0, y: 7.0 };
        let br = Point2d { x: 27.0, y: 27.0 };
        let square = Square { tl, br };

        let count = hailstones
            .iter()
            .map(|h| h.intersections2d(&hailstones))
            .flatten()
            .filter(|p| p.is_inside(&square))
            .count();

        assert_eq!(count, 2);
    }
}