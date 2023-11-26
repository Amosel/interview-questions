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
    // A closing parenthesis without paired open paranthesis will invalidate a substring
    // Support advancing the beginning of valid substring
    let mut v: i32 = -1;
    // Keep a tab of open paranthesis positions.
    let mut stack = Vec::new(); // stack of indices of open parenteses.

    for (index, char) in str.chars().enumerate() {
        match char {
            '(' => {
                stack.push(index as i32);
            }
            ')' => {
                // look for a matching opening parenthesis
                if let Some(closing_parenthesis_index) = stack.pop() {
                    // identify the valid substring:
                    // 4 cases we cover here
                    // 1. The open parenthesis pair is at the beginning of the valid substring, it is the signle memeber of the stack.
                    // 2. The possibly valid substring start indicated with 'v' is before the index of the open parenthesis pair in which case there are two options:
                    // 2.1 The substring starting at v and ending in the index of the closing parenthsis is not valid.
                    // 2.2 The substring starting at v is valid.
                    // 2.3 The substring starting at some index that is bigger than v is valid.
                    // Here are examples of that:
                    // Example of case 1: ')()' or '((((()))))' -- here the index of the opening parenthesis pair will be enough to get the length
                    // Example of case 2.1: '(()' '(()(())(()' here the substring before the current closing parenthesis index is not valid.
                    // Example of case 2.2: ')()()' '()(())' here substring starting at v is valid
                    // Example of case 2.3: ')(()()' '(()(())' here only a slice of the substring starting at v is valid
                    if closing_parenthesis_index == v {
                        // deal with case 1:
                        length = length.max((index as i32) - closing_parenthesis_index);
                    } else if stack
                        .last()
                        .map_or(false, |&last| last == closing_parenthesis_index as i32 - 1)
                    {
                        length = length.max(2);
                    } else {
                        length = length.max((index as i32) - (v + (stack.len() as i32)));
                    }
                } else {
                    v = index as i32;
                }
            }
            _ => {}
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
