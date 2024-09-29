use itertools::Itertools;
use std::collections::HashMap;

pub type Value = u32;

#[derive(Copy, Clone, Debug)]
enum Operation {
    Identity(Value),
    Addition(Value, Value),
    Multiplication(Value, Value),
    Substraction(Value, Value),
    Exponentiation(Value, Value),
}

impl Operation {
    fn symbol(&self) -> char {
        match self {
            Operation::Identity(_) => ' ',
            Operation::Addition(_, _) => '+',
            Operation::Multiplication(_, _) => '*',
            Operation::Substraction(_, _) => '-',
            Operation::Exponentiation(_, _) => '^',
        }
    }
}

pub struct FastSolver {
    base: Vec<Value>,
    inputs: Vec<Value>,
    cache: HashMap<Value, Operation>,
}

impl FastSolver {
    pub fn new(base: Vec<Value>) -> FastSolver {
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

    fn iterate_until(&mut self, target: Value) {
        'main: loop {
            for i in 0..self.inputs.len() {
                for j in 0..self.base.len() {
                    let a = self.inputs[i];
                    let b = self.base[j];

                    for (result, operation) in [
                        (a.checked_add(b), Operation::Addition(a, b)),
                        (a.checked_mul(b), Operation::Multiplication(a, b)),
                        (a.checked_sub(b), Operation::Substraction(a, b)),
                        (a.checked_pow(b), Operation::Exponentiation(a, b)),
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

    fn format_solution(&self, target: Value) -> String {
        match self.cache.get(&target).unwrap() {
            Operation::Identity(x) => format!("{}", x),
            Operation::Addition(a, b) => format!("({} + {})", self.format_solution(*a), b),
            Operation::Multiplication(a, b) => format!("({} * {})", self.format_solution(*a), b),
            Operation::Substraction(a, b) => format!("({} - {})", self.format_solution(*a), b),
            Operation::Exponentiation(a, b) => format!("({} ^ {})", self.format_solution(*a), b),
        }
    }

    pub fn solve(&mut self, target: Value) -> String {
        self.iterate_until(target);
        self.format_solution(target)
    }
}

#[derive(Debug)]
struct CacheData {
    operations: Vec<Operation>,
    complexity: u32,
}

pub struct FullSolver {
    base: Vec<Value>,
    inputs: Vec<Value>,
    cache: HashMap<Value, CacheData>,
    iterations: u32,
}

impl FullSolver {
    pub fn new(base: Vec<Value>) -> FullSolver {
        let inputs = base.clone();
        let cache = inputs
            .iter()
            .map(|&x| {
                (
                    x,
                    CacheData {
                        operations: vec![Operation::Identity(x)],
                        complexity: 0,
                    },
                )
            })
            .collect();

        FullSolver {
            base,
            inputs,
            cache,
            iterations: 1,
        }
    }

    fn iterate_until(&mut self, target: Value) {
        while !self.cache.contains_key(&target) {
            for i in 0..self.inputs.len() {
                for j in 0..self.base.len() {
                    let a = self.inputs[i];
                    let b = self.base[j];

                    for (result, operation) in [
                        (a.checked_add(b), Operation::Addition(a, b)),
                        (a.checked_mul(b), Operation::Multiplication(a, b)),
                        (a.checked_sub(b), Operation::Substraction(a, b)),
                        (a.checked_pow(b), Operation::Exponentiation(a, b)),
                    ] {
                        if let Some(result) = result {
                            self.cache
                                .entry(result)
                                .and_modify(|data| match data.complexity.cmp(&self.iterations) {
                                    std::cmp::Ordering::Equal => {
                                        data.operations.push(operation);
                                    }
                                    std::cmp::Ordering::Greater => {
                                        data.operations = vec![operation];
                                        data.complexity = self.iterations;
                                    }
                                    _ => {}
                                })
                                .or_insert_with(|| {
                                    self.inputs.push(result);
                                    CacheData {
                                        operations: vec![operation],
                                        complexity: self.iterations,
                                    }
                                });
                        }
                    }
                }
            }

            self.iterations += 1;
        }
    }

    fn format_solution(&self, value: Value) -> Vec<String> {
        self.cache
            .get(&value)
            .unwrap_or(&CacheData {
                operations: vec![],
                complexity: 0,
            })
            .operations
            .iter()
            .flat_map(|operation| match operation {
                Operation::Identity(value) => vec![value.to_string()],
                Operation::Addition(left, right)
                | Operation::Substraction(left, right)
                | Operation::Multiplication(left, right)
                | Operation::Exponentiation(left, right) => self
                    .format_solution(*left)
                    .iter()
                    .cartesian_product(self.format_solution(*right).iter())
                    .map(|(left, right)| format!("({} {} {})", left, operation.symbol(), right))
                    .collect(),
            })
            .collect()
    }

    pub fn solve(&mut self, target: Value) -> Vec<String> {
        self.iterate_until(target);
        self.format_solution(target)
    }
}
