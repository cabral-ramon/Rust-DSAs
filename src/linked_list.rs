use std::cell::RefCell;
use std::rc::Rc;

type NodePtr<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<NodePtr<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Option<NodePtr<T>>,
}

impl<T: std::fmt::Debug> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, value: T) {
        let mut node = Node::new(value);
        let curr_head = self.head.take();
        node.next = curr_head;
        self.head = Some(Rc::new(RefCell::new(node)));
    }

    pub fn push_node(&mut self, node_ptr: Rc<RefCell<Node<T>>>) {
        let curr_head = self.head.take();
        node_ptr.borrow_mut().next = curr_head;
        self.head = Some(node_ptr);
    }

    pub fn pop(&mut self) -> Option<T> {
        let curr_head = self.head.take()?;
        let next = curr_head.borrow_mut().next.take();
        // I don't think we need to do this since it gets dropped at the end of this scope
        self.head = next.clone();
        Some(
            Rc::try_unwrap(curr_head)
                .expect("There should have only been one pointer to this Node!")
                .into_inner()
                .value,
        )
    }

    pub fn peek(&self) -> Option<NodePtr<T>> {
        self.head.as_ref().cloned()
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self { head: None }
    }
}
