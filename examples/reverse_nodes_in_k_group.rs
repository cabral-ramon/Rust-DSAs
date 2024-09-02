#![allow(dead_code)]
// The task is to reverse the nodes in groups of k in a given linked list, where k is a positive
// integer, and at most the length of the linked list. If any remaining nodes are not part of a group
// of k, they should remain in their original order.
// It is not allowed to change the values of the nodes in the linked list. Only the order of the nodes can be modified.
//
// Use only O(1) extra memory space.

fn main() {}

use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    val: i32,
    next: Link,
}

impl Node {
    fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { val, next: None }))
    }
}

struct LinkedList {
    head: Link,
    // `tail` is not really necessary for this problem but it makes implementing `append` easier.
    // TODO: Maybe we can use a `Weak` Rc here. It may be helpful to protect against reference cycles.
    // Though if it increases the complexity of solving this algorithm probably be best to not do that.
    tail: Link,
    // Not needed for algorithm but can be helpful for testing.
    size: usize,
}

impl LinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn append(&mut self, val: i32) {
        let node = Node::new(val);
        match self.tail.take() {
            None => {
                self.head = Some(node.clone());
            }
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(node.clone());
            }
        }
        self.tail = Some(node);
        self.size += 1;
    }

    fn append_withouth_tail(&mut self, val: i32) {
        let node = Node::new(val);
        let mut curr = self.head.clone();

        loop {
            match curr {
                Some(curr_node) if curr_node.borrow().next.is_none() => {
                    curr_node.borrow_mut().next = Some(node.clone());
                    break;
                }
                Some(curr_node) => {
                    curr = curr_node.borrow_mut().next.clone();
                }
                None => {
                    self.head = Some(node.clone());
                    break;
                }
            }
        }
        self.size += 1;
    }

    fn as_list(&self) -> Vec<i32> {
        // Good helper to check sanity / correctness in testing
        // quicker than implementing Iterator or IntoIterator
        let mut curr = self.head.clone();
        let mut vec = Vec::new();

        while let Some(node) = curr {
            vec.push(node.borrow().val);
            curr = node.borrow().next.clone();
        }

        vec
    }

    fn reverse_nodes(&mut self) {
        // Reverse linked list algo for reference
        // TODO: see if there is a way to call into this helper
        let mut prev: Link = None;
        let mut curr = self.head.take();

        while let Some(node) = curr {
            let next = node.borrow_mut().next.clone();
            node.borrow_mut().next = prev.clone();
            prev = Some(node.clone());
            curr = next;
        }
        self.head = prev.clone();
    }

    fn reverse_nodes_in_k_group(&mut self, group_size: usize) {
        if group_size <= 1 {
            return;
        }
        // placeholder node makes keeping track of head and reversing the first group easier
        let dummy = Node::new(0);
        dummy.borrow_mut().next = self.head.clone();

        // keep track of the nodes that come before and after the Kth group
        // we need this because after we've reversed the nodes in the group
        // we'll need to place them back correctly onto the list
        // For example imagine this list: ...before -> 2 -> 3 -> ...after
        // Where K = 2 and [2, 3] form a Kth group
        // After we reverse this group we'll get  3 -> 2
        // We'll need to update ...before to point to 3
        // And 2 to point to ...after
        let mut group_prev = Some(dummy.clone());

        // iterate through the list
        'outer: while let Some(group_node_prev) = group_prev {
            // find the Kth node
            let mut kth = Some(group_node_prev.clone());
            for _ in 0..group_size {
                if let Some(node) = kth {
                    kth = node.borrow_mut().next.clone();
                } else {
                    // we've run out of nodes before reaching the next Kth
                    break 'outer;
                }
            }

            // reverse the group -- from group_prev.next up to K
            let mut curr = group_node_prev.borrow_mut().next.clone();
            let mut prev: Link = None;

            for _ in 0..group_size {
                if let Some(node) = curr {
                    let next: Link = node.borrow_mut().next.clone();
                    node.borrow_mut().next = prev.clone();
                    prev = Some(node.clone());
                    curr = next;
                }
            }
            // curr now points to the first node in the next group
            // prev now points to the last node in the current group

            // store a reference to the first node in the group (now the last in the group after being reversed)
            let last_node_of_reversed_group = group_node_prev.borrow_mut().next.clone();
            // set the `next` pointer of this node to the first of the next group
            last_node_of_reversed_group
                .clone()
                .unwrap()
                .borrow_mut()
                .next = curr.clone();
            // set group_prev.next to the last node of the current group
            group_node_prev.borrow_mut().next = prev.clone();
            // set group_prev to the last node in the group??
            group_prev = last_node_of_reversed_group.clone();

        }
        self.head = dummy.borrow_mut().next.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list_can_push_values() {
        let mut ll = LinkedList::new();
        ll.append(1);
        ll.append(2);
        ll.append(3);

        assert_eq!(ll.size, 3);
        assert_eq!(ll.as_list(), [1, 2, 3]);
    }

    #[test]
    fn test_linked_list_can_push_values_without_using_tail() {
        let mut ll = LinkedList::new();
        ll.append_withouth_tail(1);
        ll.append_withouth_tail(2);
        ll.append_withouth_tail(3);

        assert_eq!(ll.size, 3);
        assert_eq!(ll.as_list(), [1, 2, 3]);
    }

    #[test]
    fn test_linked_list_reverse_nodes() {
        let mut ll = LinkedList::new();
        ll.append(1);
        ll.append(2);
        ll.append(3);

        ll.reverse_nodes();
        assert_eq!(ll.as_list(), [3, 2, 1]);
    }

    #[test]
    fn test_linked_list_reverse_nodes_in_k_group() {
        let mut ll = LinkedList::new();
        ll.append(1);
        ll.append(2);
        ll.append(3);
        ll.append(4);
        ll.append(5);

        ll.reverse_nodes_in_k_group(2);
        assert_eq!(ll.as_list(), [2, 1, 4, 3, 5]);
    }

    #[test]
    fn test_linked_list_reverse_nodes_in_k_group_k_is_one() {
        let mut ll = LinkedList::new();
        ll.append(1);
        ll.append(2);
        ll.append(3);
        ll.append(4);
        ll.append(5);

        ll.reverse_nodes_in_k_group(1);
        assert_eq!(ll.as_list(), [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_linked_list_reverse_nodes_in_k_group_k_is_size_of_list() {
        let mut ll = LinkedList::new();
        ll.append(1);
        ll.append(2);
        ll.append(3);
        ll.append(4);
        ll.append(5);

        ll.reverse_nodes_in_k_group(5);
        assert_eq!(ll.as_list(), [5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_linked_list_reverse_nodes_in_k_group_k_is_bigger_than_list() {
        let mut ll = LinkedList::new();
        ll.append(1);
        ll.append(2);
        ll.append(3);
        ll.append(4);
        ll.append(5);

        ll.reverse_nodes_in_k_group(6);
        assert_eq!(ll.as_list(), [5, 4, 3, 2, 1]);
    }
}
