use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub enum Operation {
    Identity(i64),
    Addition(i64, i64),
    Multiplication(i64, i64),
}

impl Operation {
    fn symbol(&self) -> char {
        match self {
            Operation::Identity(_) => ' ',
            Operation::Addition(_, _) => '+',
            Operation::Multiplication(_, _) => '*',
        }
    }
}

pub struct FastSolver {
    base: Vec<i64>,
    inputs: Vec<i64>,
    cache: HashMap<i64, Operation>,
}

impl FastSolver {
    pub fn new(base: Vec<i64>) -> FastSolver {
        let inputs = base.clone();
        let cache = inputs
            .iter()
            .map(|&x| (x, Operation::Identity(x)))
            .collect();

        FastSolver {
            base,
            inputs,
            cache,
        }
    }

    fn iterate_until(&mut self, target: i64) {
        'main: loop {
            for i in 0..self.inputs.len() {
                for j in 0..self.base.len() {
                    let a = self.inputs[i];
                    let b = self.base[j];

                    for (result, operation) in [
                        (a.checked_add(b), Operation::Addition(a, b)),
                        (a.checked_mul(b), Operation::Multiplication(a, b)),
                    ] {
                        if let Some(result) = result {
                            if !self.cache.contains_key(&result) {
                                self.cache.insert(result, operation);
                                self.inputs.push(result);
                            }

                            if target == result {
                                break 'main;
                            }
                        }
                    }
                }
            }
        }
    }

    fn format_solution(&self, target: i64) -> String {
        match self.cache.get(&target).unwrap() {
            Operation::Identity(x) => format!("{}", x),
            operation @ Operation::Addition(a, b) => {
                format!("{} {} {}", self.format_solution(*a), operation.symbol(), b)
            }
            operation @ Operation::Multiplication(a, b) => {
                format!("{} {} {}", self.format_solution(*a), operation.symbol(), b)
            }
        }
    }

    pub fn solve(&mut self, target: i64) -> String {
        self.iterate_until(target);
        self.format_solution(target)
    }
}
