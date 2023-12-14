fn apply_op(op: &str, numbers: &mut Vec<f64>) {
    let b = numbers.pop().unwrap();
    let a = numbers.pop().unwrap();
    match op {
        "+" => numbers.push(a + b),
        "-" => numbers.push(a - b),
        "/" => {
            if b == 0.0 {
                todo!()
            }
            return numbers.push(a / b);
        }
        "*" => return numbers.push(a * b),
        // todo deal with errors here...
        _ => unimplemented!(),
    }
    println!("  {} {} {} = {}", a, op, b, &numbers.last().unwrap());
}

fn evaluate_expression(arr: Vec<&str>) -> Result<f64, String> {
    let mut number_stack: Vec<f64> = Vec::new();
    let mut op_stack: Vec<&str> = Vec::new();
    for el in arr {
        match el {
            number if number.parse::<f64>().is_ok() => {
                let number: f64 = number.parse().unwrap();
                number_stack.push(number);
            }
            "+" | "-" => {
                while let Some(&op_top) = op_stack.last() {
                    if op_top == "(" {
                        break;
                    } else {
                        apply_op(op_stack.pop().unwrap(), &mut number_stack);
                    }
                }
                op_stack.push(el);
            }
            "*" | "/" => op_stack.push(el),
            "(" => op_stack.push(el),
            ")" => {
                while let Some(op) = op_stack.pop() {
                    if op == "(" {
                        break;
                    } else {
                        apply_op(op, &mut number_stack);
                    }
                }
            }
            _ => {
                return Err(format!("unexpected input {}", el));
            }
        }
        println!("{}, ops: {:?} nums: {:?}", el, op_stack, number_stack);
    }

    while let Some(op) = op_stack.pop() {
        apply_op(op, &mut number_stack);
    }
    if number_stack.len() == 1 {
        Ok(*number_stack.last().unwrap())
    } else {
        Err(format!("something went wrong"))
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(evaluate_expression(vec!["1", "+", "2"]), Ok(3f64));
        assert_eq!(evaluate_expression(vec!["2", "-", "2"]), Ok(0.0));
        assert_eq!(evaluate_expression(vec!["2", "-", "4"]), Ok(-2f64));
        assert_eq!(evaluate_expression(vec!["3", "*", "2"]), Ok(6f64));
        assert_eq!(evaluate_expression(vec!["1", "/", "2"]), Ok(0.5f64));
    }
    #[test]
    fn test_operator_precedence() {
        // test operator precendence
        assert_eq!(evaluate_expression(vec!["1", "+", "2", "*", "4"]), Ok(9f64));
        assert_eq!(
            evaluate_expression(vec!["1", "*", "2", "*", "4", "+", "2"]),
            Ok(10f64)
        );
        assert_eq!(
            evaluate_expression(vec!["1", "+", "2", "/", "4"]),
            Ok(1.5f64)
        );
    }
    #[test]
    fn test_parenthesis_precedence() {
        // test operator precendence
        assert_eq!(
            evaluate_expression(vec!["3", "*", "(", "2", "+", "4", ")"]),
            Ok(18f64)
        );
        assert_eq!(
            evaluate_expression(vec![
                // 3 * 6 + 0.25
                "3", "*", "(", "2", "+", "4", ")", "+", "(", "2", "/", "8", ")"
            ]),
            Ok(18.25f64)
        );
        assert_eq!(
            evaluate_expression(vec![
                // 3 * 6 + 0.25
                "3", "*", "(", "2", "+", "4", ")", "*", "(", "2", "/", "8", ")"
            ]),
            Ok(4.5f64)
        );
    }
}
