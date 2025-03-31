/*Given the root of a binary tree, return the length of the diameter of the tree.
* The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.
* The length of a path between two nodes is represented by the number of edges between them.
*/
#![allow(unused)]

fn main() {}

#[derive(Debug)]
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Box<Self> {
        Box::new(Self {
            val,
            left: None,
            right: None,
        })
    }
}

#[derive(Debug)]
struct BinaryTree {
    root: Box<Node>,
}

use std::collections::VecDeque;

impl BinaryTree {
    fn new(root: Box<Node>) -> Self {
        Self { root }
    }

    fn from_list(list: Vec<Option<i32>>) -> Option<Self> {
        let mut iter = list.into_iter();
        let first_val = iter.next()??;
        let mut root = Node::new(first_val);
        if let Some(maybe_left_val @ Some(val)) = iter.next() {
            let left = Node::new(val);
            root.left = Some(left);
        }
        if let Some(maybe_right_val @ Some(val)) = iter.next() {
            let right = Node::new(val);
            root.right = Some(right);
        }

        while let Some(maybe_val) = iter.next() {
            if let Some(val) = maybe_val {
                let mut this_node = Node::new(val);
                if let Some(maybe_left_val @ Some(val)) = iter.next() {
                    let left = Node::new(val);
                    this_node.left = Some(left);
                }
                if let Some(maybe_right_val @ Some(val)) = iter.next() {
                    let right = Node::new(val);
                    this_node.right = Some(right);
                }
            }
        }

        Some(Self::new(root))
    }

    fn as_list(&self) -> Vec<Option<i32>> {
        let mut list = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(&self.root);
        list.push(Some(self.root.val));
        if let Some(left) = &self.root.left {
            queue.push_back(left);
        } else {
            list.push(None);
        }
        if let Some(right) = &self.root.right {
            queue.push_back(right);
        } else {
            list.push(None);
        }

        while let Some(node) = queue.pop_front() {
            list.push(Some(node.val));
            if let Some(left) = &node.left {
                queue.push_back(left);
            } else {
                list.push(None);
            }
            if let Some(right) = &node.right {
                queue.push_back(right);
            } else {
                list.push(None);
            }
        }

        list
    }

    fn get_diameter(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create_from_list() {
        let tree = BinaryTree::from_list(vec![Some(1), Some(2), Some(3)]).unwrap();

        assert_eq!(tree.as_list(), vec![Some(1), Some(2), Some(3)]);
    }
}
