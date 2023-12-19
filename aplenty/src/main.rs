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
}

fn main() {
    let aplenty = Aplenty::build("input.txt").unwrap();
    let sum = aplenty.sum();
    println!("{:?}", sum);
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
}