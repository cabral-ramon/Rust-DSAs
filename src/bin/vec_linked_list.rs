#[derive(Debug)]
struct Node {
   data: u64,
   next: Option<usize>,
   prev: Option<usize>,
}

#[derive(Debug)]
struct LinkedList {
   list: Vec<Node>,
   size: usize,
   head: Option<usize>,
   tail: Option<usize>,
}

impl Node {
   fn new(data: u64) -> Self {
      Self {
         data,
         next: None,
         prev: None,
      }
   }
}

impl LinkedList {
   fn new() -> Self {
      Self { list: vec!(), size: 0, head: None, tail: None }
   }

   fn append(&mut self, data: u64) {
      let mut node = Node::new(data);
      if let Some(idx) = self.tail {
         let new_idx = self.list.len();
         self.list[idx].next = Some(new_idx);
         node.prev = Some(idx);
         self.tail = Some(new_idx);
      } else {
         self.head = Some(0);
         self.tail = Some(0);
      }
      self.list.push(node);
      self.size += 1;
   }

   fn to_list(&self) -> Vec<u64> {
      let mut curr = self.head;
      let mut list = vec!();
      while let Some(idx) = curr {
         let node = &self.list[idx];
         list.push(node.data);
         curr = node.next;
      }
      list
   }
}

fn main() {
   let mut ll = LinkedList::new();
   println!("{:?}", ll.list);
   ll.append(1);
   println!("{:?}", ll.list);
   ll.append(2);
   println!("{:?}", ll.list);
   ll.append(3);
   println!("{:?}", ll.list);
}
