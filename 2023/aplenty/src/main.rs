use evalexpr::*;
use evalexpr::Value::Int;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;


#[derive(Debug)]
struct Aplenty {
    input: Vec<HashMapContext>,
    tree: HashMap<String, String>,
}

#[derive(Debug)]
struct Leaf {
    context: String,
    sign: String,
    value: i64,
    key: String,
}

impl Aplenty {
    fn build(file: &str) -> Result<Aplenty, std::io::Error> {
        let mut file = File::open(file)?;
        let mut content =  String::new();
        file.read_to_string(&mut content)?;

        let mut lines = content.lines();
        let mut tree: HashMap<String, String> = HashMap::new();
        let mut input: Vec<HashMapContext> = Vec::new();

        while let Some(line) = lines.next() {
            if line == "" { break; }
            let (k, v): (&str, &str) = Aplenty::parse_tree(line);
            tree.insert(k.to_string(), v.to_string());
        }

        while let Some(line) = lines.next_back() {
            if line == "" { break; }
            let xmas = Aplenty::parse_xmas(line);
            input.push(xmas);
        }

        Ok(Aplenty { input, tree })
    }

    fn parse_tree(line: &str) -> (&str, &str) {
        let [k, v] = line
            .split("{")
            .take(2)
            .collect::<Vec<_>>()[..]
            else { panic!("wrong data in file") };

        let mut chars = v.chars();
        chars.next_back();
        let v = chars.as_str();

        (k, v)
    }

    fn parse_xmas(line: &str) -> HashMapContext {
        let mut chars = line.chars();
        chars.next();
        chars.next_back();
        let line = chars.as_str();
        let mut xmas = HashMapContext::new();

        let vec: Vec<_> = line
            .split(",")
            .collect();

        for v in vec {
            eval_with_context_mut(v, &mut xmas).unwrap();
        }

        xmas
    }

    fn sum(&self) -> i64 {
        let mut sum = 0;
        let mut expr = self.get_expression("in".to_string());

        for xmas in self.input.clone() {
            let mut key = Aplenty::next_key(expr.clone(), xmas.clone());

            while key != "A".to_string() && key != "R".to_string() {
                expr =  self.get_expression(key.clone());
                key = Aplenty::next_key(expr.clone(), xmas.clone());
            }

            if key == "A".to_string() {
                for (_, v) in xmas.clone().iter_variables() {
                    let Int(val) = v else { panic!("not an integer") };
                    sum += val;
                }

            }
            expr = self.get_expression("in".to_string());
        }

        sum
    }

    fn get_expression(&self, key: String) -> Vec<Vec<String>> {
        let input = self.tree.get(&key).unwrap();
        let exprs: Vec<String> = input
            .split(",")
            .map(|expr| expr.to_string())
            .collect();

        let mut exprss: Vec<Vec<String>> = Vec::new();

        for expr in exprs {
            let expr: Vec<String> = expr
                .split(":")
                .map(|expr| expr.to_string())
                .collect();
            exprss.push(expr);
        }

        exprss
    }

    fn next_key(exprs: Vec<Vec<String>>, context: HashMapContext) -> String {
        let mut exprs = exprs.clone();
        let len = exprs.clone().len();
        let mut next_key = exprs.remove(len - 1);
        let mut next_key = next_key.remove(0);

        for expr in exprs {
            let exp = &expr[0];
            let key = &expr[1];

            let precompiled = build_operator_tree(exp).unwrap();
            if precompiled.eval_boolean_with_context(&context).unwrap() {
                next_key = key.to_string();
                break;
            }
        }

        next_key
    }

    fn combinations(&self) -> i64 {
        let mut xmas: HashMap<String, [i64; 2]> = HashMap::new();
        xmas.insert("x".to_string(), [1,4000]);
        xmas.insert("m".to_string(), [1,4000]);
        xmas.insert("a".to_string(), [1,4000]);
        xmas.insert("s".to_string(), [1,4000]);

        let mut result: Vec<HashMap<String, [i64; 2]>> = Vec::new();
        result.push(xmas.clone());

        self.count("in".to_string(), xmas.clone())
    }

    fn count(&self, key: String, xmas: HashMap<String, [i64; 2]>) -> i64 {
        if key == "R" { return 0; }
        if key == "A" {
            let mut product: i64 = 1;

            for [start, end] in xmas.values() {
                product *= end - start + 1;
            }

            return product;
        }

        let mut sum = 0;
        let leaves = self.get_info(key);
        let mut xmas = xmas.clone();
        for leaf in leaves {
            let mut opposite_context: [i64;2]  = [0,0];
            if leaf.sign == "<" {
                let [start, end] = xmas.get(&leaf.context).unwrap();
                let endd = end.clone();

                if end >= &leaf.value && start < &leaf.value {
                    *xmas.get_mut(&leaf.context).unwrap() = [*start, leaf.value - 1];
                    opposite_context[0] = leaf.value;
                    opposite_context[1] = endd;
                }
            } else if leaf.sign == ">" {
                let [start, end] = xmas.get(&leaf.context).unwrap();
                let startt = start.clone();

                if end > &leaf.value && start <= &leaf.value {
                    let val = leaf.value.clone();
                    *xmas.get_mut(&leaf.context).unwrap() = [val + 1, *end];
                    opposite_context[0] = startt;
                    opposite_context[1] = val;
                }
            }

            // println!("{:?}", leaf);
            // println!("{:?}", xmas);
            sum += self.count(leaf.key, xmas.clone());
            if leaf.context  != "" {
                *xmas.get_mut(&leaf.context).unwrap() = opposite_context;
            }
        }

        sum
    }

    fn get_info(&self, key: String) -> Vec<Leaf> {
        let input = self.tree.get(&key).unwrap();

        let exprs = input
            .split(",");

        let mut exprss: Vec<Leaf> = Vec::new();

        for expr in exprs {
            if expr.contains(":") {
                let mut chars = expr.chars();
                let context = chars.next().unwrap().to_string();
                let sign = chars.next().unwrap().to_string();

                let mut num = vec![];
                while let Some(char) = chars.next() {
                    if char == ':' { break }
                    num.push(char);
                }

                let num: String = num.into_iter().collect();
                let num: i64 = num.parse::<i64>().unwrap();
                let key: String = chars.into_iter().collect();

                exprss.push(Leaf {
                    context,
                    sign,
                    value: num,
                    key,
                })
            } else {
                exprss.push(Leaf {
                    context: "".to_string(),
                    sign: "".to_string(),
                    value: 0,
                    key: expr.to_string(),
                })
            }
        }

        exprss
    }
}

fn main() {
    let aplenty = Aplenty::build("input.txt").unwrap();
    let sum = aplenty.sum();
    println!("{:?}", sum);
    let combinations = aplenty.combinations();
    println!("{:?}", combinations);
}

#[cfg(test)]
mod tests {
    use crate::Aplenty;

    #[test]
    fn part1_sum() {
        let aplenty = Aplenty::build("test.txt").unwrap();
        let sum = aplenty.sum();
        assert_eq!(sum, 19114);
    }

    #[test]
    fn part2_sum() {
        let aplenty = Aplenty::build("test.txt").unwrap();
        let sum = aplenty.sum_2();
        assert_eq!(sum, 167409079868000);
    }
}