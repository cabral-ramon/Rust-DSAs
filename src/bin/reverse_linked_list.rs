type NodePtr = Box<Node>;

#[derive(Debug)]
struct LinkedList {
   head: Option<NodePtr>,
   len: usize
}

impl LinkedList {
   fn from_vec(v: Vec<u32>) -> Self {
      let len = v.len();
      if len == 0 {
         return Self { head: None, len: 0};
      }

      let first = v[0];
      let mut head = Box::new(Node::new_node(first));

      for n in v.iter().skip(1) {
         let curr = Node::new_node(*n);
         temp.next = Some(curr.clone());
         temp = curr;
      }

      Self {
         head: Some(head),
         len
      }

   }

   fn print(&self) {
      println!("LinkedList size: {}", self.len);
      let mut head = self.head.clone();
      while let Some(node) = head {
         println!("Node: {}", node.data);
         head = node.next.as_ref().cloned();
      }
   }

   fn reverse(&mut self) {
      let mut prev = None;
      let mut curr = self.head.clone();

      while let Some(mut node) = curr.take() {
         let next = node.next.take();
         node.next = prev;
         prev = Some(node);
         curr = next;
      }

      self.head = prev;
   }

}

#[derive(Debug, Clone)]
struct Node {
   data: u32,
   next: Option<NodePtr>,
}

impl Node {
   fn new_node(data: u32) -> NodePtr {
      Box::new(Node{data, next: None })
   }
}

fn main() {
   let nums = vec![1, 2, 3, 4];
   let list = LinkedList::from_vec(nums);
   list.print();
}
