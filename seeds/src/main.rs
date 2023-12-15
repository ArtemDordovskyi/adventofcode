use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Map<'a> {
    key: u64,
    value: u64,
    range: u64,
    name: &'a str,
}

impl Map<'_> {
    fn parse_name(map_name: &str) -> &str {
        map_name
            .split_whitespace()
            .collect::<Vec<_>>()
            .first()
            .expect("not a string")
            .split("-")
            .last()
            .unwrap()
    }

    fn parse_map<'a>(data: &'a mut Vec<&'a str>, name: &'a str) -> Vec<Map<'a>> {
        let mut maps: Vec<_> = Vec::new();

        while data.len() > 0 {
            let map: Vec<_> = data
                .remove(0)
                .split_whitespace()
                .collect::<Vec<_>>()
                .into_iter()
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

            let key = map[1];
            let value = map[0];
            let range = map[2];

            maps.push(Map {
                key,
                value,
                range,
                name,
            });
        }

        maps
    }

    fn mov(maps: Vec<Map>, seeds: Vec<u64>) -> Vec<u64> {
        let seeds =  seeds.clone();
        let len = seeds.clone().len();
        let mut new_seeds: Vec<u64> = vec![0; len];

        for map in maps {
            for (i, key) in seeds.clone().into_iter().enumerate() {
                if map.key <= key && (map.key + map.range) > key {
                    if new_seeds[i] == 0 {
                        new_seeds[i] = map.value + key - map.key;
                    }
                }
            }
        }

        for (i, key) in seeds.clone().into_iter().enumerate() {
            if new_seeds[i] == 0 {
                new_seeds[i] = key;
            }
        }

        new_seeds
    }

    fn parse(data: &mut Vec<Vec<&str>>, seeds: &mut Vec<u64>) -> Vec<u64> {
        while data.len() > 0 {
            let mut current: Vec<_> = data.remove(0);
            let name: &str = Map::parse_name(current.remove(0));

            let maps: Vec<Map> = Map::parse_map(&mut current, name);
            *seeds = Map::mov(maps, seeds.clone());

            println!("{:?}", name);
            println!("{:?}", seeds);
        }
        let seeds = seeds.clone();
        seeds
    }
}

fn main() {
    let input = File::open("input.txt");
    let mut content =  String::new();

    let _ = input.expect("File not found").read_to_string(&mut content);
    let lines = content.lines();
    let last_index = lines.clone().count() - 1;

    let mut data_arr: Vec<_> = Vec::new();
    let mut data: Vec<_> = Vec::new();

    for line in lines.clone().into_iter() {
        if line.len() > 0 {
            data.push(line);
            if lines.clone().collect::<Vec<_>>()[last_index] == line {
                data_arr.push(data.clone());
            }
        } else {
            data_arr.push(data.clone());
            data = Vec::new();
        }
    }

    let first_seeds = data_arr.remove(0);
    let seeds = first_seeds.first();
    let mut seeds: Vec<_> = seeds
        .expect("not a vecvaluer")
        .split(": ")
        .collect::<Vec<_>>()
        .last()
        .expect("not a string")
        .split_whitespace()
        .into_iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let data = Map::parse(&mut data_arr, &mut seeds);
    let min = data.iter().min().unwrap();
    println!("{}", min);
}
