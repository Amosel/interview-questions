// taken from: https://leetcode.com/problems/merge-k-sorted-lists/
// Definition for singly-linked list.

use std::{cmp::Ordering, collections::BinaryHeap};
// not importing leetcode here so avoid Rusts No Orphan Rule and impl Ord and PartialEq

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

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering to make it a min-heap
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut heap: BinaryHeap<Box<ListNode>> = BinaryHeap::new();

    // Initialize the heap with the head of each list
    for list in lists {
        if let Some(node) = list {
            heap.push(node);
        }
    }

    // Sentinel head to simplify list construction
    let mut head = Box::new(ListNode::new(0));
    let mut tail = &mut head;

    // Continuously pop the smallest element and add it to the result list
    while let Some(mut node) = heap.pop() {
        if let Some(next) = node.next.take() {
            heap.push(next);
        }
        tail.next = Some(node);
        tail = tail.next.as_mut().unwrap();
    }

    head.next
}

fn main() {}

#[allow(dead_code)]
fn to_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
    // Sentinel head to simplify list construction
    let mut head = Box::new(ListNode::new(0));
    let mut tail = &mut head;
    for val in arr {
        tail.next = Some(Box::new(ListNode::new(val)));
        tail = tail.next.as_mut().unwrap();
    }
    head.next
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
            merge_k_lists(vec![
                to_list(vec![1, 4, 5]),
                to_list(vec![1, 3, 4]),
                to_list(vec![2, 6])
            ]),
            to_list(vec![1, 1, 2, 3, 4, 4, 5, 6])
        )
    }
}
