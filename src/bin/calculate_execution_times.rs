fn calculate_execution_times<'a>(
    logs: &Vec<(&'a str, i32, &'a str)>,
) -> Result<Vec<(&'a str, i32)>, String> {
    let mut stack = Vec::new();
    // TODO: Add your implementation here
    for (name, timestamp, start_or_end) in logs {
        match *start_or_end {
            "START" => {
                // foo
                stack.push((name, timestamp, 0));
            }
            "END" => {
                // foo
                let index = stack.iter().position(|item| item.0 == name).unwrap();
                let sum = (1 + index..stack.len()).map(|i| stack[i].2).sum::<i32>();
                stack[index].2 = timestamp - stack[index].1 - sum;
                println!("{}: {}", name, stack[index].2);
            }
            _ => {}
        }
    }
    Ok(stack
        .iter()
        .map(|&(&name, _, duration)| (name, duration))
        .collect::<Vec<(&'a str, i32)>>())
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
    // TODO: return an error when overlapping
    // // Test for overlapping function calls
    // #[test]
    // fn test_overlapping_functions() {
    //     let logs = vec![
    //         ("func1", 100, "START"),
    //         ("func2", 150, "START"),
    //         ("func1", 200, "END"),
    //         ("func2", 250, "END"),
    //     ];
    //     let execution_times = calculate_execution_times(&logs);
    //     assert_eq!(execution_times.unwrap().get(0), Some(&("func1", 100)));
    //     assert_eq!(execution_times.unwrap().get(1), Some(&("func2", 100)));
    // }

    // Test for function calls with no end
    #[test]
    fn test_no_end_call() {
        let logs = vec![("func1", 100, "START"), ("func2", 150, "START")];
        let execution_times = calculate_execution_times(&logs);
        assert!(execution_times.is_ok());
        let execution_times = execution_times.unwrap();
        // Assuming functions without an end are not counted
        assert_eq!(execution_times.get(0), None);
        assert_eq!(execution_times.get(0), None);
    }

    // Test for function calls with no start
    #[test]
    fn test_no_start_call() {
        let logs = vec![("func1", 200, "END"), ("func2", 250, "END")];
        let execution_times = calculate_execution_times(&logs);
        assert!(execution_times.is_ok());
        let execution_times = execution_times.unwrap();
        // Assuming functions without a start are not counted
        assert_eq!(execution_times.get(0), None);
        assert_eq!(execution_times.get(0), None);
    }

    // TODO: resturn error on out of order
    // Test for out-of-order logs
    // #[test]
    // fn test_out_of_order_logs() {
    //     let logs = vec![("func1", 300, "END"), ("func1", 100, "START")];
    //     let execution_times = calculate_execution_times(&logs);
    //     // Assuming the function correctly handles out-of-order logs
    //     assert_eq!(execution_times.get("func1"), Some(&200));
    // }

    // Add more tests for other complex scenarios as needed
}
