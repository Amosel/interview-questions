// Description:

// Write a Rust program to merge two sorted linked lists into a single sorted linked list. The final list should be made by splicing together the nodes of the first two lists.

// Example Inputs and Outputs:

// Input: List1 = 1 -> 2 -> 4, List2 = 1 -> 3 -> 4
// Output: Merged List = 1 -> 1 -> 2 -> 3 -> 4 -> 4
// Input: List1 = 5 -> 10 -> 15, List2 = 2 -> 3 -> 20
// Output: Merged List = 2 -> 3 -> 5 -> 10 -> 15 -> 20
// Input: List1 = empty, List2 = 1 -> 3 -> 4
// Output: Merged List = 1 -> 3 -> 4
// Input: List1 = 1 -> 2 -> 4, List2 = empty
// Output: Merged List = 1 -> 2 -> 4
// Constraints:

// The number of nodes in both lists will be in the range [0, 50].
// -100 <= Node.val <= 100
// Both list1 and list2 are sorted in non-decreasing order.
// Expected Time Complexity:

// O(n + m), where n and m are the lengths of the two lists.

// Expected Space Complexity:

// O(1). The iterative approach is expected.

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

fn main() {
    // Test your function with different lists here
}

fn merge_two_lists(
    list1: Option<Box<ListNode>>, 
    list2: Option<Box<ListNode>>
) -> Option<Box<ListNode>> {
    // Implement your function here
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a list from a vector
    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &val in vec.iter().rev() {
            let mut new_node = ListNode::new(val);
            new_node.next = current;
            current = Some(Box::new(new_node));
        }
        current
    }

    #[test]
    fn test_merge_two_lists() {
        assert_eq!(
            merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4])),
            to_list(vec![1, 1, 2, 3, 4, 4])
        );
        assert_eq!(
            merge_two_lists(to_list(vec![5, 10, 15]), to_list(vec![2, 3, 20])),
            to_list(vec![2, 3, 5, 10, 15, 20])
        );
        assert_eq!(
            merge_two_lists(to_list(vec![]), to_list(vec![1, 3, 4])),
            to_list(vec![1, 3, 4])
        );
        assert_eq!(
            merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![])),
            to_list(vec![1, 2, 4])
        );
    }
}
