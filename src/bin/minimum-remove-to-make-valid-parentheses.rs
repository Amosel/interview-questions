fn min_remove_to_make_valid(s: String) -> String {
    let mut buffer = vec![' '; s.len()];
    let mut balance = 0;
    let mut index = 0;

    // Forward pass: track balance and build the string
    for c in s.chars() {
        if c == '(' {
            balance += 1;
            buffer[index] = c;
            index += 1;
        } else if c == ')' {
            if balance > 0 {
                balance -= 1;
                buffer[index] = c;
                index += 1;
            }
        } else {
            // Copy other characters directly
            buffer[index] = c;
            index += 1;
        }
    }

    // Reverse pass: remove excess '(' from the end
    let mut excess_open = balance;
    let mut valid_string = String::with_capacity(index);
    for &c in buffer.iter().take(index).rev() {
        if c == '(' && excess_open > 0 {
            excess_open -= 1;
        } else {
            valid_string.push(c);
        }
    }

    valid_string.chars().rev().collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_two_lists() {
        assert_eq!(
            min_remove_to_make_valid("lee(t(c)o)de)".to_string()),
            "lee(t(c)o)de".to_string()
        );
        assert_eq!(
            min_remove_to_make_valid("a)b(c)d".to_string()),
            "ab(c)d".to_string()
        );
        assert_eq!(min_remove_to_make_valid("))((".to_string()), "".to_string());
    }
}
