use num::integer::lcm;

#[derive(Debug)]
struct Network {
    key: String,
    value: (String, String)
}

impl From<String> for Network {
    fn from(value: String) -> Self {
        let mut line = value.to_string();
        line.retain(|c| !c.is_whitespace());
        let [key, val] = line
            .split('=')
            .take(2)
            .collect::<Vec<_>>()[..]
            else { panic!("wrong data in file") };

        let mut chars = val.chars();
        chars.next();
        chars.next_back();

        let [left, right] = chars
            .as_str()
            .split(",")
            .take(2)
            .collect::<Vec<_>>()[..]
        else { panic!("wrong data in file") };

        Self {
            key: key.to_string(),
            value: (left.to_string(), right.to_string())
        }
    }
}


fn main() {
    let time = std::time::Instant::now();
    let test = std::fs::read_to_string("input.txt").unwrap();
    let mut result: usize = 0;
    let mut lines: Vec<String> = test.lines()
        .map(|line| line.to_string())
        .collect();
    let rules = lines.remove(0);
    lines.remove(0);

    let network: Vec<Network> = lines.into_iter().map(|line| line.into()).collect();

    let start = network.iter().position(|n| &n.key == &"AAA".to_string()).unwrap();
    let mut key: &String = &network[start].key;
    while key != &"ZZZ".to_string() {
        for rule in rules.chars() {
            result += 1;

            let index = network.iter().position(|n| &n.key == key).unwrap();
            let keys = &network[index].value;
            key = if rule == 'L' {
                &keys.0
            } else {
               &keys.1
            };

            if key == &"ZZZ".to_string() {
                break
            }
        }
    }

    println!("Result: {:?} ({:?})", result, time.elapsed());


    let mut start_keys: Vec<String> = network
        .iter()
        .filter(|n| n.key.ends_with("A"))
        .map(|n| n.key.clone())
        .collect();


    let mut new_keys = Vec::new();
    for mut key in start_keys {
        let mut result = 0;
        while !key.ends_with('Z') {
            for rule in rules.chars() {
                result += 1;
                let index = network.iter().position(|n| n.key == key).unwrap();
                let keys = &network[index].value;
                key = if rule == 'L' {
                    keys.0.clone()
                } else {
                    keys.1.clone()
                };

                if key.ends_with('Z') {
                    break;
                }
            }
        }
        new_keys.push(result);
    }

    let mut result: i64 = new_keys[0].clone();
    for i in 1..new_keys.len() {
        result = lcm(result, new_keys[i].clone())
    }

    println!("Result: {:?} ({:?})", result, time.elapsed());
}

#[cfg(test)]
mod tests {
    use crate::Network;

    #[test]
    fn part_1() {
        let test = std::fs::read_to_string("test.txt").unwrap();
        let mut result: usize = 0;
        let mut lines: Vec<String> = test.lines()
            .map(|line| line.to_string())
            .collect();
        let rules = lines.remove(0);
        lines.remove(0);

        let network: Vec<Network> = lines.into_iter().map(|line| line.into()).collect();

        let mut key: &String = &network[0].key;
        while key != &"ZZZ".to_string() {
            for rule in rules.chars() {
                result += 1;

                let index = network.iter().position(|n| &n.key == key).unwrap();
                let keys = &network[index].value;
                key = if rule == 'L' {
                    &keys.0
                } else {
                    &keys.1
                };

                if key == &"ZZZ".to_string() {
                    break
                }
            }
        }


        assert_eq!(result, 2)
    }

    #[test]
    fn part_2() {
        let test = std::fs::read_to_string("test2.txt").unwrap();
        let mut lines: Vec<String> = test.lines()
            .map(|line| line.to_string())
            .collect();
        let rules = lines.remove(0);
        lines.remove(0);

        let network: Vec<Network> = lines.into_iter().map(|line| line.into()).collect();


        let mut result = 0;
        let mut start_keys: Vec<String> = network
            .iter()
            .filter(|n| n.key.ends_with("A"))
            .map(|n| n.key.clone())
            .collect();

        let mut rule = 'L';
        let mut new_keys = Vec::new();
        while start_keys.len() > 0 {
            result += 1;
            let mut next_keys = Vec::new();
            for key in start_keys {
                let net_index = network.iter().position(|n| n.key == key).unwrap();
                let keys = &network[net_index].value;
                let key = if rule == 'L' {
                    &keys.0
                } else {
                    &keys.1
                };
                if key.ends_with('Z') {
                    new_keys.push(result);
                    continue;
                }
                next_keys.push(key.clone());
            }
            start_keys = next_keys;
            rule = if rule == 'L' { 'R' } else { 'L' }
        }

        println!("{:?}", new_keys);



        assert_eq!(result, 6)
    }
}