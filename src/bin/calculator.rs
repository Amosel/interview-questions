use std::collections::HashMap;

// Evaluates a mathematical expression represented as a vector of strings.
pub fn evaluate_expression(tokens: Vec<&str>) -> Result<f64, String> {
    let mut operator_stack: Vec<&str> = Vec::new();
    let mut output_stack: Vec<f64> = Vec::new();
    let precedence = operator_precedence();

    for token in tokens {
        match token {
            // If token is a number, parse it and push onto the output stack.
            number if number.parse::<f64>().is_ok() => {
                output_stack.push(number.parse().unwrap());
            }
            // If token is an operator, handle according to precedence.
            "+" | "-" | "*" | "/" => {
                while let Some(&op_top) = operator_stack.last() {
                    if op_top != "(" && precedence[op_top] >= precedence[token] {
                        let result =
                            apply_operator(operator_stack.pop().unwrap(), &mut output_stack)?;
                        output_stack.push(result);
                    } else {
                        break;
                    }
                }
                operator_stack.push(token);
            }
            // If token is a left parenthesis, push it onto the stack.
            "(" => operator_stack.push(token),
            // If token is a right parenthesis, pop operators until a left parenthesis is encountered.
            ")" => {
                while let Some(op) = operator_stack.pop() {
                    // reached end:
                    if op == "(" {
                        break;
                    }
                    let result = apply_operator(op, &mut output_stack)?;
                    output_stack.push(result);
                }
            }
            _ => return Err("Invalid token".to_string()),
        }
    }

    // After processing all tokens, apply remaining operators.
    while let Some(op) = operator_stack.pop() {
        if op != "(" {
            let result = apply_operator(op, &mut output_stack)?;
            output_stack.push(result);
        }
    }

    // The final result should be the only element in the output stack.
    if output_stack.len() == 1 {
        Ok(output_stack[0])
    } else {
        Err("Invalid expression".to_string())
    }
}

// Returns a HashMap representing the precedence of each operator.
fn operator_precedence() -> HashMap<&'static str, i32> {
    let mut precedence = HashMap::new();
    precedence.insert("+", 1);
    precedence.insert("-", 1);
    precedence.insert("*", 2);
    precedence.insert("/", 2);
    precedence
}

// Applies an operator to the last two values in the output stack.
fn apply_operator(operator: &str, output_stack: &mut Vec<f64>) -> Result<f64, String> {
    if output_stack.len() < 2 {
        return Err("Insufficient values in the expression".to_string());
    }

    let b = output_stack.pop().unwrap();
    let a = output_stack.pop().unwrap();

    let result = match operator {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == 0.0 {
                return Err("Division by zero".to_string());
            }
            a / b
        }
        _ => return Err("Invalid operator".to_string()),
    };

    Ok(result)
}

fn main() {
    let expression = vec!["3", "+", "5", "*", "(", "2", "-", "1", ")"];
    match evaluate_expression(expression) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        assert_eq!(
            evaluate_expression(vec!["3", "+", "5", "*", "(", "2", "-", "1", ")"]),
            Result::Ok(8f64)
        );
    }
}
