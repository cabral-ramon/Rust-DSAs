#![allow(dead_code)]

type Memory = Vec<Node>;

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<usize>,
    prev: Option<usize>,
}

impl Node {
    #[allow(clippy::new_ret_no_self)]
    fn new(val: i32, memory: &mut Memory) -> usize {
        let node = Self {
            val,
            next: None,
            prev: None,
        };
        memory.push(node);
        memory.len() - 1
    }

    fn from_list(list: &[i32], memory: &mut Memory) -> Option<usize> {
        if list.is_empty() {
            return None;
        }

        let mut items = list.iter();
        let first_val = items.next().unwrap();
        let root_idx = Self::new(*first_val, memory);
        let mut prev: usize = root_idx;

        for n in items {
            let new_node_idx = Node::new(*n, memory);
            memory[prev].next = Some(new_node_idx);
            memory[new_node_idx].prev = Some(prev);
            prev = new_node_idx;
        }

        Some(root_idx)
    }

    fn print(root: usize, memory: &Memory) {
        let mut curr = Some(root);

        while let Some(node_ptr) = curr {
            println!("{:?}", memory[node_ptr].val);
            curr = memory[node_ptr].next;
        }
    }
}

fn main() {
    let mut memory = Memory::new();
    let vals = &[1, 2, 3, 4];
    let root_idx = Node::from_list(vals, &mut memory).unwrap();

    Node::print(root_idx, &memory);
    let root = reverse_list(root_idx, &mut memory);
    Node::print(root, &memory);
}

fn reverse_list(root: usize, memory: &mut Memory) -> usize {
    let mut curr = Some(root);
    let mut prev = None;

    while let Some(curr_idx) = curr {
        let next_node = memory[curr_idx].next;

        memory[curr_idx].next = prev;
        prev = Some(curr_idx);
        curr = next_node;
    }

    prev.unwrap()
}

#[test]
fn test_reverse_list() {
    let vals = &[1, 2, 3, 4];
    let mut memory = Memory::new();
    let root_idx = Node::from_list(vals, &mut memory).unwrap();
    Node::print(root_idx, &memory);
    println!("{:?}", memory);
}

fn reverse_list_at_idx(root: usize, memory: &mut Memory, left: usize, _right: usize) -> usize {
    let mut curr_idx = root;
    for _ in 0..left {
        let maybe_next = memory[curr_idx].next;
        match maybe_next {
            Some(idx) => curr_idx = idx,
            None => panic!("Left is bigger than the number of nodes allocated for this list")
        }
    }

    1
}

