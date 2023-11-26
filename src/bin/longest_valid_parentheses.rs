// Write a Rust program to find the length of the longest valid (well-formed) parentheses substring. A valid parentheses substring is a string containing only the characters '(' and ')' that is balanced.
// Example Inputs and Outputs:

// Input: "(()"
// Output: 2 (The longest valid parentheses substring is "()")
// Input: ")()())"
// Output: 4 (The longest valid parentheses substring is "()()")
// Input: "" (empty string)
// Output: 0
// Input: "(()(((()"
// Output: 2
// Constraints:

// The string length will not exceed 10,000 characters.
// The string will contain only the characters '(' and ')'.
// Expected Time Complexity:

// Your solution should aim for O(n) time complexity, where n is the length of the input string.

// Expected Space Complexity:

// Your solution should aim for O(n) space complexity in the worst case.

fn main() {
    let test_string = "(()";
    println!(
        "The length of the longest valid parentheses in '{}' is {}",
        test_string,
        longest_valid_parentheses(test_string)
    );
}

// we need to first establish an index that indicate the start of a valid index.
// the index before the start of a valid substring

fn longest_valid_parentheses(str: &str) -> i32 {
    // the count of open parenthesis:
    let mut length = 0i32;
    let mut stack = Vec::new(); // stack of indices of open parenteses.
    stack.push(-1);
    for (index, char) in str.chars().enumerate() {
        if char == '(' {
            stack.push(index as i32);
        } else {
            stack.pop();
            if stack.is_empty() {
                stack.push(index as i32)
            } else {
                length = length.max(index as i32 - *stack.last().unwrap());
            }
        }
    }
    length
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        assert_eq!(longest_valid_parentheses("(()"), 2);
        assert_eq!(longest_valid_parentheses(")()"), 2);
        assert_eq!(longest_valid_parentheses("((((()))))"), 10);
    }

    #[test]
    fn test_longer_invalid_substring_cases() {
        assert_eq!(longest_valid_parentheses("(()"), 2);
        assert_eq!(longest_valid_parentheses("(()(((()"), 2);
    }

    #[test]
    fn test_longer_valid_complete_substring_cases() {
        assert_eq!(longest_valid_parentheses(")()()"), 4);
        assert_eq!(longest_valid_parentheses("()()(())"), 8);
    }
    #[test]
    fn test_longer_valid_substring_slice_cases() {
        assert_eq!(longest_valid_parentheses(")(()()"), 4);
        assert_eq!(longest_valid_parentheses("(()(())"), 6);
    }
}
