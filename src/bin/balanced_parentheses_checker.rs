fn main() {
    let test_string = "[{()}]";
    println!(
        "Is '{}' balanced? {}",
        test_string,
        is_balanced(test_string)
    );
}

fn is_balanced(s: &str) -> bool {
    let mut stack = Vec::new();
    for char in s.chars() {
      match char {
          ')' => if stack.pop() != Some('(') { return false },
          ']' => if stack.pop() != Some('[') { return false },
          '}' => if stack.pop() != Some('{') { return false },
          '(' | '[' | '{' => stack.push(char),
          _ => {}
      }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced() {
        assert!(is_balanced("()"));
        assert!(is_balanced("[]"));
        assert!(is_balanced("{}"));
        assert!(is_balanced("({[]})"));
        assert!(is_balanced("(())"));
        assert!(is_balanced("{([])}"));
    }

    #[test]
    fn test_not_balanced() {
        assert!(!is_balanced("(]"));
        assert!(!is_balanced("([)]"));
        assert!(!is_balanced("{[}]"));
        assert!(!is_balanced("{"));
        assert!(!is_balanced("}"));
    }
}
