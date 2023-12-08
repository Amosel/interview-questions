// taken from: https://leetcode.com/problems/merge-k-sorted-lists/
// Definition for singly-linked list.
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

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let root = ListNode::new(0);
    return root.next;
}

fn main() {}

#[allow(dead_code)]
fn to_list(input: Vec<i32>) -> Option<Box<ListNode>> {
    let root = ListNode::new(0);
    let mut tail: Option<Box<ListNode>> = None;
    for val in input {
        let node = ListNode::new(val);
        if tail.is_some() {
            let mut temp = tail.take().unwrap();
            temp.next = Some(Box::new(node));
            tail = temp.next;
        } else {
            tail = Some(Box::new(node));
        }
    }
    Some(Box::new(root))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_all() {
        // Input: lists = [[1,4,5],[1,3,4],[2,6]]
        // Output: [1,1,2,3,4,4,5,6]
        // Explanation: The linked-lists are:
        // [
        //   1->4->5,
        //   1->3->4,
        //   2->6
        // ]
        // merging them into one sorted list:
        // 1->1->2->3->4->4->5->6
        assert_eq!(
            merge_k_lists(
              vec![
                to_list(vec![1, 4, 5]),
                to_list(vec![1, 3, 4]),
                to_list(vec![2, 6])
            ]),
            to_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
        )
    }
}
