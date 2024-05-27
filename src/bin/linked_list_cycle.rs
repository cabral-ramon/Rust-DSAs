extern crate rust_dsa;
use std::{cell::RefCell};
use std::rc::Rc;

use rust_dsa::linked_list::{LinkedList, Node};

fn main() {
   let mut ll: LinkedList<i32> = LinkedList::new();
   let mut node_3: Node<i32> = Node::new(3);
   let node_2: Node<i32> = Node::new(2);
   let node_1: Node<i32> = Node::new(1);
   let node_1 = Rc::new(RefCell::new(node_1));
   // create cycle
   node_3.next = Some(node_1.clone());
   ll.push_node(Rc::new(RefCell::new(node_3)));
   ll.push_node(Rc::new(RefCell::new(node_2)));
   ll.push_node(node_1.clone());

   assert!(detect_linked_list_cycle(ll));
}


fn detect_linked_list_cycle<T>(list: LinkedList<T>) -> bool {
    unimplemented!()
}
