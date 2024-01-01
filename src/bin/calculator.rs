// https://leetcode.com/problems/basic-calculator/

// use core::num;
// use std::collections::HashMap;
// Evaluates a mathematical expression represented as a vector of strings.

use std::collections::HashMap;

fn main() {}

fn apply_op(op: &str, numbers: &mut Vec<f64>) -> Result<(), String> {
    if numbers.len() < 2 {
        return Err(format!("Not enought numbers to run expr"));
    }
    let b = numbers.pop().unwrap();
    let a = numbers.pop().unwrap();
    match op {
        "+" => numbers.push(a + b),
        "-" => numbers.push(a - b),
        "*" => numbers.push(a * b),
        "/" => {
            if a == 0.0 {
                return Err(format!("Divide by zero"));
            }
            numbers.push(b / a)
        }
        _ => return Err(format!("Expected an op {op}")),
    }
    println!(
        "Applying op {op} with {a} and {b} got {}",
        numbers.last().unwrap()
    );
    Ok(())
}

// vec![
//                 "(", "1", "+", "(", "4", "+", "5", "+", "2", ")", "-", "3", ")", "+", "(", "6",
//                 "+", "8", ")"
//             ]
#[allow(dead_code)]
fn evaluate_expression(input: Vec<&'static str>) -> Result<f64, String> {
    let mut op_stack: Vec<&'static str> = Vec::new();
    let mut num_stack = Vec::new();
    let mut precedence = HashMap::new();
    precedence.insert("+", 1);
    precedence.insert("-", 1);
    precedence.insert("*", 2);
    precedence.insert("/", 2);

    for curr_item in input {
        match curr_item {
            "+" | "-" | "*" | "/" => {
                while op_stack
                    .last()
                    .and_then(|&op| {
                        if op != "(" && precedence[op] >= precedence[curr_item] {
                            Some(op)
                        } else {
                            None
                        }
                    })
                    .is_some()
                {
                    apply_op(op_stack.pop().unwrap(), &mut num_stack)?;
                }
                op_stack.push(curr_item);
                println!("Adding {curr_item}, {:?}", op_stack);
            }
            "(" => op_stack.push(curr_item),
            ")" => {
                while let Some(op) = op_stack.pop() {
                    if op == "(" {
                        break;
                    }
                    apply_op(op, &mut num_stack)?;
                }
            }
            _ => num_stack.push(
                curr_item
                    .parse::<f64>()
                    .map_err(|_| format!("Expected a float literal {curr_item}"))?,
            ),
        }
    }
    println!("last one {:?}", op_stack);
    while let Some(op) = op_stack.pop() {
        if op == "(" {
            break;
        }
        apply_op(op, &mut num_stack)?;
    }
    Ok(num_stack.pop().unwrap())
}

#[allow(dead_code)]
fn to_array(input: &'static str) -> Result<Vec<&'static str>, String> {
    let mut start: Option<usize> = None;
    let mut result: Vec<&'static str> = Vec::new();
    for i in 0..input.len() {
        let c: &'static str = input.get(i..i + 1).unwrap();

        match c {
            "+" | "-" | "*" | "/" | "(" | ")" => {
                if let Some(start_index) = start.take() {
                    let number: &'static str = input.get(start_index..i).unwrap().trim();
                    if number.parse::<f64>().is_err() {
                        return Err(format!("Expected {number} to be a float literal"));
                    }
                    result.push(number);
                }
                result.push(c);
            }
            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0" | "." => {
                if start.is_none() {
                    start = Some(i);
                }
            }
            _ => {}
        }
    }
    if let Some(start_index) = start.take() {
        let len = input.len();
        let number: &'static str = input.get(start_index..len).unwrap().trim();
        if number.parse::<f64>().is_err() {
            return Err(format!("Expected {number} to be a float literal"));
        }
        result.push(number);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_array() {
        assert_eq!(to_array("100 + 100"), Ok(vec!["100", "+", "100"]));
        assert_eq!(to_array(" 2 - 1 + 2 "), Ok(vec!["2", "-", "1", "+", "2"]));
        assert_eq!(
            to_array("3+5 * (2 - 1)"),
            Ok(vec!["3", "+", "5", "*", "(", "2", "-", "1", ")"])
        );
        assert_eq!(
            to_array("(1+(4+5+2)-3)+(6+8)"),
            Ok(vec![
                "(", "1", "+", "(", "4", "+", "5", "+", "2", ")", "-", "3", ")", "+", "(", "6",
                "+", "8", ")"
            ])
        );
    }
    #[test]
    fn test_all() {
        assert_eq!(evaluate_expression(to_array("1 + 1").unwrap()), Ok(2.0));
        assert_eq!(
            evaluate_expression(to_array(" 2 - 1 * 4 ").unwrap()),
            Ok(-2.0)
        );
        assert_eq!(
            evaluate_expression(to_array("3 + 5 * (2 - 1)").unwrap()),
            Ok(8.0)
        );
        assert_eq!(
            evaluate_expression(to_array("(1+(4+5+2)-3)+(6+8)").unwrap()),
            Ok(23.0)
        );
    }
}
