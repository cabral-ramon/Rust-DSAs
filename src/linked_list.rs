#[derive(Debug)]
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            next: None,
        }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
        }
    }

    pub fn push(&mut self, value: T) {
        let mut node = Node::new(value);
        let curr_head = self.head.take();
        node.next = curr_head;
        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut curr_head = self.head.take()?;
        let next = curr_head.next;
        curr_head.next = None;
        self.head = next;
        Some(curr_head.value)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self {
            head: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_golden_path() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push(1);
        ll.push(2);
        ll.push(3);

        assert_eq!(ll.peek(), Some(&3));
        assert_eq!(ll.pop(), Some(3));

        assert_eq!(ll.peek(), Some(&2));
        assert_eq!(ll.pop(), Some(2));

        ll.push(42);

        assert_eq!(ll.peek(), Some(&42));
        assert_eq!(ll.pop(), Some(42));

        assert_eq!(ll.peek(), Some(&1));
        assert_eq!(ll.pop(), Some(1));

        assert_eq!(ll.pop(), None);
        assert_eq!(ll.peek(), None);
    }
}
