#[derive(Debug)]
struct Reflection {
    index: usize,
    value: usize,
}

#[derive(Debug)]
struct Note {
    grid: Vec<Vec<char>>,
    transpose: Vec<Vec<char>>,
}

impl From<&str> for Note {
    fn from(value: &str) -> Self {
        let grid: Vec<Vec<char>> = value.lines().map(|line| line.chars().collect()).collect();

        let n = grid.len();
        let m = grid[0].len();

        let mut transpose: Vec<Vec<char>> = vec![vec!['.'; n]; m];
        for (i, line) in value.lines().enumerate() {
            for (j, char) in line.char_indices() {
                transpose[j][i] = char;
            }
        }

        Self { grid, transpose }
    }
}

impl Note {
    fn find_reflection(&self) -> usize {
        let indices = &[true, false].map(|dir| self.get_index(dir));

        if indices[1].value > indices[0].value {
            (indices[1].index + 1) * 100
        } else {
            indices[0].index + 1
        }
    }

    fn get_index(&self, dir: bool) -> Reflection {
        let mut dirv = if dir {
            self.transpose.clone()
        } else {
            self.grid.clone()
        };
        let windows: Vec<Vec<Vec<char>>> = dirv.windows(2).map(|w| w.to_vec()).collect::<Vec<_>>();

        let indices: Vec<usize> = windows
            .into_iter()
            .enumerate()
            .filter_map(|(i, rows)| if rows[0] == rows[1] { Some(i) } else { None })
            .collect();

        let mut index = 0;
        let mut value = 0;
        for i in indices {
            let mut diff = 0;
            let mirror = i.min(dirv.len() - i - 2);
            for m in 1..=mirror {
                diff += self.rows_diff(dir, i - m, i + m + 1);
            }
            println!("index: {:?} diff: {:?}, mirror: {:?}", i, diff, mirror);
            if diff > 1 {
                continue;
            }
            if mirror > value {
                value = mirror;
                index = i;
            }
        }

        Reflection { index, value }
    }

    fn rows_diff(&self, dir: bool, y1: usize, y2: usize) -> usize {
        let mut dirv = if dir {
            self.transpose.clone()
        } else {
            self.grid.clone()
        };
        let mut delta = 0;

        for x in 0..dirv[y1].len() {
            if dirv[y1][x] != dirv[y2][x] {
                delta += 1;
            }
        }
        delta
    }
}

fn main() {
    let time = std::time::Instant::now();
    let input = std::fs::read_to_string("input").unwrap();
    let notes: Vec<Note> = input.split("\n\n").map(|note| note.into()).collect();
    let res: Vec<usize> = notes.iter().map(|note| note.find_reflection()).collect();

    println!("Result: {:?} ({:?})", res, time.elapsed());

    let res: usize = res.iter().sum();

    println!("Result: {:?} ({:?})", res, time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = std::fs::read_to_string("test").unwrap();
        let notes: Vec<Note> = input.split("\n\n").map(|note| note.into()).collect();
        let res: usize = notes.iter().map(|note| note.find_reflection()).sum();

        assert_eq!(res, 405)
    }
}
