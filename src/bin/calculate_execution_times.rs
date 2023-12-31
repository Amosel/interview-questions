fn calculate_execution_times(
    logs: &Vec<(&'static str, i32, &str)>,
) -> Result<Vec<(&'static str, i32)>, String> {
    let mut stack: Vec<(&'static str, i32, Option<i32>)> = Vec::new();
    for (name, timestamp, start_or_end) in logs {
        match *start_or_end {
            "START" => stack.push((*name, *timestamp, None)),
            "END" => {
                if let Some(index) = stack.iter().position(|s| s.0 == *name) {
                    let mut duration = timestamp - stack[index].1;
                    for i in (index + 1)..stack.len() {
                        if let Some(item_duration) = stack[i].2 {
                            duration -= item_duration;
                        } else {
                            let error_name: &str = stack[i].0;
                            return Err(format!("Expected End for {error_name}"));
                        }
                    }
                    stack[index].2 = Some(duration);
                } else {
                    return Err(format!("Expected END for {name}"));
                }
            }
            _ => return Err(format!("Expected START or END got {start_or_end}")),
        }
    }
    let mut result = Vec::new();

    for item in stack {
        match item {
            (name, _, Some(duration)) => result.push((name, duration)),
            _ => {
                let error_name: &str = item.0;
                return Err(format!("Expected End for {error_name}"));
            }
        }
    }
    Ok(result)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic test for a single function call
    #[test]
    fn test_single_function() {
        let logs = vec![("func1", 100, "START"), ("func1", 200, "END")];
        let execution_times = calculate_execution_times(&logs);
        assert_eq!(execution_times.unwrap().get(0), Some(&("func1", 100)));
    }

    // Test for nested function calls
    #[test]
    #[rustfmt::skip]
    fn test_nested_functions() {
        let logs = vec![
            ("func1", 100, "START"),
              ("func2", 150, "START"),
                ("func3", 160, "START"),
                  ("func4", 170, "START"),
                  ("func4", 180, "END"),    // func4 ends, lasting 10
                  ("func5", 190, "START"),
                  ("func5", 200, "END"),    // func5 ends lasting 10
                ("func3", 210, "END"),      // func3 ends lasting 210 - 160 - (10 + 10) = 30
              ("func2", 250, "END"),        // func2 ends lasting 250 - 150 - (30 + 10 + 10) = 50
              ("func6", 260, "START"),      // func6 starts, not nested
              ("func6", 300, "END"),        // func6 ends 300 - 260 = 40
            ("func1", 350, "END"),          // func1 ends 350 - 100 - (40 + 50 + 30 + 10 + 10) = 110

        ];
        let execution_times = calculate_execution_times(&logs);
        assert!(execution_times.is_ok());
        let execution_times = execution_times.unwrap();
        assert_eq!(execution_times.get(0), Some(&("func1", 110))); // Total time for func1
        assert_eq!(execution_times.get(1), Some(&("func2", 50))); // Total time for func2
        assert_eq!(execution_times.get(2), Some(&("func3", 30))); // Total time for func3
        assert_eq!(execution_times.get(3), Some(&("func4", 10))); // Total time for func4
        assert_eq!(execution_times.get(4), Some(&("func5", 10))); // Total time for func5
        assert_eq!(execution_times.get(5), Some(&("func6", 40))); // Total time for func6
    }
    #[test]
    fn test_err_no_end_call() {
        let logs = vec![("func1", 100, "START"), ("func2", 150, "START")];
        let execution_times = calculate_execution_times(&logs);
        assert!(execution_times.is_err());
    }
    #[test]
    fn test_err_no_start_call() {
        let logs = vec![("func1", 200, "END"), ("func2", 250, "END")];
        let execution_times = calculate_execution_times(&logs);
        assert!(execution_times.is_err());
    }
}
