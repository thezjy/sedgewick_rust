use std::collections::HashMap;

use crate::binary_search::binary_search;

pub fn two_sum(list: &[i32]) -> usize {
    let mut count = 0;

    for i in 0..list.len() {
        for j in (i + 1)..list.len() {
            if list[i] + list[j] == 0 {
                count += 1;
            }
        }
    }

    count
}

pub fn two_sum_fast(list: &mut [i32]) -> usize {
    list.sort();

    let mut count = 0;

    list.iter().enumerate().for_each(|(i, item)| {
        if let Ok(j) = list.binary_search(&-item) {
            if i < j {
                count += 1;
            }
        }
    });

    count
}

pub fn three_sum(list: &[i32]) -> usize {
    let mut count = 0;
    let len = list.len();

    for i in 0..len {
        for j in (i + 1)..len {
            for k in (j + 1)..len {
                if list[i] + list[j] + list[k] == 0 {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn three_sum_binary(list: &mut [i32]) -> usize {
    list.sort();

    let mut count = 0;
    let len = list.len();

    for i in 0..len {
        for j in (i + 1)..len {
            if let Ok(k) = list.binary_search(&(-list[i] - list[j])) {
                if j < k {
                    count += 1;
                }
            }
        }
    }

    count
}

enum BinaryOperators {
    Add,
    Subtract,
    Multiply,
    Divide,
}

enum UnaryOperators {
    Sqrt,
}

enum Operators {
    Unary(UnaryOperators),
    Binary(BinaryOperators),
}

impl UnaryOperators {
    fn evaluate(&self, operand: f64) -> f64 {
        match self {
            UnaryOperators::Sqrt => operand.sqrt(),
        }
    }
}

impl BinaryOperators {
    fn evaluate(&self, l: f64, r: f64) -> f64 {
        match self {
            BinaryOperators::Add => l + r,
            BinaryOperators::Subtract => l - r,
            BinaryOperators::Multiply => l * r,
            BinaryOperators::Divide => l / r,
        }
    }
}

pub fn evaluate_arithmetic_expression(expression: &str) -> f64 {
    let mut operators = Vec::new();
    let mut operands: Vec<f64> = Vec::new();

    expression.split(' ').for_each(|s| match s {
        "+" => operators.push(Operators::Binary(BinaryOperators::Add)),
        "-" => operators.push(Operators::Binary(BinaryOperators::Subtract)),
        "*" => operators.push(Operators::Binary(BinaryOperators::Multiply)),
        "/" => operators.push(Operators::Binary(BinaryOperators::Divide)),
        "sqrt" => operators.push(Operators::Unary(UnaryOperators::Sqrt)),
        "(" => {}
        ")" => {
            if let Some(right) = operands.pop() {
                if let Some(op) = operators.pop() {
                    match op {
                        Operators::Unary(op) => {
                            operands.push(op.evaluate(right));
                        }
                        Operators::Binary(op) => {
                            if let Some(left) = operands.pop() {
                                operands.push(op.evaluate(left, right));
                            }
                        }
                    }
                }
            }
        }
        other => {
            if let Ok(num) = other.parse() {
                operands.push(num);
            }
        }
    });

    operands[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate() {
        assert_eq!(
            evaluate_arithmetic_expression("( 1 + ( ( 2 + 3 ) * ( 4 * 5 ) ) )"),
            101.0
        );
        assert_eq!(
            evaluate_arithmetic_expression("( ( 1 + sqrt ( 5.0 ) ) / 2.0 )"),
            1.618033988749895
        );
    }
}
