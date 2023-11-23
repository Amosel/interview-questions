// Define a basic Binary Tree Node
#[derive(Debug)]
pub struct TreeNode {
    pub value: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

#[allow(dead_code)]
impl TreeNode {
    pub fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

// Define a Binary Tree
#[derive(Debug)]
pub struct BinaryTree {
    pub root: Option<Box<TreeNode>>,
}

#[allow(dead_code)]
impl BinaryTree {

    pub fn new() -> Self {
        BinaryTree { root: None }
    }

    // Insert a value into the binary tree
    // Insertion maintains the sorted order and ensures values to the left are smaller, and values to the right are greater.
    // For any given node in a Binary Tree
    //     All nodes in its left subtree have values less than the node's value.
    //     All nodes in its right subtree have values greater than the node's value.
    //     Comparing the new value with the values of nodes they are traversed,
    //         the value is inserted as the left child if it's less than the current node,
    //         or as the right child if it's greater.
    //
    pub fn insert(&mut self, value: i32) {
        let new_node = Box::new(TreeNode::new(value));
        let mut current = &mut self.root;

        // here, first dereferencing current, borrowing the `Box<TreeNode>` held by it,
        // then inside the while loop (left) we borrow a mutable references to the TreeNode inside that Box.
        while let Some(ref mut node) = *current {
            // a binary tree insert are sorted.
            if value < node.value {
                current = &mut node.left;
            } else {
                current = &mut node.right;
            }
        }

        *current = Some(new_node);
    }

    // Perform an in-order traversal and print the values
    pub fn inorder_traversal(&self, node: &Option<Box<TreeNode>>) {
        if let Some(ref n) = *node {
            self.inorder_traversal(&n.left);
            print!("{} ", n.value);
            self.inorder_traversal(&n.right);
        }
    }
}
