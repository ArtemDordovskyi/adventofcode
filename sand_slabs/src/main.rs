use std::fs::File;
use std::io::Read;

#[derive(Clone, Debug, PartialEq)]
struct Brick {
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

impl Brick {
    fn intersect(&self, b: &Brick) -> bool {
        self.start.0.max(b.start.0) <= self.end.0.min(b.end.0)
            && self.start.1.max(b.start.1) <= self.end.1.min(b.end.1)
    }

    fn max_z(&self, bricks: &[Brick]) -> usize {
        bricks
            .iter()
            .filter(|&b| b != self)
            .filter(|&b| self.intersect(b))
            .map(|b| b.end.2)
            .max()
            .unwrap_or(0)
    }

    fn support(&self, bricks: &[Brick]) -> Vec<Brick> {
        bricks
            .iter()
            .filter(|&b| self != b && self.intersect(b) && self.end.2 == (b.start.2 - 1))
            .cloned()
            .collect()
    }

    fn supported(&self, bricks: &[Brick]) -> Vec<Brick> {
        bricks
            .iter()
            .filter(|&b| self != b && self.intersect(b) && self.start.2 == (b.end.2 + 1))
            .cloned()
            .collect()
    }

    fn is_removable(&self, bricks: &[Brick]) -> bool {
        let support = self.support(bricks);
        if support.is_empty() {
            return true;
        }

        let to_check = bricks
            .iter()
            .filter(|&b| b != self)
            .cloned()
            .collect::<Vec<_>>();

        for support in support.iter() {
            let supported = support.supported(&to_check);
            if supported.is_empty() {
                return false;
            }
        }

        true
    }

}

#[derive(Debug)]
struct Cube {
    bricks: Vec<Brick>,
}

impl Cube {
    fn new(file: &str) -> Result<Cube, std::io::Error> {
        let mut file = File::open(file)?;
        let mut content =  String::new();
        file.read_to_string(&mut content)?;

        let lines = content.lines();

        let mut bricks: Vec<Brick> = Vec::new();

        for line in lines {
            let brick_vec: Vec<_> = line
                .split("~")
                .map(|point| {
                    let [x,y,z] = point
                        .split(",")
                        .take(3)
                        .collect::<Vec<_>>()[..]
                        else { panic!("wrong data in file") };
                    (x.parse::<usize>().unwrap(),
                     y.parse::<usize>().unwrap(),
                     z.parse::<usize>().unwrap())
                })
                .collect();

            let brick = Brick {
                start: brick_vec[0],
                end: brick_vec[1],
            };

            bricks.push(brick);
        }

        bricks.sort_by_key(|k| k.start.2);

        Ok(Cube {
            bricks,
        })
    }

    fn dropped(&mut self) -> Cube {
        let mut dropped: Vec<Brick> = Vec::new();

        for block in self.bricks.iter_mut() {
            let max_z = block.max_z(&dropped);

            let diff = block.end.2 - block.start.2;
            block.start.2 = max_z + 1;
            block.end.2 = block.start.2 + diff;
            dropped.push(block.clone());
        }

        Cube {
            bricks: dropped,
        }
    }

    fn removable(&self) -> Vec<Brick> {
        self
            .bricks
            .clone()
            .into_iter()
            .filter(|b| b.is_removable(&self.bricks))
            .collect::<Vec<Brick>>()
    }
}

fn main() {
    let now = std::time::Instant::now();

    let mut cube = Cube::new("input.txt").unwrap();

    let len = cube
        .dropped()
        .removable()
        .len();
    println!("len: {} ({:?})", len, now.elapsed());
}

#[cfg(test)]
mod tests {
    use crate::Cube;

    #[test]
    fn removable() {
        let mut cube = Cube::new("test.txt").unwrap();

        let count = cube
            .dropped()
            .removable()
            .len();
        assert_eq!(count, 5);
    }
}