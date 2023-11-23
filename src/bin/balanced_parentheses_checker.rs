fn main() {
  let test_string = "[{()}]";
  println!("Is '{}' balanced? {}", test_string, is_balanced(test_string));
}

fn is_balanced(s: &str) -> bool {
  // TODO: Implement the logic to check if the string `s` is balanced.
  unimplemented!()
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
      assert!(is_balanced(""));
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
