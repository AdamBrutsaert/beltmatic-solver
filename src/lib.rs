use std::collections::HashMap;

pub type Value = u32;

enum Operation {
    Identity(Value),
    Addition(Value, Value),
    Multiplication(Value, Value),
    Substraction(Value, Value),
    Exponentiation(Value, Value),
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
