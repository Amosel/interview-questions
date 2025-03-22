// https://leetcode.com/problems/add-two-numbers/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
fn add_large_numbers(l: Vec<i32>, r: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut carry = 0;
    let len = l.len().max(r.len());

    for i in 0..len {
        let left = *l.get(i).unwrap_or(&0);
        let right = *r.get(i).unwrap_or(&0);
        let sum = left + right + carry;
        result.push(sum % 10);
        carry = sum / 10;
    }

    if carry != 0 {
        result.push(carry);
    }
    result
}

#[allow(dead_code)]
fn from_rev_list_node(mut input: &Option<Box<ListNode>>) -> Vec<i32> {
    if input.is_none() {
        return vec![];
    }
    let mut result = Vec::new();
    loop {
        result.push(input.as_ref().unwrap().val);
        input = &input.as_ref().unwrap().next;
        if input.is_none() {
            break;
        }
    }
    result
}

#[allow(dead_code)]
fn add_large_numbers_list_node(
    left_node: Option<Box<ListNode>>,
    right_node: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;
    let mut carry = 0;
    let mut left_node = left_node;
    let mut right_node = right_node;

    while left_node.is_some() || right_node.is_some() || carry > 0 {
        let mut sum = carry;

        if let Some(node) = left_node {
            sum += node.val;
            left_node = node.next;
        }

        if let Some(node) = right_node {
            sum += node.val;
            right_node = node.next;
        }

        carry = sum / 10;
        let digit = sum % 10;

        // Create new node and append to result list
        *tail = Some(Box::new(ListNode::new(digit)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_large_numbers() {
        // Test with empty vectors
        assert_eq!(add_large_numbers(vec![], vec![]), vec![]);

        // Test with one empty vector
        assert_eq!(add_large_numbers(vec![], vec![5, 7, 9]), vec![5, 7, 9]);

        // Test with large carry propagation
        assert_eq!(
            add_large_numbers(vec![9, 9, 9, 9, 9], vec![9, 9, 9, 9, 9]),
            vec![8, 9, 9, 9, 9, 1]
        );

        // Test with uneven lengths and multiple carries
        assert_eq!(
            add_large_numbers(vec![5, 6, 7], vec![5, 6, 7, 8, 9]),
            vec![0, 3, 5, 9, 9]
        );

        // Test with a single digit that causes a carry
        assert_eq!(add_large_numbers(vec![9], vec![1]), vec![0, 1]);
        assert_eq!(
            add_large_numbers(
                vec![
                    2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4,
                    3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2,
                    4, 3, 2, 4, 3, 2, 4, 3, 9,
                ],
                vec![
                    5, 6, 4, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4,
                    3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2,
                    4, 3, 2, 4, 3, 9, 9, 9, 9,
                ]
            ),
            vec![
                7, 0, 8, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4,
                8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8,
                6, 1, 4, 3, 9, 1
            ]
        );
    }

    macro_rules! rev_list {
      ($($x:expr),*) => {
        {

            let mut head = None;
          $(
            head = Some(Box::new(ListNode { val: $x, next: head }));
          )*
          head
        }
      };
    }
    #[test]
    fn test_rev_list_macro() {
        let node = rev_list![8, 0, 9];
        if let Some(node) = &node {
            assert_eq!(node.val, 9);
            if let Some(node) = &node.next {
                assert_eq!(node.val, 0);
                if let Some(node) = &node.next {
                    assert_eq!(node.val, 8);
                } else {
                    panic!("Third mode missing")
                }
            } else {
                panic!("Second mode missing");
            }
        } else {
            panic!("List is empty");
        }
    }

    macro_rules! rev_list_to_vec {
        ($list:expr) => {{
            let mut result = Vec::new();
            let mut current = &$list;
            while let Some(node) = current {
                result.insert(0, node.val);
                current = &node.next;
            }
            result
        }};
    }

    #[test]
    fn test_rev_list_to_vec() {
        let vec = rev_list_to_vec!(Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 8, next: None }))
            }))
        })));
        assert_eq!(vec, vec![8, 0, 9]);
    }

    macro_rules! list {
        ($($x:expr),*) => {{
            let mut vec = vec![$($x),*];
            let mut head = None;
            while let Some(val) = vec.pop() {
                head = Some(Box::new(ListNode { val, next: head }));
            }
            head
        }};
    }

    #[test]
    fn test_add_large_numbers_list_node() {
        // Test with empty vectors
        assert_eq!(
            add_large_numbers_list_node(rev_list![], rev_list![]),
            list![]
        );

        // Test with one empty vector
        assert_eq!(
            add_large_numbers_list_node(rev_list![], rev_list![5, 7, 9]),
            rev_list![5, 7, 9]
        );
        assert_eq!(
            add_large_numbers_list_node(rev_list![2, 4, 3], rev_list![5, 6, 4]),
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode { val: 8, next: None }))
                }))
            }))
        );
        assert_eq!(
            add_large_numbers_list_node(rev_list![2, 4, 3], rev_list![5, 6, 4]),
            rev_list![8, 0, 7] // Now this is 807 in reverse order: 7 → 0 → 8
        );
    }
}

fn main() {}
