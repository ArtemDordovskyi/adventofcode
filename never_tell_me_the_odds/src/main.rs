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
    let hailstones: Vec<Hailstone> = input.lines().map(|line| line.into()).collect();

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