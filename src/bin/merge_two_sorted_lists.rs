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

fn main() {
    // Test your function with different lists here
}

fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {

    // How to work with nodes?
    // 1. we need to have a referecen to a head node, which is what we will return
    let mut head: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 0, next: None }));
    // 2. we need a marker that is the last item in the linked list, that will be our tail
    let mut tail = &mut head;
    // 3. we need to use mutable version of both lists, so we can advance from the list head to the last element
    let mut list1 = list1;
    let mut list2 = list2;
    
    // we will advance throught both nodes while they both have some values, then when we get to the end of their overlap, we check which of the list have more values, and we append the whole list of the one to the end of the newly contructed list
    // Algorithm
    while let (Some(l1), Some(l2)) = (&list1, &list2) {
      // just see which value is smaller and pullt the node and put it on the tail:
      if l1.val <= l2.val {
        // taking the next node of list1, (setting list1.next to None)
        let next = list1.as_mut().unwrap().next.take();
        // setting tail's next item to the current node, after it's next is None
        tail.as_mut().unwrap().next = list1.take();
        // advancing tail to it's next node,
        tail = &mut tail.as_mut().unwrap().next;
        // advancing list1 to what was it's next:
        list1 = next;
      } else {
        // taking list2.next, now list2.next is none.
        let next = list2.as_mut().unwrap().next.take();
        // moving the current node of list one (where next is none) to tail.next, not list2 is None.
        tail.as_mut().unwrap().next = list2.take();
        // advancing the tail node forward
        tail = &mut tail.as_mut().unwrap().next;
        // moving list2 to be next
        list2 = next;
      }
    }
    // See which list is greater and append it to the tail
    if list1.is_some() {
      // take the next of tail, and append it list1
      tail.as_mut().unwrap().next = list1.take();
    } else if list2.is_some() {
      tail.as_mut().unwrap().next = list2.take();
    }
    head.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a list from a vector
    fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut current = None;
        for &val in vec.iter().rev() {
            let mut new_node = ListNode { next: None, val };
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
