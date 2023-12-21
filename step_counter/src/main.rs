use std::collections::{HashMap};
use std::fs::File;
use std::io::Read;

#[derive(Clone, Debug)]
struct Counter {
    even: HashMap<(i64, i64), bool>,
    odd: HashMap<(i64, i64), bool>,
    garden: HashMap<(i64, i64), bool>,
    size: (i64, i64),
    step: i64,
    odd_len: i64,
    even_len: i64,
}

impl Counter {
    fn new(file: &str) -> Result<Counter, std::io::Error> {
        let mut file = File::open(file)?;
        let mut content =  String::new();
        file.read_to_string(&mut content)?;

        let mut odd: HashMap<(i64, i64), bool> = HashMap::new();
        let mut garden: HashMap<(i64, i64), bool> = HashMap::new();
        let mut size= (0,0);

        let lines = content.lines();
        size.1 = lines.clone().count() as i64;
        for (y, line) in lines.into_iter().enumerate() {
            let chars = line.chars();
            if size.0 == 0 {
                size.0 = chars.clone().count() as i64;
            }
            for (x, char) in chars.into_iter().enumerate() {
                garden.insert((x as i64, y as i64), char != '#');
                if char == 'S' {
                    odd.insert((x as i64, y as i64), true);
                }
            }
        }


        Ok(Counter {
            even: HashMap::new(),
            odd,
            garden,
            size,
            step: 0,
            odd_len: 1,
            even_len: 0,
        })
    }

    fn step(&mut self, step: i64) -> Counter {
        println!("Step: {step}");
        if step == 0 {
            return self.clone();
        }

        let os = if step % 2 == 0 {
            self.clone().odd
        } else {
            self.clone().even
        };

        // println!("Even len: {:?}", self.even_len);
        // println!("Odd len: {:?}", self.odd_len);


        let dirs = [(-1,0), (0, -1), (1, 0), (0, 1)];

        for (o, _) in os.clone().iter().filter(|(_, v)| **v) {
            for dir in dirs {
                let new_o = (o.0 + dir.0, o.1 + dir.1);
                let mut key_x = (o.0 + dir.0) % self.size.0;
                let mut key_y = (o.1 + dir.1) % self.size.1;
                if key_x < 0 {
                    key_x = self.size.0 + key_x
                }
                if key_y < 0 {
                    key_y = self.size.0 + key_y
                }

                if *self.garden.get(&(key_x, key_y)).unwrap()  {
                    if step % 2 == 0 {
                        if self.even.get(&new_o) == None {
                            self.even.insert(new_o, true);
                            self.even_len += 1;
                        }
                    } else {
                        if self.odd.get(&new_o) == None {
                            self.odd.insert(new_o, true);
                            self.odd_len += 1;
                        }
                    }
                }
            }
        }

        if step % 2 == 0 {
            let mut os = self.odd.clone();
            os.retain(|_, &mut v| v);

            for (_,v) in os.iter_mut()  {
                *v = false;
            }

            // println!("{:?}", os.len());

            self.odd = os;
        } else {
            let mut os = self.even.clone();
            os.retain(|_, &mut v| v);

            for (_,v) in os.iter_mut()  {
                *v = false;
            }

            // println!("{:?}", os.len());

            self.even = os;
        }

        if self.step == 0 {
            self.step = step;
        }

        let step = step.clone() - 1;
        self.step(step)
    }

    fn len(&self) -> i64 {
        if self.step % 2 == 0 {
            self.odd_len
        } else {
            self.even_len
        }
    }

}

fn main() {
    let mut counter = Counter::new("input.txt").unwrap();
    let counter = counter.step(26501365);
    let count = counter.len();
    println!("{:?}", count);
}

#[cfg(test)]
mod tests {
    use crate::Counter;

    #[test]
    fn part1_count() {
        let mut counter = Counter::new("test.txt").unwrap();
        let counter = counter.step(6);
        let count = counter.len();
        assert_eq!(count, 16);
    }

    #[test]
    fn part2_count_500() {
        let mut counter = Counter::new("test.txt").unwrap();
        let counter = counter.step(500);
        let count = counter.len();
        assert_eq!(count, 167004);
    }

    #[test]
    fn part2_count_5000() {
        let mut counter = Counter::new("test.txt").unwrap();
        let counter = counter.step(5000);
        let count = counter.len();
        assert_eq!(count, 16733044);
    }
}