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

fn longest_valid_parentheses(str: &str) -> i32 {
    let mut o = 0i32; // open parents counter
    let mut s = -1i32; // substring index before start.
    let mut c = 0i32; // completed pairs in current substring.
    let mut length = 0i32;
    println!("string: {}", str);
    for (index, char) in str.chars().enumerate() {
        match char {
            '(' => {
                o += 1;
            }
            ')' => {
                if o == 0 {
                    s = index as i32;
                    o = 0;
                    c = 0;
                } else {
                    length = length.max(if o == 1 {
                        (index as i32) - (s)
                    } else {
                        (index as i32 + 1) - (s + o + (c * 2))
                    });
                    o -= 1;
                    c += 1;
                }
            }
            _ => {}
        }
    }
    println!("final {}\n", length);
    length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_cases() {
        assert_eq!(longest_valid_parentheses("(()"), 2);
        assert_eq!(longest_valid_parentheses(")()())"), 4);
        assert_eq!(longest_valid_parentheses("(()(((()"), 2);
        assert_eq!(longest_valid_parentheses(""), 0);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(longest_valid_parentheses("()(()"), 2);
        assert_eq!(longest_valid_parentheses("()()"), 4);
        assert_eq!(longest_valid_parentheses("(()))"), 4);
        assert_eq!(longest_valid_parentheses("((()))"), 6);
        assert_eq!(longest_valid_parentheses(")("), 0);
    }
}
