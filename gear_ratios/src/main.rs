use std::fs::File;
use std::io::Read;


#[derive(Clone, Debug, PartialEq)]
struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn adjacent(self: &Self, coords: &Vec<Self>) -> bool {
        coords.into_iter().any(|coord| {
            (coord.x as i32 - self.x  as i32).abs() < 2
                && (coord.y as i32 - self.y  as i32).abs() < 2
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Symbol {
    val: char,
    coord: Coord,
}

#[derive(Clone, Debug, PartialEq)]
struct Number {
    val: u32,
    coords: Vec<Coord>
}

impl Number {
    fn new() -> Self {
        Number {
            val: 0,
            coords: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Product {
    nums: Vec<u32>,
    symbol: Symbol,
}

fn is_sym(char: char) -> bool {
    char.is_ascii_punctuation() && char.to_string() != "."
}

fn main() {
    let input = File::open("input.txt");
    let mut content =  String::new();

    let _ = input.expect("File not found").read_to_string(&mut content);
    let lines = content.lines();

    let mut symbols: Vec<Symbol> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();
    let mut products: Vec<Product> = Vec::new();

    let mut sum = 0;

    for (i, line) in lines.enumerate() {
        let mut num = String::new();
        let mut number = Number::new();

        for (j, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                num.push_str(&char.to_string());
                number.coords.push(Coord {
                    x: j as u32,
                    y: i as u32,
                });
            } else {
                if is_sym(char) {
                    symbols.push(Symbol {
                        val: char,
                        coord: Coord {
                            x: j as u32,
                            y: i as u32,
                        }
                    });
                }

                number.val = num.parse::<u32>().unwrap_or_else(|_| 0);

                if number.val > 0 {
                    numbers.push(number);
                    number = Number::new();
                }

                num = String::new();
            }
        }

        number.val = num.parse::<u32>().unwrap_or_else(|_| 0);

        if number.val > 0 {
            numbers.push(number);
        }
    }

    let mut numbers_clone = numbers.clone();

    // part 1
    while numbers.len() > 0 {
        let number = numbers.pop().unwrap();
        for symbol in &symbols {
            let coord = &symbol.coord;
            if coord.adjacent(&number.coords) {
                sum += number.val;
                break;
            }
        }
    }


    // part 2
    while numbers_clone.len() > 0 {
        let number = numbers_clone.pop().unwrap();
        for symbol in &symbols {
            let coord = &symbol.coord;
            if coord.adjacent(&number.coords) {
                for product in &mut products {
                    if product.symbol == *symbol {
                        product.nums.push(number.val);
                    }
                }

                if !products
                    .iter()
                    .any(|x| x.symbol == *symbol ) {
                    products.push(Product {
                        nums: vec![number.val],
                        symbol: symbol.clone(),
                    });
                }

                break;
            }
        }
    }

    let sum2: u32 = products
        .into_iter()
        .filter(|x| x.nums.len() > 1)
        .map(|x| x.nums.iter().product::<u32>())
        .sum();

    println!("Sum: {:?}", sum);
    println!("Sum: {:?}", sum2);
}
